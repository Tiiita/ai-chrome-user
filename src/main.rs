use std::{env, process::Command};

use ai_browser::chrome_setup;
use dotenv::dotenv;
use env_logger::Builder;
use log::{debug, info, LevelFilter};
use reqwest::Client;
use thirtyfour::{error::WebDriverError, ChromiumLikeCapabilities, DesiredCapabilities, WebDriver};

#[tokio::main]
async fn main() -> Result<(), WebDriverError> {
    dotenv().expect("Failed to load .env file");
    init_logger();
    let http = Client::new();
    info!("Booting up ({})..", env::consts::OS);

    start_chrome_download(&http).await;

    let mut caps = DesiredCapabilities::chrome();
    caps.set_binary(chrome_setup::get_predicted_path(false).as_str()).expect("Failed to set chrome binary path");
    caps.set_no_sandbox().expect("Unable to deactivate sandbox");
    execute_chrome_driver();

    let _driver = WebDriver::new("http://localhost:9515", caps).await?;
    info!("Started WebDriver");

    info!("Running event loop, startup done!");
    loop {
        
    }
}

fn execute_chrome_driver() {
    tokio::spawn(async {
        Command::new(chrome_setup::get_predicted_path(true))
        .arg("--port=9515")
        .output()
        .expect("Failed to execute chromedriver binary");
    });
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
