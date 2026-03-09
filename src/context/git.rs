use std::path::{Path, PathBuf};
use std::process::Command;

use nix::unistd::Pid;

use crate::{background::get_background_data, context::AsyncValue};

/// Ongoing Git operations that can be in progress.
#[derive(Copy, Clone)]
pub enum GitOperations {
    CherryPick,
    Merge,
    Bisect,
    RebaseApply,
    RebaseMerge,
}

impl GitOperations {
    /// Get the single-letter symbolic representation of this operation.
    pub fn symbolic_letter(&self) -> &'static str {
        match self {
            GitOperations::CherryPick => "P",
            GitOperations::Merge => "M",
            GitOperations::Bisect => "B",
            GitOperations::RebaseApply => "R",
            GitOperations::RebaseMerge => "r",
        }
    }

    fn filename(&self) -> &'static str {
        match self {
            GitOperations::CherryPick => "CHERRY_PICK_HEAD",
            GitOperations::Merge => "MERGE_HEAD",
            GitOperations::Bisect => "BISECT_LOG",
            GitOperations::RebaseApply => "rebase-apply",
            GitOperations::RebaseMerge => "rebase-merge",
        }
    }
}

/// The Git status of the current repository.
pub enum GitStatus {
    Clean,
    Dirty,
}

impl GitStatus {
    /// Create a GitStatus from a boolean indicating whether there are changes.
    pub fn from_has_changes(has_changes: bool) -> Self {
        if has_changes {
            GitStatus::Dirty
        } else {
            GitStatus::Clean
        }
    }
}

/// Information about the current Git repository.
pub struct GitContext {
    head_ref: Option<String>,
    head_sha: Option<String>,
    remote_head_sha: Option<String>,
    num_stashes: usize,
    operations: Vec<GitOperations>,
    status: AsyncValue<Option<GitStatus>>,
}

impl GitContext {
    /// Create a new GitContext for the given directory.
    pub fn new(cwd: &Path, shell_pid: Option<Pid>, exec_no: Option<u64>) -> Option<Self> {
        let root = find_git_root(cwd)?;
        let git_dir = resolve_git_dir(&root)?;
        let common_dir = resolve_common_dir(&git_dir);
        let (head_ref, head_sha, remote_head_sha) =
            get_git_head_info(&root, &git_dir, &common_dir);

        let background_data = match (shell_pid, exec_no) {
            (Some(pid), Some(exec_no)) => get_background_data(cwd, pid, exec_no),
            _ => None,
        };

        let status = background_data
            .map(|data| data.has_changes.map(GitStatus::from_has_changes))
            .into();

        Some(Self {
            head_ref,
            head_sha,
            remote_head_sha,
            num_stashes: get_num_stashes(&common_dir),
            operations: get_git_operations(&git_dir),
            status,
        })
    }

    /// The HEAD ref of the current Git repository.
    pub fn head_ref(&self) -> Option<&str> {
        self.head_ref.as_deref()
    }

    /// The SHA of the current HEAD commit.
    pub fn head_sha(&self) -> Option<&str> {
        self.head_sha.as_deref()
    }

    /// The SHA of the remote HEAD commit.
    pub fn remote_head_sha(&self) -> Option<&str> {
        self.remote_head_sha.as_deref()
    }

    /// The number of stashes in the current Git repository.
    pub fn num_stashes(&self) -> usize {
        self.num_stashes
    }

    /// The current Git operations in progress (e.g. merge, rebase, etc).
    pub fn operations(&self) -> &[GitOperations] {
        &self.operations
    }

    /// The current Git status.
    pub fn status(&self) -> &AsyncValue<Option<GitStatus>> {
        &self.status
    }
}

fn find_git_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|p| {
            let git_path = p.join(".git");
            git_path.is_dir() || git_path.is_file()
        })
        .map(Path::to_path_buf)
}

/// Resolve the git directory from the repository root.
///
/// For a normal repo, this returns `root/.git`. For a worktree, the `.git`
/// file contains `gitdir: <path>` which points to the actual git metadata
/// directory.
fn resolve_git_dir(root: &Path) -> Option<PathBuf> {
    let git_path = root.join(".git");

    if git_path.is_dir() {
        return Some(git_path);
    }

    let contents = std::fs::read_to_string(&git_path).ok()?;
    let target = contents.trim().strip_prefix("gitdir: ")?;
    let target_path = Path::new(target);

    let resolved = if target_path.is_absolute() {
        target_path.to_path_buf()
    } else {
        root.join(target_path)
    };

    Some(resolved)
}

