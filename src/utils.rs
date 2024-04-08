use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct PositionInfo {
    pub position: usize,
    pub name: String,
    pub points: usize,
}

pub fn get_or_create_tmp_dir() -> Result<PathBuf> {
    let tmp_dir = std::env::temp_dir().join("f1_schedule_standings");
    if !tmp_dir.exists() {
        std::fs::create_dir(&tmp_dir)?;
    }
    Ok(tmp_dir)
}
