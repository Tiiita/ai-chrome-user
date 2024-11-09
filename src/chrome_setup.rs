
use std::{env::{self, consts::OS}, fs::{DirBuilder, File}, io::Write, path::Path};

use log::{debug, error};
use reqwest::Client;
use zip::ZipArchive;

pub const CHROME_CACHE_PATH: &str = "cache/chrome";

//driver = true => driver, driver = false => browser
pub async fn download(client: &Client, driver: bool) {
    let response = client.get(build_url(driver)).send().await.expect("Failed to download chrome(driver)");

    let bytes = response.bytes().await.expect("Failed to get bytes");

    DirBuilder::new()
    .recursive(true)
    .create(CHROME_CACHE_PATH)
    .expect("Failed to create cache directory");

    let mut zip_path = CHROME_CACHE_PATH.to_string();
    zip_path.push_str(if driver { "/driver.zip" } else { "/browser.zip" });

    let mut zip_file = File::create(zip_path).expect("Failed to create chrome(driver) executable file");
    zip_file.write(&bytes).expect("Failed to write zip file bytes to file");
    extract_zip(zip_file);
}

fn extract_zip(file: File) {
    let mut archive = ZipArchive::new(file).expect("Failed to create zip archive");

    debug!("Extracting process starting..");
    archive.extract("test").expect("Failed to extract archive");
}

pub fn cache_exists() -> bool {
    Path::new(CHROME_CACHE_PATH).exists()
}

//driver = true => driver, driver = false => browser
fn build_url(driver: bool) -> String {
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