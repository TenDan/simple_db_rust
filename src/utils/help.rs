
pub fn get_help() {
    println!("List of available commands:");
    println!("\tcreate <db_name|entity_name> {description:>8}", description = "Create entry");
    println!("\tread <db_name|entity_name> {description:->8}", description = "Read from entry");
    println!("\tupdate <db_name|entity_name> {description:>8}", description = "Update entry");
    println!("\tdelete <db_name|entity_name> {description:>8}", description = "Delete entry");
    println!("\topen <db_name> {description:>8}", description = "Open database");
    println!("\thelp {description:>8}", description = "Open this prompt.");
    println!("\tquit {description:>8}", description = "Exit the program.");
}