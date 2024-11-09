use env_logger::Builder;
use log::{info, LevelFilter};
use thirtyfour::{error::WebDriverError, DesiredCapabilities, WebDriver};

#[tokio::main]
async fn main() -> Result<(), WebDriverError>{
    init_logger();
    info!("Booting up..");
    let caps = DesiredCapabilities::chrome();
    let _driver = WebDriver::new("http://localhost:9515", caps).await?;
    info!("Started webdriver");

    info!("Starting event loop, startup done!");
    loop {
        
    }
}


fn init_logger() {
    Builder::new()
    .filter_level(LevelFilter::Debug)
    .format_target(false)
    .init();

}