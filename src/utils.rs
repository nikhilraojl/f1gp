use std::path::PathBuf;

use crate::error::Result;

pub fn get_or_create_tmp_dir() -> Result<PathBuf> {
    let tmp_dir = std::env::temp_dir().join("f1_schedule_standings");
    if !tmp_dir.exists() {
        std::fs::create_dir(&tmp_dir)?;
    }
    Ok(tmp_dir)
}
