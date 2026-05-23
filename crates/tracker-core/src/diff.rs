use std::io::stdout;

use anyhow::{Context, Ok, Result};
use colored::*;
use similar::{ChangeTag, TextDiff};
use std::io::Write;

use crate::parser::SiegeSettings;

pub fn diff_settings(old: &SiegeSettings, new: &SiegeSettings) -> Result<()> {
    let stdout = stdout();
    let mut lock = stdout.lock();

    print_diff(&mut lock, old, new)?;

    Ok(())
}

fn print_diff<W: Write>(writer: &mut W, old: &SiegeSettings, new: &SiegeSettings) -> Result<()> {
    let old_text = serde_json::to_string_pretty(old).context("Failed json convert for old")?;
    let new_text = serde_json::to_string_pretty(new).context("Failed json convert for new")?;

    writeln!(
        writer,
        "{}",
        "--- Comparing Setting Snapshots ---".bold().yellow()
    )?;

    let diff = TextDiff::from_lines(&old_text, &new_text);

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => {
                writeln!(writer, "{}", format!("- {}", change).red())?;
            }
            ChangeTag::Insert => {
                writeln!(writer, "{}", format!("+ {}", change).green())?;
            }
            ChangeTag::Equal => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {}
