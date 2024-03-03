use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

use anyhow::Result;

pub fn should_update_from_time(in_time: &SystemTime, out_file: &PathBuf) -> bool {
    match fs::metadata(&out_file) {
        Ok(metadata) => {
            match metadata.modified() {
                Ok(modified) => &modified < in_time,
                _ => true
            }
        },
        _ => true
    }
}

pub fn should_update(in_file: &PathBuf, out_file: &PathBuf) -> Result<bool> {
    Ok(should_update_from_time(&fs::metadata(&in_file)?.modified()?, out_file))
}
