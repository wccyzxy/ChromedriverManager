# Chromedriver Manager

## Crates.io
https://crates.io/crates/chromedriver-manager

## Example
```rs
use thirtyfour::prelude::*;

// Require the Handler

use chromedriver_manager::{manager::Handler, loglevel::LogLevel};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create Chrome capabilities
    let mut caps = DesiredCapabilities::chrome(); 

    // Launch chromedriver on port 9515 
    let mut chromedriver = Handler::new()
        .launch_chromedriver(&mut caps, "9515", LogLevel::Off)
        .await?;

    // Connect to chrome on the same port
    let driver = WebDriver::new("http://localhost:9515", caps).await?; 

    // Close the proccess after tasks are finished
    chromedriver.kill()?

    Ok(())
}
```

