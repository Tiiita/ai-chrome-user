use log::{error, info, warn};
use thirtyfour::{By, WebDriver, WebElement};

use crate::cmd_parser::{Action, HtmlElementIdentifier};


pub async fn execute(action: Action, driver: WebDriver) {
    match action {
        Action::Click(identifier) => {
            let element = get_element(identifier.clone(), driver).await;
            match element {
                Some(element) => { click(element, identifier).await },
                None => { warn!("Unable to locate element: {:?}", identifier) }
            }
        },
        _ => {
            error!("Cannot execute unknown action: {:?}", action);
        }
    }
}

async fn get_element(identifier: HtmlElementIdentifier, driver: WebDriver) -> Option<WebElement> {
    return match identifier {
        HtmlElementIdentifier::Name(name, index) | 
        HtmlElementIdentifier::Class(name, index) => {
            if let Ok(elements) = driver.find_all(By::Name(name)).await {
                return elements.get(index).cloned()
            }

             None
        },
    
        HtmlElementIdentifier::Id(id) => {
            match driver.find(By::Id(id)).await {
               Ok(element) => { Some(element) },
               Err(_) => { None },
           }
        }
    }
}

async fn click(element: WebElement, identifier: HtmlElementIdentifier) {
    match element.click().await {
        Ok(_) => { info!("Clicked element: {:?}", identifier) },
        Err(why) => { error!("Failed to click element: {:?}. Error: {:?}", identifier, why) },
    }
}