use std::path::Path;

/// Information about a World monorepo worktree.
pub struct WorldContext {
    tree: String,
    path: String,
}

impl WorldContext {
    /// Create a new WorldContext if the current directory is within a World tree.
    pub fn new(cwd: &Path, home_dir: &Path) -> Option<Self> {
        let world_trees_path = home_dir.join("world").join("trees");
        let relative = cwd.strip_prefix(world_trees_path).ok()?;
        let mut components = relative.components();
        let tree = components.next()?.as_os_str().to_string_lossy().to_string();
        if components.next()?.as_os_str() != "src" {
            return None;
        }

        // Path is just everything after src with // at the beginning (e.g. ~/world/trees/root/src/foo/bar) becomes //foo/bar
        let path = format!("//{}", components.as_path().display());

        Some(Self { tree, path })
    }

    /// The name of the current World tree.
    pub fn tree(&self) -> &str {
        &self.tree
    }

    /// The path within the World tree (e.g., "//foo/bar").
    pub fn path(&self) -> &str {
        &self.path
    }
}
