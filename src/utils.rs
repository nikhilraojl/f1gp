use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

pub const TMP_DIR_NAME: &str = "f1_schedule_standings";
pub const F1_TABLE_SELECTOR: &str = "table.f1-table > tbody > tr";

#[derive(Debug, Deserialize, Serialize)]
pub struct PositionInfo {
    pub position: usize,
    pub name: String,
    pub points: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QualiPositionInfo {
    pub position: usize,
    pub name: String,
    pub q1: Option<String>,
    pub q2: Option<String>,
    pub q3: Option<String>,
}

pub trait DataFetcher {
    type A;

    fn cache_file_name() -> String;
    fn resource_url() -> String;
    fn process_data(raw_data: String, file_path: &Path) -> Result<Self::A>;

    fn get_or_create_tmp_dir() -> Result<PathBuf> {
        let tmp_dir = std::env::temp_dir().join(TMP_DIR_NAME);
        if !tmp_dir.exists() {
            std::fs::create_dir(&tmp_dir)?;
        }

        Ok(tmp_dir)
    }

    fn fetch_internet_resource() -> Result<String> {
        let body = ureq::get(&Self::resource_url()).call()?.into_string()?;
        Ok(body)
    }

    fn cache_and_return_data(serialized_data: Self::A, file_path: &Path) -> Result<Self::A>
    where
        Self::A: DeserializeOwned,
        Self::A: Serialize,
    {
        let json_data_to_cache = serde_json::to_string(&serialized_data)?;
        fs::write(file_path, json_data_to_cache)?;
        Ok(serialized_data)
    }

    fn read_from_cache(path: &Path) -> Result<Self::A>
    where
        Self: Sized,
        Self::A: DeserializeOwned,
    {
        let data = fs::read_to_string(path)?;
        let mut schedule = serde_json::Deserializer::from_str(&data);
        Ok(Self::A::deserialize(&mut schedule)?)
    }

    fn get_cache_file_path() -> Result<PathBuf> {
        let tmp_dir = Self::get_or_create_tmp_dir()?;
        let file_name = Self::cache_file_name();
        let file_path = tmp_dir.join(file_name);
        Ok(file_path)
    }

    // Setting `force_pull` to true will always make a call to
    // the internet resource.
    // If `force_pull` is false, it may or may not fetch from internet
    // resource depending on the existing of local cache
    // TODO: May be split into read_from_cache & fetch_from_internet
    // functions and avoid this confusion
    fn get_data_internal_with_pull(force_pull: bool) -> Result<Self::A>
    where
        Self: Sized,
        Self::A: DeserializeOwned,
        Self::A: Serialize,
    {
        let file_path = Self::get_cache_file_path()?;
        if !file_path.exists() || force_pull {
            let raw_data = Self::fetch_internet_resource()?;
            let data = Self::process_data(raw_data, &file_path)?;
            Self::cache_and_return_data(data, &file_path)
        } else {
            Self::read_from_cache(&file_path)
        }
    }

    fn get_data() -> Result<Self::A>
    where
        Self: Sized,
        Self::A: DeserializeOwned,
        Self::A: Serialize,
    {
        Self::get_data_internal_with_pull(false)
    }

    fn pull() -> Result<()>
    where
        Self: Sized,
        Self::A: DeserializeOwned,
        Self::A: Serialize,
    {
        let _ = Self::get_data_internal_with_pull(true)?;
        Ok(())
    }
}
