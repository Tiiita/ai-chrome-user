use std::{env::{self, consts::OS}, io::{stdin, stdout}, process::Command};

use ai_browser::{
    action_executor::{self},
    chrome_setup::{self},
    cmd_parser,
};
use dotenv::dotenv;
use env_logger::Builder;
use log::{debug, error, info, LevelFilter};
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
    caps.set_binary(chrome_setup::get_predicted_path(false).as_str())
        .expect("Failed to set chrome binary path");
    caps.set_no_sandbox().expect("Unable to deactivate sandbox");
    execute_chrome_driver();

    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    info!("Running event loop, startup done!");

    loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        match cmd_parser::parse(buf.trim().to_string()) {
            Ok(action) => {
                action_executor::execute(action, driver.clone()).await;
            }
            Err(err) => {
                error!("Got error: {err}")
            }
        }
    }
}

fn execute_chrome_driver() {
    tokio::spawn(async {
        let predicted_path = chrome_setup::get_predicted_path(true);
        let cmd = if OS.eq("windows") {
           predicted_path
        } else { 
            "./".to_string() + predicted_path.as_str()
        };

        debug!("CMD: {}", cmd);

        Command::new(cmd)
            .arg("--port=9515")
            .stderr(stdout())
            .output()
            .expect("Failed to execute chromedriver binary");
        debug!("Executed chromedriver binary");
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
