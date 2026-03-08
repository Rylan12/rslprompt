use std::path::{Path, PathBuf};

pub struct GitInfo {
    is_git_repo: bool,
    head: Option<String>,
    num_stashes: usize,
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
        }
    }

    pub fn empty() -> Self {
        Self {
            is_git_repo: false,
            head: None,
            num_stashes: 0,
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
