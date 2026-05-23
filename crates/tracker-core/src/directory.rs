use std::path::PathBuf;

use anyhow::{Context, Ok, Result, anyhow};
use directories::UserDirs;

pub(crate) const MY_GAMES_DIR: &'static str = "My Games";
pub(crate) const SIEGE_DIR: &'static str = "Rainbow Six - Siege";
pub(crate) const GAME_SETTINGS_FILE: &'static str = "GameSettings.ini";

pub fn get_r6_dir() -> Result<PathBuf> {
    let user_dirs = UserDirs::new().context("Failed to get user_dirs")?;
    let doc_dir = user_dirs
        .document_dir()
        .context("Failed to get user document dir")?;

    let mut r6_dir = PathBuf::from(doc_dir);
    r6_dir.push(MY_GAMES_DIR);
    r6_dir.push(SIEGE_DIR);

    if !r6_dir.exists() {
        return Err(anyhow!("siege dir does not exist"));
    }

    Ok(r6_dir)
}
