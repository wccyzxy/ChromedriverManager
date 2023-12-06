use super::chrome::{ChromeDownload, DriverDownload};
use crate::utils::version::Version;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Downloads {
    chrome: Vec<ChromeDownload>,
    chromedriver: Option<Vec<DriverDownload>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChromePackage {
    pub version: Version,
    revision: String,
    downloads: Downloads,
}

impl ChromePackage {
    pub fn get_chrome_download(&self, platform: &str) -> Option<&ChromeDownload> {
        self.downloads
            .chrome
            .iter()
            .find(|download| download.platform == platform)
    }

    pub fn get_chromedriver_download(&self, platform: &str) -> Option<&DriverDownload> {
        self.downloads
            .chromedriver
            .as_ref()
            .unwrap()
            .iter()
            .find(|download| download.platform == platform)
    }
}

/* JSON Example:
{
    "version": "115.0.5763.0",
    "revision": "1141961",
    "downloads": {
        "chrome": [
            {
                "platform": "linux64",
                "url": "https://.../chrome-linux64.zip"
            },
            ...
        ],
        "chromedriver": [ // Some versions don't have this field
            {
            "platform": "linux64",
            "url": "https://.../chromedriver-linux64.zip"
            },
            ...
        ]
    }
},
*/
