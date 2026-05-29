use std::{
    ffi::{OsStr, OsString},
    fs::{self, File},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Ok, Result, anyhow};
use directories::ProjectDirs;
use log::info;
use tracker_core::{parser::SiegeSettings, storage::HistoryAdapter};

#[derive(Default)]
pub struct LocalJsonStorage {}

impl LocalJsonStorage {
    const JSON_FILE_PATH: &'static str = "config_history";
    const APPLICATION_NAME: &'static str = "siege-config-tracker";

    pub fn new() -> Self {
        LocalJsonStorage::default()
    }

    fn get_save_file_dir(&self) -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("", "", LocalJsonStorage::APPLICATION_NAME)
            .context("Failed to resolve addata dir")?;
        let data_dir = proj_dirs.data_dir().join(LocalJsonStorage::JSON_FILE_PATH);

        fs::create_dir_all(&data_dir).context("Failed to create data dir")?;

        Ok(data_dir)
    }

    fn get_save_file_path(&self, filename: PathBuf) -> Result<PathBuf> {
        self.get_save_file_dir().map(|path| path.join(filename))
    }

    fn get_save_file_for_current_ts(&self) -> Option<PathBuf> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()
            .map(|d| d.as_secs())
            .map(|ts| self.get_save_file_for_ts(ts))
    }

    fn get_save_file_for_ts(&self, ts: u64) -> PathBuf {
        PathBuf::from(format!("{}.json", ts))
    }

    fn parse_timestamp_from_filename(&self, filename: OsString) -> Result<u64> {
        Path::new(&filename)
            .file_stem()
            .and_then(OsStr::to_str)
            .context("Failed to get filename stem")?
            .parse::<u64>()
            .context("Failed to parse timestamp from filename")
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

        info!("Created snapshot {}", path.display());

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
        let dir = self.get_save_file_dir()?;

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

    fn get_previous_snapshot(&self) -> Result<SiegeSettings> {
        let snapshot_dir = self.get_save_file_dir()?;

        let mut timestamps: Vec<u64> = fs::read_dir(snapshot_dir)
            .ok()
            .context("Failed to read snapshot dir")?
            .flatten()
            .filter_map(|entry| self.parse_timestamp_from_filename(entry.file_name()).ok())
            .collect();
        timestamps.sort_unstable();

        let latest = timestamps.pop().context("No latest snapshot")?.to_string();
        self.get_snapshot(&latest)
    }
}
