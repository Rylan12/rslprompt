use std::path::Path;

pub struct GitInfo {
    head: Option<String>,
}

impl GitInfo {
    pub fn new(cwd: &Path) -> Self {
        let root = cwd
            .ancestors()
            .find(|p| p.join(".git").is_dir())
            .map(Path::to_path_buf);

        let head = root.as_ref().and_then(|r| get_git_head(r));

        Self { head }
    }

    pub fn empty() -> Self {
        Self { head: None }
    }

    /// The HEAD ref of the current Git repository
    pub fn head(&self) -> Option<&str> {
        self.head.as_deref()
    }
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
