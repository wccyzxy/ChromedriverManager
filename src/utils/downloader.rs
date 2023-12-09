use crate::{
    structs::chrome::{ChromeDownload, DriverDownload},
    utils::appdata::get_cache_dir,
    utils::functions::write_file,
};

use std::fs::{self, File};
use zip_extensions::zip_extract;

pub async fn download_chromedriver(
    client: &reqwest::Client,
    chromedriver: &DriverDownload,
) -> anyhow::Result<()> {
    let driver_path = chromedriver.to_zip_path();
    let response = client.get(&chromedriver.url).send().await?;

    let file = File::create(&driver_path)?;

    write_file(&file, response, "Downloading Chromedriver...".to_string()).await?;

    println!("Extracting Chromedriver...");
    zip_extract(&driver_path, &get_cache_dir()).unwrap();

    println!("Completed Chromedriver Download");

    // Delete zip file
    fs::remove_file(&driver_path).unwrap();

    Ok(())
}

pub async fn download_chrome(
    client: &reqwest::Client,
    chrome: &ChromeDownload,
) -> anyhow::Result<()> {
    let chrome_path = chrome.to_zip_path();
    let response = client.get(&chrome.url).send().await?;

    let file = File::create(&chrome_path)?;

    write_file(&file, response, "Downloading Chrome".to_string()).await?;

    println!("Extracting Chrome...");
    zip_extract(&chrome_path, &get_cache_dir()).unwrap();

    println!("Completed Chrome Download");

    // Delete zip file
    fs::remove_file(&chrome_path).unwrap();

    Ok(())
}
