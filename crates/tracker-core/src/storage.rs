use anyhow::Result;

use crate::parser::SiegeSettings;

pub trait HistoryAdapter: Send + 'static {
    fn save_snapshot(&self, settings: &SiegeSettings) -> Result<String>;
    fn list_snapshots(&self) -> Result<Vec<SiegeSettings>>;
    fn get_snapshot(&self, id: &str) -> Result<SiegeSettings>;
    fn get_previous_snapshot(&self) -> Result<SiegeSettings>;
}
