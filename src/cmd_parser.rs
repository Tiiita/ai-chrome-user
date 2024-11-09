

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

pub fn parse<'a>(command: String) -> Result<Action, &'a str> {
    let split: Vec<&str> = command.split(" ").collect();
    if split.len() != 2 {
        return Err("Wrong command length");
    }

    let arg = split[1].to_string();

    match split[0] {
        "type" => {
            return Ok(Action::Type(arg));
        }

        //example: "click name(str i32)"
        "click" => {
            let identifier = extract_html_identifier(arg)?;
            return Ok(Action::Click(identifier));
        }

        "seturl" => {
            return Ok(Action::SetUrl(arg));
        }

        _ => {
            return Err("Unknown command");
        }
    }
}

fn extract_html_identifier(arg: String) -> Result<HtmlElementIdentifier, &'static str> {
    if let (Some(start), Some(end)) = (arg.find('('), arg.find(')')) {
        let identifier = &arg[..start].trim();
        let contents = &arg[start + 1..end];
        let parts: Vec<&str> = contents.split_whitespace().collect();
        
        match identifier {
            &"name" | &"class" if parts.len() == 2 => {
                let value = parts[0].to_string();
                if let Ok(index) = parts[1].parse::<usize>() {
                    match identifier {
                        &"name" => Ok(HtmlElementIdentifier::Name(value, index)),
                        &"class" => Ok(HtmlElementIdentifier::Class(value, index)),
                        _ => Err("Unexpected identifier"),
                    }
                } else {
                    Err("Invalid index format")
                }
            }
            &"id" if parts.len() == 1 => Ok(HtmlElementIdentifier::Id(parts[0].to_string())),
            _ => Err("Invalid format: Expected 'name(value index)', 'class(value index)', or 'id(value)'"),
        }
    } else {
        Err("Invalid format: Missing parentheses")
    }
}
