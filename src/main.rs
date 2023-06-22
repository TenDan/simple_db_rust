
mod core;
mod utils;
use crate::core::command;

fn main() {
    loop {
        let result = command::command_prompt();

        match result {
            Some(err) => print_error(&err),
            _ => {},
        }
    }
}

fn print_error(err: &str) {
    println!("{}", err);
    println!("Type `help` for info about available commands");
}
