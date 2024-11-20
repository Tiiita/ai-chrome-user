use std::{fs::File, io::Read};

use log::error;

use crate::action_executor::{Action, HtmlElementIdentifier};

pub fn parse<'a>(command: String) -> Result<Action, &'a str> {
     match command.split_once(' ') {
        Some(split) => {
            let arg = split.1.to_string();
            let command = split.0;

            match command {
                "type" => {
                    return Ok(Action::Type(arg));
                },
        
                "click" => {
                    let identifier = extract_html_identifier(arg)?;
                    return Ok(Action::Click(identifier));
                },
        
                "goto" => {
                    return Ok(Action::GoTo(arg));
                },

                "enter" => {
                    return Ok(Action::PressEnter);
                },

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


//Panics when getting file error!
pub fn parse_commands_from_file(path: &str) -> Vec<Action> {
    let mut cmds = Vec::new();
    let mut file = File::open(path).expect("Failed to open commands file");
    let mut content_buf = String::new();
    file.read_to_string(&mut content_buf).expect("Failed to read file");


    let cmd_strings: Vec<&str> = content_buf.split("\n").collect();

    for mut ele in cmd_strings {
        ele = ele.trim();
        if ele.len() == 0 {
            continue;
        }

         match parse(ele.to_string()) {
            Ok(action) => {
                cmds.push(action);
            },
            Err(why) => { error!("Unable to parse command: {:?}", why) },
        }
    }

    cmds
}
