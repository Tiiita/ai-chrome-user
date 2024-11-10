
use log::{error, info, warn};
use thirtyfour::{By, WebDriver, WebElement};

#[derive(Debug)]
pub enum Action {
    Type(String),
    Click(HtmlElementIdentifier),
    SetUrl(String),
}

#[derive(Debug, Clone)]
pub enum HtmlElementIdentifier {
    Name(String, usize), //Element by name and the index of the list
    Class(String, usize),
    Id(String),
}

pub async fn execute(action: Action, driver: WebDriver) {
    match action {
        Action::Click(identifier) => {
            let element = get_element(identifier.clone(), driver).await;
            match element {
                Some(element) => { 
                    click(element, identifier).await;
                },
                None => { warn!("Unable to locate element: {:?}", identifier) }
            }
        },

        Action::SetUrl(url) => {
            match driver.goto(url.clone()).await {
                Ok(_) => { info!("Changed url to: {url}") },
                Err(why) => {  error!("Failed to go to url. Err: {:?}", why); },
            }
        }
        Action::Type(_text) => {
            todo!("Implement this by saving last clicked to send keys");
        },
       
    }
}

async fn get_element(identifier: HtmlElementIdentifier, driver: WebDriver) -> Option<WebElement> {
    return match identifier {
        HtmlElementIdentifier::Name(name, index) => {
            if let Ok(elements) = driver.find_all(By::Name(name)).await {
                return elements.get(index).cloned()
            }

             None
        },

        HtmlElementIdentifier::Class(name, index) => {
            if let Ok(elements) = driver.find_all(By::ClassName(&name)).await {
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