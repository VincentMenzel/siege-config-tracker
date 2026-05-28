use adapter_local::LocalJsonStorage;
use log::error;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let adapter = LocalJsonStorage::new();
    if let Err(err) = tracker_core::start_watcher(adapter) {
        error!("Failed to create watcher {}", err);
    }
}
