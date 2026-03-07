use std::cell::RefCell;

/// Information about the current shell state.
pub struct Context {
    cwd: RefCell<Option<String>>,
    home_dir: RefCell<Option<String>>,
    ssh_connection: RefCell<bool>,
    exit_status: RefCell<Option<u8>>,
    vi_mode: RefCell<Option<String>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            cwd: RefCell::new(None),
            home_dir: RefCell::new(None),
            ssh_connection: RefCell::new(false),
            exit_status: RefCell::new(None),
            vi_mode: RefCell::new(None),
        }
    }

    /// The current working directory
    pub fn cwd(&self) -> Option<String> {
        let mut cwd = self.cwd.borrow_mut();
        if cwd.is_none() {
            *cwd = std::env::current_dir()
                .ok()
                .and_then(|path| path.to_str().map(|s| s.to_string()));
        }
        cwd.clone()
    }

    /// The user's home directory
    pub fn home_dir(&self) -> Option<String> {
        let mut home_dir = self.home_dir.borrow_mut();
        if home_dir.is_none() {
            *home_dir = env("HOME");
        }
        home_dir.clone()
    }

    /// Whether the current session is an SSH connection
    pub fn ssh_connection(&self) -> bool {
        let mut ssh_connection = self.ssh_connection.borrow_mut();
        if !*ssh_connection {
            *ssh_connection = env("SSH_CONNECTION").is_some();
        }
        *ssh_connection
    }

    /// The exit code of the last command executed
    pub fn exit_code(&self) -> Option<u8> {
        let mut exit_status = self.exit_status.borrow_mut();
        if exit_status.is_none() {
            *exit_status = env("EXIT_STATUS").and_then(|val| val.parse::<u8>().ok());
        }
        *exit_status
    }

    /// The current VI mode (e.g. "i" for insert mode).
    pub fn vi_mode(&self) -> Option<String> {
        let mut vi_mode = self.vi_mode.borrow_mut();
        if vi_mode.is_none() {
            *vi_mode = env("VI_MODE");
        }
        vi_mode.clone()
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
