use std::path::{Path, PathBuf};

use crate::git::GitInfo;

/// Information about the current shell state.
pub struct Context {
    /// Information about the current directory's Git repository.
    pub git: GitInfo,
    cwd: Option<PathBuf>,
    home_dir: Option<PathBuf>,
    ssh_connection: bool,
    exit_status: Option<u8>,
    vi_mode: Option<String>,
}

impl Context {
    pub fn new() -> Self {
        let cwd = std::env::current_dir().ok();

        let git = if let Some(cwd) = &cwd {
            GitInfo::new(cwd)
        } else {
            GitInfo::empty()
        };

        Self {
            git,
            cwd,
            home_dir: env("HOME").map(PathBuf::from),
            ssh_connection: env("SSH_CONNECTION").is_some(),
            exit_status: env("EXIT_STATUS").and_then(|val| val.parse::<u8>().ok()),
            vi_mode: env("VI_MODE"),
        }
    }

    /// The current working directory.
    pub fn cwd(&self) -> Option<&Path> {
        self.cwd.as_deref()
    }

    /// The user's home directory.
    pub fn home_dir(&self) -> Option<&Path> {
        self.home_dir.as_deref()
    }

    /// Whether the current session is an SSH connection.
    pub fn ssh_connection(&self) -> bool {
        self.ssh_connection
    }

    /// The exit code status of the last command executed.
    pub fn exit_status(&self) -> Option<u8> {
        self.exit_status
    }

    /// The current VI mode (e.g. "i" for insert mode).
    pub fn vi_mode(&self) -> Option<&str> {
        self.vi_mode.as_deref()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

fn env(name: &str) -> Option<String> {
    std::env::var(name).ok()
}
