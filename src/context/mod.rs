mod git;
mod world;

use std::path::{Path, PathBuf};

use nix::unistd::Pid;

use crate::context::{git::GitContext, world::WorldContext};

pub use git::GitStatus;

/// Represents a value that might not be available yet.
pub enum AsyncValue<T> {
    Ready(T),
    Pending,
}

impl<T> From<Option<T>> for AsyncValue<T> {
    fn from(opt: Option<T>) -> Self {
        if let Some(value) = opt {
            AsyncValue::Ready(value)
        } else {
            AsyncValue::Pending
        }
    }
}

/// Information about the current shell state.
pub struct Context {
    git: Option<GitContext>,
    world: Option<WorldContext>,
    cwd: Option<PathBuf>,
    home_dir: Option<PathBuf>,
    ssh_connection: bool,
    exit_status: Option<u8>,
    vi_mode: Option<String>,
}

impl Context {
    pub fn new() -> Self {
        let cwd = std::env::current_dir().ok();
        let home_dir = env("HOME").map(PathBuf::from);

        let world = if let (Some(cwd), Some(home_dir)) = (&cwd, &home_dir) {
            WorldContext::new(cwd, home_dir)
        } else {
            None
        };

        let shell_pid = env("SHELL_PID")
            .and_then(|val| val.parse::<i32>().ok())
            .map(Pid::from_raw)
            .filter(|pid| pid.as_raw() > 0);
        let exec_no = env("PS1_EXEC_NO").and_then(|val| val.parse::<u64>().ok());

        let git = if let Some(cwd) = &cwd {
            GitContext::new(cwd, shell_pid, exec_no)
        } else {
            None
        };

        Self {
            git,
            world,
            cwd,
            home_dir,
            ssh_connection: env("SSH_CONNECTION").is_some(),
            exit_status: env("EXIT_STATUS").and_then(|val| val.parse::<u8>().ok()),
            vi_mode: env("VI_MODE"),
        }
    }

    /// Information about the current directory's Git repository.
    pub fn git(&self) -> Option<&GitContext> {
        self.git.as_ref()
    }

    /// Information about the current directory's World tree (if applicable).
    pub fn world(&self) -> Option<&WorldContext> {
        self.world.as_ref()
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
