use std::path::{Path, PathBuf};

#[derive(Copy, Clone)]
pub enum GitOperations {
    CherryPick,
    Merge,
    Bisect,
    RebaseApply,
    RebaseMerge,
}

impl GitOperations {
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

pub struct GitInfo {
    is_git_repo: bool,
    head_ref: Option<String>,
    head_sha: Option<String>,
    remote_head_sha: Option<String>,
    num_stashes: usize,
    operations: Vec<GitOperations>,
}

impl GitInfo {
    pub fn new(cwd: &Path) -> Self {
        let Some(root) = find_git_root(cwd) else {
            return Self::empty();
        };

        let (head_ref, head_sha, remote_head_sha) = get_git_head_info(root.as_ref());

        Self {
            is_git_repo: true,
            head_ref,
            head_sha,
            remote_head_sha,
            num_stashes: get_num_stashes(root.as_ref()),
            operations: get_git_operations(root.as_ref()),
        }
    }

    pub fn empty() -> Self {
        Self {
            is_git_repo: false,
            head_ref: None,
            head_sha: None,
            remote_head_sha: None,
            num_stashes: 0,
            operations: Vec::new(),
        }
    }

    /// Whether the current directory is part of a Git repository.
    pub fn is_git_repo(&self) -> bool {
        self.is_git_repo
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
