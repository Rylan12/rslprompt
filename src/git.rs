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
    head: Option<String>,
    num_stashes: usize,
    operations: Vec<GitOperations>,
}

impl GitInfo {
    pub fn new(cwd: &Path) -> Self {
        let Some(root) = find_git_root(cwd) else {
            return Self::empty();
        };

        Self {
            is_git_repo: true,
            head: get_git_head(root.as_ref()),
            num_stashes: get_num_stashes(root.as_ref()),
            operations: get_git_operations(root.as_ref()),
        }
    }

    pub fn empty() -> Self {
        Self {
            is_git_repo: false,
            head: None,
            num_stashes: 0,
            operations: Vec::new(),
        }
    }

    /// Whether the current directory is part of a Git repository
    pub fn is_git_repo(&self) -> bool {
        self.is_git_repo
    }

    /// The HEAD ref of the current Git repository
    pub fn head(&self) -> Option<&str> {
        self.head.as_deref()
    }

    /// The number of stashes in the current Git repository
    pub fn num_stashes(&self) -> usize {
        self.num_stashes
    }

    /// The current Git operations in progress (e.g. merge, rebase, etc.)
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

fn get_git_head(root: &Path) -> Option<String> {
    let head_path = root.join(".git").join("HEAD");
    std::fs::read_to_string(head_path)
        .ok()
        .map(|s| s.trim().to_string())
        .and_then(parse_git_head)
}

fn parse_git_head(head_content: String) -> Option<String> {
    head_content
        // Branches are in the format "ref: refs/heads/branch-name"
        .strip_prefix("ref: refs/heads/")
        .map(|s| s.to_string())
        // Otherwise, it's a detached HEAD with the SHA directly in the file
        .or_else(|| head_content.get(..7).map(|s| s.to_string()))
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
