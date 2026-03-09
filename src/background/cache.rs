use nix::unistd::Pid;

pub(super) struct CacheKey {
    pid: Pid,
    exec_no: u64,
}

impl CacheKey {
    pub(super) fn new(pid: Pid, exec_no: u64) -> Self {
        Self { pid, exec_no }
    }

    pub(super) fn pid(&self) -> Pid {
        self.pid
    }

    pub(super) fn cache_path(&self) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "rslprompt-async-{}-{}.json",
            self.pid, self.exec_no
        ))
    }
}
