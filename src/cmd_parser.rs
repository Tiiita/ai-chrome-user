use crate::action_executor::{Action, HtmlElementIdentifier};





pub fn parse<'a>(command: String) -> Result<Action, &'a str> {
     match command.split_once(' ') {
        Some(split) => {
            let arg = split.1.to_string();
            let command = split.0;

            match command {
                "type" => {
                    return Ok(Action::Type(arg));
                }
        
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
        },
        None => { return Err("Wrong format"); },
    };
}

fn parse_expression(input: &str) -> Vec<String> {
    let mut split = input
        .split(|c| c == '(' || c == ',')
        .map(|i| i.trim().to_owned())
        .collect::<Vec<_>>();

    if let Some(last) = split.last() {
        let last_modified = last.replace(")", "").trim().to_string();
        let last_index = split.len() - 1;
        split[last_index] = last_modified;
    }

    split
}

fn extract_html_identifier(arg: String) -> Result<HtmlElementIdentifier, &'static str> {
    let parts: Vec<String> = parse_expression(arg.as_str());
    let wrong_fmt_msg = "Wrong format.";

    match parts.get(0) {
        Some(id_type) => match id_type.as_str() {
            "id" => match parts.get(1) {
                Some(a) => Ok(HtmlElementIdentifier::Id(a.to_string())),
                None => Err(wrong_fmt_msg),
            },

            "class" => match parts.get(1..3) {
                Some(class_args) => {
                    match class_args[1].parse::<usize>() {
                        Ok(index) => {
                            return Ok(HtmlElementIdentifier::Class(
                                class_args[0].to_string(),
                                index,
                            ))
                        }
                        Err(_) => return Err(wrong_fmt_msg),
                    };
                }

                None => Err(wrong_fmt_msg),
            },
            _ => return Err("Unknown HtmlElementIdentifier"),
        },
        None => return Err("Unable to extract identifier type. Wrong format!"),
    }
}
