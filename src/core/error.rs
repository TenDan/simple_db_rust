

pub enum ErrorMessage {
    CommandNotFound(String),
    NotImplemented(String),
    Other(String)
}

impl ErrorMessage {
    pub fn get_error_message(&self) -> String {
        match &self {
            Self::CommandNotFound(cmd) => format!("Command {} not found", cmd),
            Self::Other(msg) => format!("{}", msg),
            _ => panic!("Not implemented"),
        }
    }
}
