use std::{
    fs::{self, File},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, anyhow};
use directories::ProjectDirs;
use tracker_core::{parser::SiegeSettings, storage::HistoryAdapter};

pub struct LocalJsonStorage {}

impl LocalJsonStorage {
    const JSON_FILE_PATH: &'static str = "config_history";
    const APPLICATION_NAME: &'static str = "siege-config-tracker";

    pub fn new() -> Self {
        return LocalJsonStorage {};
    }

    fn get_save_file_dir(&self) -> Option<PathBuf> {
        let proj_dirs = ProjectDirs::from("", "", LocalJsonStorage::APPLICATION_NAME)?;
        let data_dir = proj_dirs.data_dir().join(LocalJsonStorage::JSON_FILE_PATH);

        fs::create_dir_all(&data_dir).ok()?;

        Some(data_dir)
    }

    fn get_save_file_path(&self, filename: PathBuf) -> Option<PathBuf> {
        self.get_save_file_dir().map(|path| path.join(filename))
    }

    fn get_save_file_for_current_ts(&self) -> Option<PathBuf> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()
            .map(|d| d.as_secs())
            .map(|ts| format!("{}.json", ts))
            .map(PathBuf::from)
    }
}

impl HistoryAdapter for LocalJsonStorage {
    fn save_snapshot(&self, settings: &SiegeSettings) -> Result<String> {
        let filename = self
            .get_save_file_for_current_ts()
            .context("Failed to generate snapshot filename")?;
        let id = filename
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();
        let path = self
            .get_save_file_path(filename)
            .context("Failed to resolve snapshot path")?;

        let file = File::create(&path)
            .map_err(|err| anyhow!("Failed to create file at {} - '{}'", path.display(), err))?;
        serde_json::to_writer_pretty(file, settings).context("Failed to write config")?;

        if let Some(path_str) = path.to_str() {
            println!("Created snapshot {}", path_str);
        } else {
            println!("failed to get path str ");
        }

        Ok(id)
    }

    fn get_snapshot(&self, id: &str) -> Result<SiegeSettings> {
        let path = self
            .get_save_file_path(PathBuf::from(format!("{}.json", id)))
            .context("Failed to get savefile path")?;

        let bytes =
            fs::read(&path).with_context(|| format!("Failed to read savefile at {:?}", path))?;

        serde_json::from_slice(&bytes)
            .context("Failed to parse savefile JSON matching SiegeSettings schema")
    }

    fn list_snapshots(&self) -> Result<Vec<SiegeSettings>> {
        let dir = self
            .get_save_file_dir()
            .context("Failed to resolve snapshot directory")?;
        fs::create_dir_all(&dir).context("Failed to create snapshot directory")?;
        let mut results = Vec::new();
        for entry in fs::read_dir(&dir).context("Failed to read snapshot directory")? {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                let bytes =
                    fs::read(&path).with_context(|| format!("Failed to read {:?}", path))?;
                let settings: SiegeSettings = serde_json::from_slice(&bytes)
                    .with_context(|| format!("Failed to parse {:?}", path))?;
                results.push(settings);
            }
        }
        Ok(results)
    }

    fn get_previous_snapshot(&self) -> Option<SiegeSettings> {
        self.list_snapshots().ok()?.pop()
    }
}
