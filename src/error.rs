use std::error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: &str) -> Self {
        Error {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}
