

enum EntryType {
    Database,
    Entity,
}

struct Database {

}

// TODO
//impl BasicOperations for Database {}
//impl OpenOperation for Database {}

struct Entity {

}

// TODO
//impl BasicOperations for Entity {}
//impl ReadOperation for Entity {}

trait BasicOperations {
    fn create(&self, args: &str);

    fn update(&self, args: &str);

    fn delete(&self, args: &str);
}

trait ReadOperation {
    fn read(&self, args: &str);
}

trait OpenOperation {
    fn open(&self, args: &str);
}