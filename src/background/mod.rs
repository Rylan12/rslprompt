use std::path::Path;

mod cache;
mod data;
mod dispatcher;

pub use data::BackgroundData;
use nix::unistd::Pid;

pub fn get_background_data(cwd: &Path, shell_pid: Pid, exec_no: u64) -> Option<BackgroundData> {
    let key = cache::CacheKey::new(shell_pid, exec_no);
    BackgroundData::read(&key).or_else(|| {
        dispatcher::dispatch(cwd, key);
        None
    })
}
