use std::env;

use ai_browser::chrome_setup;
use dotenv::dotenv;
use env_logger::Builder;
use log::{info, LevelFilter};
use reqwest::Client;
use thirtyfour::{error::WebDriverError, DesiredCapabilities, WebDriver};

#[tokio::main]
async fn main() -> Result<(), WebDriverError> {
    dotenv().expect("Failed to load .env file");
    init_logger();
    let http = Client::new();
    info!("Booting up ({})..", env::consts::OS);
    start_chrome_download(&http).await;

    let caps = DesiredCapabilities::chrome();
    let _driver = WebDriver::new("http://localhost:59421", caps).await?;
    info!("Started webdriver");

    info!("Running event loop, startup done!");
    loop {}
}

async fn start_chrome_download(http: &Client) {
    if !chrome_setup::cache_exists() {
        info!("Downloading chrome browser..");
        chrome_setup::download(http, false).await;

        info!("Downloading chrome driver..");
        chrome_setup::download(http, true).await;
    } else {
        info!("Found cache, skipping chrome(driver) download")
    }
}


fn init_logger() {
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .format_target(false)
        .init();
}
