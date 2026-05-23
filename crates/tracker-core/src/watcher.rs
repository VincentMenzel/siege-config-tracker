use anyhow::{Context, Result};
use log::{error, info};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::thread;

use crate::directory::GAME_SETTINGS_FILE;

pub struct WatcherHandle {
    _stop_tx: crossbeam_channel::Sender<()>,
}


pub fn watch_directory<F, T>(dir_path: &Path, on_modified: F) -> Result<WatcherHandle>
where
    F: Fn(&Path) -> Result<T> + Send + 'static,
{
    let (tx, rx) = crossbeam_channel::unbounded();
    let (stop_tx, stop_rx) = crossbeam_channel::bounded::<()>(1);

    info!("Start Watcher on {}", dir_path.display());
    let mut watcher = RecommendedWatcher::new(tx, Config::default())
        .context("Failed to get RecommendedWatcher")?;
    watcher
        .watch(dir_path, RecursiveMode::Recursive)
        .context(format!("Failed to start watching {:?}", dir_path))?;

    thread::spawn(move || {
        let _watcher = watcher;
        loop {
            crossbeam_channel::select! {
                recv(rx) -> msg => match msg {
                    Ok(Ok(event)) => {
                        if let EventKind::Modify(_) = event.kind {
                            for path in event.paths {
                                if path.file_name().and_then(|s| s.to_str()) == Some(GAME_SETTINGS_FILE) {
                                    if let Err(err) = on_modified(&path) {
                                        error!("on_modified failed with {}", err);
                                    }
                                } else {
                                    info!("Unrelated file '{}' changed", path.display());
                                }
                            }
                        }
                    }
                    Ok(Err(e)) => error!("Watch error: {:?}", e),
                    Err(_) => break,
                },
                recv(stop_rx) -> _ => break,
            }
        }
    });

    Ok(WatcherHandle { _stop_tx: stop_tx })
}
