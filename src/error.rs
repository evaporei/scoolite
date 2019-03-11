use std::error;
use std::fmt;

/// Main `Error` `struct`, it holds any `message` as `String`.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    UnrecognizedStatement(String),
    SyntaxError(String),
}

impl Error {
    /// This function just get's the description depending of
    /// each variant of the `Error` type, usually the first
    /// field.
    fn get_description(&self) -> &str {
        match self {
            Error::UnrecognizedStatement(description) => description,
            Error::SyntaxError(description) => description,
        }
    }
}

impl fmt::Display for Error {
    /// Just writes the `message` of the `Error` to the `stdout`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_description())
    }
}

impl error::Error for Error {
    /// The description is just the private field `message`.
    fn description(&self) -> &str {
        self.get_description()
    }
}
