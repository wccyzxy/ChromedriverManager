# Chromedriver Manager

## Crates.io
https://crates.io/crates/chromedriver-manager

```rs
use thirtyfour::prelude::*;

// Require the Handler
use chromedriver_manager::manager::Handler; 

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create Chrome capabilities
    let mut caps = DesiredCapabilities::chrome(); 

    // Launch chromedriver on port 9515 
    Handler::new()
        .launch_chromedriver(&mut caps, "9515") 
        .await?;

    // Connect to chrome on the same port
    let driver = WebDriver::new("http://localhost:9515", caps).await?; 

    Ok(())
}
```
