use crate::utils::appdata::get_cache_dir;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DriverDownload {
    pub platform: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChromeDownload {
    pub platform: String,
    pub url: String,
}

impl DriverDownload {
    fn get_file_name(&self) -> String {
        format!("chromedriver-{}", self.platform)
    }

    pub fn to_zip_path(&self) -> PathBuf {
        let relative = PathBuf::from(self.get_file_name()).with_extension("zip");
        let absolute = get_cache_dir().join(relative);

        absolute
    }

    pub fn to_folder_path(&self) -> PathBuf {
        let relative = PathBuf::from(self.get_file_name());
        let absolute = get_cache_dir().join(relative);

        absolute
    }
}

impl ChromeDownload {
    fn get_file_name(&self) -> String {
        format!("chrome-{}", self.platform)
    }

    pub fn to_zip_path(&self) -> PathBuf {
        let relative = PathBuf::from(self.get_file_name()).with_extension("zip");
        let absolute = get_cache_dir().join(relative);

        absolute
    }

    pub fn to_folder_path(&self) -> PathBuf {
        let relative = PathBuf::from(self.get_file_name());
        let absolute = get_cache_dir().join(relative);

        absolute
    }
}
