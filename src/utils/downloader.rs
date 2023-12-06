use crate::{
    structs::chrome::{ChromeDownload, DriverDownload},
    utils::functions::write_file,
};

use std::{
    fs::{self, File},
    path::PathBuf,
};
use zip_extensions::zip_extract;

pub async fn download_chromedriver(
    client: &reqwest::Client,
    chromedriver: &DriverDownload,
) -> anyhow::Result<()> {
    let driver_path = chromedriver.to_zip_path();
    let response = client.get(&chromedriver.url).send().await?;

    let file = File::create(&driver_path)?;

    println!("Downloading chromedriver...");
    write_file(&file, response).await?;

    println!("Extracting chromedriver...");
    zip_extract(&driver_path, &PathBuf::from(".")).unwrap();

    println!("Downloaded chromedriver\n");

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

    println!("Downloading chrome...");
    write_file(&file, response).await?;

    println!("Extracting chrome...");
    zip_extract(&chrome_path, &PathBuf::from(".")).unwrap();

    println!("Downloaded chrome\n");

    // Delete zip file
    fs::remove_file(&chrome_path).unwrap();

    Ok(())
}
