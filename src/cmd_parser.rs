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
    let clean_arg = arg.trim_matches(|c| c == '(' || c == ')');
    let parts: Vec<&str> = clean_arg.split(',').collect();
    let wrong_fmt_msg = "Wrong format.";

    match parts.get(0) {
        Some(id_type) => {
            match id_type {
                &"id" => {
                    match parts.get(1) {
                        Some(a) => { Ok(HtmlElementIdentifier::Id(a.to_string())) },
                        None => { Err(wrong_fmt_msg) },
                    }      
                },

                &"class" => {
                    match parts.get(1..2) {
                        Some(class_args) => {
                            match class_args[1].parse::<usize>() {
                                Ok(index) => { return Ok(HtmlElementIdentifier::Class(class_args[0].to_string(), index)) },
                                Err(_) => { return Err(wrong_fmt_msg) },
                            };
                         },

                        None => { Err(wrong_fmt_msg) },
                    }
                },
                _ => { return Err("Unknown HtmlElementIdentifier") },
            }
        },
        None => { return Err("Unable to extract identifier type. Wrong format!") },
    }
}
