
struct CreateDatabaseOperation {
    db_name: &'static str,
}

struct UpdateDatabaseOperation {
    db_name: &'static str,
}

struct DeleteDatabaseOperation {
    db_name: &'static str,
}

struct OpenDatabaseOperation {
    db_name: &'static str,
}