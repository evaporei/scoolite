use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ProcessError {
    message: String,
}

impl ProcessError {
    pub fn new(message: &str) -> Self {
        ProcessError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for ProcessError {
    fn description(&self) -> &str {
        &self.message
    }
}
