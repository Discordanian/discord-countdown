//! Date file loading — read YYYYMMDD.txt files from a directory.

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Load date files from the given directory.
/// Each filename's first 8 characters (YYYYMMDD) become the key; file contents become the value.
pub fn load_dates(dir: &str) -> Result<HashMap<String, String>> {
    let path = Path::new(dir);
    let entries = fs::read_dir(path).with_context(|| format!("Could not list directory [{}]", dir))?;

    let mut dates = HashMap::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            if file_name.len() >= 8 {
                let key = file_name[..8].to_string();
                let contents = fs::read_to_string(&path)
                    .with_context(|| format!("Error reading [{}]", path.display()))?;
                dates.insert(key, contents);
            }
        }
    }
    Ok(dates)
}
