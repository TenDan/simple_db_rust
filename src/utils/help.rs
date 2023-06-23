use crate::core::error::ErrorMessage;


pub fn get_help() -> Option<ErrorMessage> {
    println!("List of available commands:");
    println!("\tcreate (database|entity) <db_name|entity_name> {description:>8}", description = "Create entry");
    println!("\tread <entity_name> {description:->8}", description = "Read from entry");
    println!("\tupdate (database|entity) <db_name|entity_name> {description:>8}", description = "Update entry");
    println!("\tdelete (database|entity) <db_name|entity_name> {description:>8}", description = "Delete entry");
    println!("\topen <db_name> {description:>8}", description = "Open database");
    println!("\thelp {description:>8}", description = "Open this prompt.");
    println!("\tquit {description:>8}", description = "Exit the program.");

    None
}