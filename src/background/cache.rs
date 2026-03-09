use nix::unistd::Pid;

pub struct CacheKey {
    pid: Pid,
    exec_no: u64,
}

impl CacheKey {
    pub fn new(pid: Pid, exec_no: u64) -> Self {
        Self { pid, exec_no }
    }

    pub fn pid(&self) -> Pid {
        self.pid
    }

    pub fn cache_path(&self) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "rslprompt-async-{}-{}.json",
            self.pid, self.exec_no
        ))
    }
}
