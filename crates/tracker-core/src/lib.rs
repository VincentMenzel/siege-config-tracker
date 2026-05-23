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
    watcher::watch_directory(&r6_dir, move |path| on_modified(path, &history_adapter))?;

    Ok(())
}

fn on_modified<H: HistoryAdapter>(path: &Path, history_adapter: &H) -> Result<()> {
    info!("Ini change deteced: '{}", path.display());

    info!("Parsing new ini file");
    let settings = parse_path(path)?;

    info!("Saving Snapshot");
    history_adapter.save_snapshot(&settings)?;

    info!("Check for div preview");
    if let Some(previous) = history_adapter.get_previous_snapshot() {
        diff::diff_settings(&settings, &previous)?;
    }

    info!("Event Complete");
    Ok(())
}
