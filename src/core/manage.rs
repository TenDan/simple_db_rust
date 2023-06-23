use super::error::ErrorMessage;

type StringOrErrorMessage = Result<String, ErrorMessage>;

enum EntryType {
    Database,
    Entity,
}

pub struct Database {

}

// TODO
impl BasicOperations for Database {
    fn create(&self, args: &str) -> StringOrErrorMessage {
        Ok(String::from("Created database"))
    }
    fn update(&self, args: &str) -> StringOrErrorMessage {
        Ok(String::from("Updated database"))
    }
    fn delete(&self, args: &str) -> StringOrErrorMessage {
        Ok(String::from("Deleted database"))
    }
}
impl OpenOperation for Database {
    fn open(&self, args: &str) -> StringOrErrorMessage {
        Ok(String::from("Opened database"))
    }
}

struct Entity {

}

// TODO
//impl BasicOperations for Entity {}
//impl ReadOperation for Entity {}

pub trait BasicOperations {
    fn create(&self, args: &str) -> StringOrErrorMessage;

    fn update(&self, args: &str) -> StringOrErrorMessage;

    fn delete(&self, args: &str) -> StringOrErrorMessage;
}

pub trait ReadOperation {
    fn read(&self, args: &str) -> StringOrErrorMessage;
}

pub trait OpenOperation {
    fn open(&self, args: &str) -> StringOrErrorMessage;
}