use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::background::cache::CacheKey;

#[derive(Serialize, Deserialize)]
pub(crate) struct BackgroundData {
    pub(crate) has_changes: Option<bool>,
}

impl BackgroundData {
    pub(super) fn fetch(cwd: &Path) -> Self {
        Self {
            has_changes: fetch_git_has_changes(cwd),
        }
    }

    pub(super) fn read(key: &CacheKey) -> Option<Self> {
        let path = key.cache_path();
        let contents = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&contents).ok()
    }

    pub(super) fn write(&self, key: &CacheKey) -> std::io::Result<()> {
        let path = key.cache_path();
        let tmp_path = path.with_extension(format!("tmp-{}", std::process::id()));
        let contents = serde_json::to_string(self)?;
        // Write to a temp file and rename so readers never see partial JSON.
        std::fs::write(&tmp_path, contents)?;
        std::fs::rename(tmp_path, path)
    }
}

fn fetch_git_has_changes(cwd: &Path) -> Option<bool> {
    let output = Command::new("git")
        .arg("-C")
        .arg(cwd)
        .arg("status")
        .arg("--porcelain")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;

    Some(!output.stdout.is_empty())
}
