use std::path::Path;

use anyhow::{Ok, Result};
use log::info;

use crate::{directory::get_r6_dir, parser::parse_path, storage::HistoryAdapter};

pub(crate) mod diff;
pub(crate) mod directory;
pub mod parser;
pub mod storage;
pub(crate) mod watcher;

pub fn start_watcher(history_adapter: impl HistoryAdapter) -> Result<()> {
    let r6_dir = get_r6_dir()?;
    let _handle =
        watcher::watch_directory(&r6_dir, move |path| on_modified(path, &history_adapter))?;

    info!("Watcher running. Press Ctrl+C to stop.");
    std::thread::park();

    Ok(())
}

fn on_modified<H: HistoryAdapter>(path: &Path, history_adapter: &H) -> Result<()> {
    info!("Start Processing Filechange '{}", path.display());

    info!("Parsing new ini file");
    let current_settings = parse_path(path)?;
    history_adapter.save_snapshot(&current_settings)?;

    if let Some(previous) = history_adapter.get_previous_snapshot().ok() {
        info!("Preview diff");
        diff::diff_settings(&current_settings, &previous)?;
    } else {
        info!("No previous snapshot found");
    }

    info!("End Processing Filechange");
    Ok(())
}
