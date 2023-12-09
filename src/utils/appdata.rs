use platform_dirs::AppDirs;
use std::{fs, path::PathBuf};

fn get_chached_dir() -> PathBuf {
    let app_dirs = AppDirs::new(Some("chromedriver-manager"), true).unwrap();
    let cache_dir = app_dirs.cache_dir;

    cache_dir
}

pub fn get_cache_dir() -> PathBuf {
    let cache_dir = get_chached_dir();

    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir).unwrap();
    }

    cache_dir
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{:?}", get_cache_dir());
    }
}
