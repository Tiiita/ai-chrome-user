
use std::{env::{self, consts::OS}, fs::{DirBuilder, File}};

use log::{debug, error};
use reqwest::Client;

const CACHE_PATH: &str = "cache";

//driver = true => driver, driver = false => browser
pub async fn download(client: Client, driver: bool) {
    let response = client.get(build_url(driver)).send().await.expect("Failed to download chrome(driver)");
    debug!("Driver={driver}, Download Res Status: {}", response.status());

    DirBuilder::new()
    .recursive(true)
    .create(CACHE_PATH)
    .expect("Failed to create cache directory");

    let mut zip_path = CACHE_PATH.to_string();
    zip_path.push_str("chrome");

    File::create(zip_path).expect("Failed to create chrome(driver) executable file");
}

//driver = true => driver, driver = false => browser
pub fn build_url(driver: bool) -> String {
    let version = env::var("CHROME_VERSION").expect("Could not find chrome version in .env");
    let mut url = "https://storage.googleapis.com/chrome-for-testing-public/".to_string();
    url.push_str(&version);
    url.push('/');

    match OS {
        "windows" => { 
            url.push_str("win64/");
            if driver { url.push_str("chromedriver-win64.zip") } else { url.push_str("chrome-win64.zip"); }
        },
        "macos" => { 
            url.push_str("mac-arm64/"); 
            if driver { url.push_str("chromedriver-mac-arm64.zip") } else { url.push_str("chrome-mac-arm64.zip"); }
        },
        "linux" => { 
            url.push_str("linux64/"); 
            if driver { url.push_str("chromedriver-linux64.zip") } else { url.push_str("chrome-linux64.zip"); }
        }

        _ => {
            error!("Incopatible operating system: {}", OS);
            panic!()
        },
    }

    url
}