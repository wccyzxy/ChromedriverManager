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
        PathBuf::from(self.get_file_name()).with_extension("zip")
    }

    pub fn to_folder_path(&self) -> PathBuf {
        PathBuf::from(self.get_file_name())
    }
}

impl ChromeDownload {
    fn get_file_name(&self) -> String {
        format!("chrome-{}", self.platform)
    }

    pub fn to_zip_path(&self) -> PathBuf {
        PathBuf::from(self.get_file_name()).with_extension("zip")
    }

    pub fn to_folder_path(&self) -> PathBuf {
        PathBuf::from(self.get_file_name())
    }
}
