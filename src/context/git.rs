use std::path::{Path, PathBuf};

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
        let (head_ref, head_sha, remote_head_sha) = get_git_head_info(root.as_ref());

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
            num_stashes: get_num_stashes(root.as_ref()),
            operations: get_git_operations(root.as_ref()),
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
        .find(|p| p.join(".git").is_dir())
        .map(Path::to_path_buf)
}

fn get_git_head_info(root: &Path) -> (Option<String>, Option<String>, Option<String>) {
    let (head_ref, head_sha) = parse_git_head_file(root);

    // If we don't get a ref name, don't even try searching any remotes.
    let Some(head_ref) = head_ref else {
        return (None, head_sha, None);
    };

    let head_ref_path = format!("refs/heads/{}", head_ref);
    let head_sha = get_ref_sha(root, &head_ref_path);

    let remote_head_ref_path = format!("refs/remotes/origin/{}", head_ref);
    let remote_head_sha = get_ref_sha(root, &remote_head_ref_path);

    (Some(head_ref), head_sha, remote_head_sha)
}

fn parse_git_head_file(root: &Path) -> (Option<String>, Option<String>) {
    let head_path = root.join(".git").join("HEAD");
    let Ok(contents) = std::fs::read_to_string(head_path) else {
        return (None, None);
    };

    if let Some(head_ref) = contents.trim().strip_prefix("ref: refs/heads/") {
        return (Some(head_ref.to_string()), None);
    }

    // For detached HEAD, the file contains the SHA directly.
    (None, Some(contents.trim().to_string()))
}

fn get_ref_sha(root: &Path, ref_path: &str) -> Option<String> {
    let ref_file = root.join(".git").join(ref_path);
    std::fs::read_to_string(ref_file)
        .ok()
        .map(|s| s.trim().to_string())
}

fn get_num_stashes(root: &Path) -> usize {
    let stash_path = root.join(".git").join("logs").join("refs").join("stash");
    std::fs::read_to_string(stash_path)
        .ok()
        .map(|s| s.lines().count())
        .unwrap_or(0)
}

fn get_git_operations(root: &Path) -> Vec<GitOperations> {
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
            let op_path = root.join(".git").join(op.filename());
            op_path.exists()
        })
        .cloned()
        .collect()
}
