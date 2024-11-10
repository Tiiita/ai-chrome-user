
use std::{env::{self, consts::OS}, fs::{DirBuilder, File}, io::Write, path::Path};

use log::error;
use reqwest::Client;
use zip::ZipArchive;

pub const CHROME_CACHE_PATH: &str = "cache/chrome";
const DRIVER_ZIP_NAME: &str = "/driver.zip";
const BROWSER_ZIP_NAME: &str = "/browser.zip";

//driver = true => driver, driver = false => browser
pub async fn download(client: &Client, driver: bool) {
    let response = client.get(build_url(driver)).send().await.expect("Failed to download chrome(driver)");

    let bytes = response.bytes().await.expect("Failed to get bytes");

    DirBuilder::new()
    .recursive(true)
    .create(CHROME_CACHE_PATH)
    .expect("Failed to create cache directory");

    let mut zip_path = CHROME_CACHE_PATH.to_string();
    zip_path.push_str(if driver { DRIVER_ZIP_NAME } else { BROWSER_ZIP_NAME });

    let mut zip_file = File::create(&zip_path).expect("Failed to create chrome(driver) executable file");
    zip_file.write_all(&bytes).expect("Failed to write zip file bytes to file");
    drop(zip_file);
    
    extract_zip(File::open(zip_path).expect("Failed to open zip file"));
}

fn extract_zip(file: File) {
    let mut archive = ZipArchive::new(file).expect("Failed to create zip archive");
    archive.extract(CHROME_CACHE_PATH).expect("Failed to extract archive");
}

pub fn cache_exists() -> bool {
    Path::new(CHROME_CACHE_PATH).exists()
}

pub fn get_predicted_path(driver: bool) -> String {
    let key = match OS {
        "windows" => { if driver { "CHROME_DRIVER_WIN" } else { "CHROME_BINARY_WIN" } },
        "macos" => { if driver { "CHROME_DRIVER_MAC" } else { "CHROME_BINARY_MAC" } },
        "linux" => { if driver { "CHROME_DRIVER_LINUX" } else { "CHROME_BINARY_LINUX" } },
        _ => { 
            incompatible_os();
            "" 
        }
    };
 
    env::var(key).expect("Failed to get chrome(driver) path from .env")
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

        _ => { incompatible_os() },
    }

    url
}

fn incompatible_os() {
    error!("Incopatible operating system: {}", OS);
    panic!()
}