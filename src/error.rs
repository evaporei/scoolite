use std::error;
use std::fmt;

/// Main `Error` `struct`, it holds any `message` as `String`.
#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    message: String,
}

impl Error {
    /// Creates a new `Error`.
    pub fn new(message: &str) -> Self {
        Error {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for Error {
    /// Just writes the `message` of the `Error` to the `stdout`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Error {
    /// The description is just the private field `message`.
    fn description(&self) -> &str {
        &self.message
    }
}
