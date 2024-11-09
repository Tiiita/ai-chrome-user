use std::env;

use dotenv::dotenv;
use env_logger::Builder;
use log::{info, LevelFilter};
use thirtyfour::{error::WebDriverError, DesiredCapabilities, WebDriver};

#[tokio::main]
async fn main() -> Result<(), WebDriverError>{
    dotenv().expect("Failed to load .env file");
    init_logger();
    info!("Booting up ({})..", env::consts::OS);
    let caps = DesiredCapabilities::chrome();
    let _driver = WebDriver::new("http://localhost:9515", caps).await?;
    info!("Started webdriver");

    info!("Running event loop, startup done!");
    loop {
        
    }
}


fn init_logger() {
    Builder::new()
    .filter_level(LevelFilter::Debug)
    .format_target(false)
    .init();

}