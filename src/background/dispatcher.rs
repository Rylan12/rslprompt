use std::path::Path;
use std::process::exit;

use fork::{Fork, daemon, fork};
use nix::{
    sys::signal::{Signal, kill},
    unistd::Pid,
};

use crate::background::{BackgroundData, cache::CacheKey};

fn signal_shell(pid: Pid) -> Result<(), nix::Error> {
    kill(pid, Signal::SIGALRM)
}

/// Dispatch a background task to fetch data asynchronously, write it to the cache, and signal the shell when done.
pub fn dispatch(cwd: &Path, key: CacheKey) {
    // First fork: parent returns so prompt rendering continues immediately.
    let Ok(Fork::Child) = fork() else {
        return;
    };

    // In the child branch, daemonize to detach from TTY and redirect stdio to /dev/null.
    // nochdir=true keeps cwd intact, noclose=false closes stdio FDs.
    let Ok(Fork::Child) = daemon(true, false) else {
        exit(1);
    };

    let data = BackgroundData::fetch(cwd);
    if data.write(&key).is_err() {
        exit(1);
    }

    match signal_shell(key.pid()) {
        Ok(_) => exit(0),
        Err(e) => exit(e as i32),
    };
}