/// Resolve the common directory for shared git data.
///
/// In a worktree, the git dir contains a `commondir` file that points to the
/// shared git directory (where refs/heads, refs/remotes, objects, etc. live).
/// For a normal repo, the common dir is the git dir itself.
fn resolve_common_dir(git_dir: &Path) -> PathBuf {
    let commondir_path = git_dir.join("commondir");
    match std::fs::read_to_string(commondir_path) {
        Ok(contents) => {
            let relative = contents.trim();
            git_dir.join(relative)
        }
        Err(_) => git_dir.to_path_buf(),
    }
}

fn get_git_head_info(
    root: &Path,
    git_dir: &Path,
    common_dir: &Path,
) -> (Option<String>, Option<String>, Option<String>) {
    let (head_ref, head_sha) = parse_git_head(root, git_dir);

    // If we don't get a ref name, don't even try searching any remotes.
    let Some(head_ref) = head_ref else {
        return (None, head_sha, None);
    };

    let head_ref_path = format!("refs/heads/{}", head_ref);
    let head_sha = get_ref_sha(root, common_dir, &head_ref_path);

    let remote_head_ref_path = format!("refs/remotes/origin/{}", head_ref);
    let remote_head_sha = get_ref_sha(root, common_dir, &remote_head_ref_path);

    (Some(head_ref), head_sha, remote_head_sha)
}

/// Parse the HEAD to get the current branch name or detached SHA.
///
/// Reads the HEAD file directly. Falls back to `git symbolic-ref` or
/// `git rev-parse` only for reftable repos where the HEAD file contains a
/// placeholder ref.
fn parse_git_head(root: &Path, git_dir: &Path) -> (Option<String>, Option<String>) {
    let head_path = git_dir.join("HEAD");
    let Ok(contents) = std::fs::read_to_string(head_path) else {
        return (None, None);
    };

    if let Some(head_ref) = contents.trim().strip_prefix("ref: refs/heads/") {
        if !head_ref.starts_with('.') {
            return (Some(head_ref.to_string()), None);
        }

        // Reftable placeholder (e.g. `.invalid`). Shell out to git.
        if let Some(head_ref) = git_symbolic_ref(root) {
            return (Some(head_ref), None);
        }
        return (None, git_rev_parse(root, "HEAD"));
    }

    // Detached HEAD — the file contains the SHA directly.
    let trimmed = contents.trim().to_string();
    (None, Some(trimmed))
}

/// Get the SHA for a ref by reading files. Tries the loose ref file first,
/// then the `packed-refs` file. Falls back to `git rev-parse` only if
/// neither file contains the ref (e.g. reftable format).
fn get_ref_sha(root: &Path, common_dir: &Path, ref_path: &str) -> Option<String> {
    // Try loose ref file.
    let ref_file = common_dir.join(ref_path);
    if let Ok(contents) = std::fs::read_to_string(ref_file) {
        return Some(contents.trim().to_string());
    }

    // Try packed-refs file.
    if let Some(sha) = lookup_packed_ref(common_dir, ref_path) {
        return Some(sha);
    }

    // Fall back to git (reftable or other exotic formats).
    git_rev_parse(root, ref_path)
}

/// Look up a ref in the `packed-refs` file.
///
/// The packed-refs format has lines like `<sha> <ref>`, with comment lines
/// starting with `#` and peeled entries starting with `^`.
fn lookup_packed_ref(common_dir: &Path, ref_path: &str) -> Option<String> {
    let packed_refs_path = common_dir.join("packed-refs");
    let contents = std::fs::read_to_string(packed_refs_path).ok()?;

    for line in contents.lines() {
        if line.starts_with('#') || line.starts_with('^') {
            continue;
        }
        if let Some((sha, refname)) = line.split_once(' ') {
            if refname == ref_path {
                return Some(sha.to_string());
            }
        }
    }

    None
}

fn git_symbolic_ref(root: &Path) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .arg("symbolic-ref")
        .arg("--short")
        .arg("HEAD")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let name = String::from_utf8(output.stdout).ok()?;
    let name = name.trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

fn git_rev_parse(root: &Path, rev: &str) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .arg("rev-parse")
        .arg(rev)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let sha = String::from_utf8(output.stdout).ok()?;
    let sha = sha.trim();
    if sha.is_empty() {
        None
    } else {
        Some(sha.to_string())
    }
}

fn get_num_stashes(common_dir: &Path) -> usize {
    let stash_path = common_dir.join("logs").join("refs").join("stash");
    std::fs::read_to_string(stash_path)
        .ok()
        .map(|s| s.lines().count())
        .unwrap_or(0)
}

fn get_git_operations(git_dir: &Path) -> Vec<GitOperations> {
    let operations = [
        GitOperations::CherryPick,
        GitOperations::Merge,
        GitOperations::Bisect,
        GitOperations::RebaseApply,
        GitOperations::RebaseMerge,
    ];

    operations
        .iter()
        .filter(|op| {
            let op_path = git_dir.join(op.filename());
            op_path.exists()
        })
        .cloned()
        .collect()
}
