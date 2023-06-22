
use std::{io::{stdin, stdout, Write}, process::exit};
use crate::core::error::*;
use crate::utils::help::get_help;

pub enum PromptCommand {
    Create,
    Read,
    Update,
    Delete,
    Open,
    Help,
    Quit,
}

pub enum StatusCode {
    NoChanges,
    SuccessfulUpdate,
}

fn read_command(cmd: &str) -> Result<PromptCommand, ErrorMessage> {
    match cmd.to_lowercase().as_str() {
        "create" => Ok(PromptCommand::Create),
        "read" => Ok(PromptCommand::Read),
        "update" => Ok(PromptCommand::Update),
        "delete" => Ok(PromptCommand::Delete),
        "help" => Ok(PromptCommand::Help),
        "quit" => Ok(PromptCommand::Quit),
        _ => Err(ErrorMessage::CommandNotFound(String::from(cmd))),
    }
}

fn run_command(prompt_command: PromptCommand, _args: Vec<(usize, &str)>) -> Option<ErrorMessage> {
    match prompt_command {
        PromptCommand::Help => get_help(),
        PromptCommand::Quit => exit(0),
        _ => println!("Not implemented yet"),
    };
    None
}

pub fn command_prompt() -> Option<String> {
    print!("> ");
    let _ = stdout().flush();
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let elements = Vec::from_iter(line.split_ascii_whitespace().enumerate());

    if elements.len() == 0 {
        return None
    }

    let prompt = elements[0];
    let prompt = match read_command(prompt.1) {
        Ok(v) => v,
        Err(e) => return Some(get_command_error(prompt.0, &e.get_error_message())),
    };

    if let Some(err) = 
        run_command(prompt, elements
                                                .iter().skip(1).copied()
                                                .collect()
    ) {
        println!("{}", err.get_error_message());
    };

    return None;
}

fn get_command_error(position: usize, message: &String) -> String {
    format!("Error occured at {} position:\n{}", position + 1, message)
}