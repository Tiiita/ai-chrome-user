use log::{error, info, warn};
use thirtyfour::{By, Key, WebDriver, WebElement};

#[derive(Debug, Clone)]
pub enum Action {
    Type(String),
    Click(HtmlElementIdentifier),
    GoTo(String),
    PressEnter,
}

#[derive(Debug, Clone)]
pub enum HtmlElementIdentifier {
    Name(String, usize), //Element by name and the index of the list
    Class(String, usize),
    Id(String),
}

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    action: Action,
    interacted_element: Option<WebElement>,
}

pub async fn execute(action: Action, driver: WebDriver, action_history: &mut Vec<HistoryEntry>) {
    match action.clone() {
        Action::Click(identifier) => {
            let element = get_element(identifier.clone(), driver).await;
            match element {
                Some(element) => {
                    click(&element, identifier).await;
                    action_history.push(HistoryEntry {
                        action,
                        interacted_element: Some(element),
                    });
                }
                None => {
                    warn!("Unable to locate element: {:?}", identifier)
                }
            }
        }

        Action::GoTo(url) => match driver.goto(url.clone()).await {
            Ok(_) => {
                action_history.push(HistoryEntry {
                    action,
                    interacted_element: None,
                });
                info!("Changed url to: {url}")
            }
            Err(why) => {
                error!("Failed to go to url. Err: {:?}", why);
            }
        },
        Action::Type(text) => {
            let clicks = get_history_clicks(action_history);

            match clicks.last() {
                Some(click_entry) => match click_entry.interacted_element.clone() {
                    Some(element) => match element.send_keys(text.clone()).await {
                        Ok(_) => {
                            info!("Wrote text: {}", text);
                            action_history.push(HistoryEntry {
                                action,
                                interacted_element: Some(element),
                            });
                        }
                        Err(why) => {
                            error!("Failed to send keys: {:?}", why)
                        }
                    },
                    None => {
                        warn!("Last clicked element not found")
                    }
                },
                None => {
                    warn!("No target field clicked")
                }
            }
        }

        Action::PressEnter => {
            let clicks = get_history_clicks(action_history);
            match clicks.last() {
                Some(click_entry) => match click_entry.interacted_element.clone() {
                    Some(ele) => match ele.send_keys(Key::Enter).await {
                        Ok(_) => {
                            info!(
                                "Sent press key to last clicked element: {:?}",
                                ele.element_id
                            )
                        }
                        Err(why) => {
                            error!(
                                "Failed to send enter keys to last clicked element: {:?}",
                                why
                            )
                        }
                    },
                    None => {
                        warn!("No interaction element in last clicked entry found")
                    }
                },
                None => {
                    warn!("No last clicked element for which can receive enter keys")
                }
            }
        }
    }
}

pub async fn execute_from_file() {}

//Gets all history entries which are from type Action::Click
fn get_history_clicks(action_history: &Vec<HistoryEntry>) -> Vec<HistoryEntry> {
    let mut clicks = Vec::new();

    for ele in action_history.iter() {
        match ele.action {
            Action::Click(_) => clicks.push(ele.clone()),
            _ => {}
        }
    }

    clicks
}

async fn get_element(identifier: HtmlElementIdentifier, driver: WebDriver) -> Option<WebElement> {
    return match identifier {
        HtmlElementIdentifier::Name(name, index) => {
            if let Ok(elements) = driver.find_all(By::Name(name)).await {
                return elements.get(index).cloned();
            }

            None
        }

        HtmlElementIdentifier::Class(name, index) => {
            if let Ok(elements) = driver.find_all(By::ClassName(&name)).await {
                return elements.get(index).cloned();
            }

            None
        }

        HtmlElementIdentifier::Id(id) => match driver.find(By::Id(id)).await {
            Ok(element) => Some(element),
            Err(_) => None,
        },
    };
}

async fn click(element: &WebElement, identifier: HtmlElementIdentifier) {
    match element.click().await {
        Ok(_) => {
            info!("Clicked element: {:?}", identifier)
        }
        Err(why) => {
            error!(
                "Failed to click element: {:?}. Error: {:?}",
                identifier, why
            )
        }
    }
}
