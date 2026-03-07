pub enum ExitStatus {
    Success,
    Failure(i32),
    Unknown,
}

/// Information about the current shell state.
pub struct Context {
    /// The current working directory, or None if it cannot be determined.
    pub cwd: Option<String>,
}

impl Context {
    pub fn new() -> Self {
        let cwd = std::env::current_dir()
            .ok()
            .and_then(|path| path.to_str().map(|s| s.to_string()));
        Self { cwd }
    }

    /// Retrieves the exit code of the last executed command from the environment variable `EXIT_STATUS`.
    pub fn exit_code(&self) -> ExitStatus {
        env("EXIT_STATUS")
            .and_then(|val| val.parse::<i32>().ok())
            .map_or(ExitStatus::Unknown, |code| match code {
                0 => ExitStatus::Success,
                _ => ExitStatus::Failure(code),
            })
    }

    /// Retrieves the current VI mode (e.g. "i" for insert mode).
    pub fn vi_mode(&self) -> Option<String> {
        env("VI_MODE")
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
