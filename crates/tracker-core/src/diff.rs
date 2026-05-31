use std::io::stdout;

use anyhow::{Context, Ok, Result};
use colored::{Colorize, *};
use serde_json::Value;
use similar::{Change, ChangeTag, TextDiff};
use std::io::Write;

use crate::parser::SiegeSettings;

pub fn diff_settings(old: &SiegeSettings, new: &SiegeSettings) -> Result<()> {
    let stdout = stdout();
    let mut lock = stdout.lock();

    print_diff(&mut lock, old, new)?;

    Ok(())
}

fn print_diff<W: Write>(writer: &mut W, old: &SiegeSettings, new: &SiegeSettings) -> Result<()> {
    let old_text = to_sorted_json(old, "Failed json convert for old")?;
    let new_text = to_sorted_json(new, "Failed json convert for new")?;

    writeln!(
        writer,
        "{}",
        "--- Comparing Setting Snapshots ---".bold().yellow()
    )?;

    let diff = TextDiff::from_lines(&old_text, &new_text);

    for change in diff.iter_all_changes() {
        if let Some(change_log) = map_change(&change) {
            writer
                .write_all(change_log.as_bytes())
                .context("Failed to write change_log")?;
        }
    }

    Ok(())
}

fn map_change(change: &Change<&str>) -> Option<ColoredString> {
    match change.tag() {
        ChangeTag::Insert => Some(format!("+ {}", change).green()),
        ChangeTag::Delete => Some(format!("- {}", change).red()),
        ChangeTag::Equal => None,
    }
}

fn to_sorted_json(settings: &SiegeSettings, label: &str) -> Result<String> {
    let mut value = serde_json::to_value(settings).context(label.to_string())?;
    sort_json_keys(&mut value);
    serde_json::to_string_pretty(&value).context(label.to_string())
}

fn sort_json_keys(value: &mut Value) {
    match value {
        Value::Object(map) => {
            map.iter_mut().for_each(|(_, v)| sort_json_keys(v));
            let mut pairs: Vec<(String, Value)> =
                map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            pairs.sort_by(|a, b| a.0.cmp(&b.0));
            *map = pairs.into_iter().collect();
        }
        Value::Array(arr) => arr.iter_mut().for_each(sort_json_keys),
        _ => {}
    }
}

#[cfg(test)]
mod tests {}
