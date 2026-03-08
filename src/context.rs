/// Information about the current shell state.
pub struct Context {
    cwd: Option<String>,
    home_dir: Option<String>,
    ssh_connection: bool,
    exit_status: Option<u8>,
    vi_mode: Option<String>,
}

impl Context {
    pub fn new() -> Self {
        let cwd = std::env::current_dir()
            .ok()
            .and_then(|p| p.to_str().map(|s| s.to_string()));

        Self {
            cwd,
            home_dir: env("HOME"),
            ssh_connection: env("SSH_CONNECTION").is_some(),
            exit_status: env("EXIT_STATUS").and_then(|val| val.parse::<u8>().ok()),
            vi_mode: env("VI_MODE"),
        }
    }

    /// The current working directory
    pub fn cwd(&self) -> Option<&str> {
        self.cwd.as_deref()
    }

    /// The user's home directory
    pub fn home_dir(&self) -> Option<&str> {
        self.home_dir.as_deref()
    }

    /// Whether the current session is an SSH connection
    pub fn ssh_connection(&self) -> bool {
        self.ssh_connection
    }

    /// The exit code status of the last command executed
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
