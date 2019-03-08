use std::any::Any;
use std::process;

use super::{AsAny, Error};

/// This function is just a proxy that creates a `Command` or returns an `Error`.
/// The way it decides if it will return a `MetaCommand` or a `Statement` is
/// by looking on the `String` `input` if it starts with a dot (`.`).
pub fn build_command(input: String) -> Result<Box<Command>, Error> {
    if input.chars().next() == Some('.') {
        MetaCommand::from_str(&input.trim())
    } else {
        Statement::from_str(&input.trim())
    }
}

/// Creates an `Error` with the default `"not implemented"` message.
fn build_not_implemented_error(input: &str) -> Error {
    let message = format!("Command '{}' does not exist nor is implemented", input);
    Error::new(&message)
}

/// The interface that every `Command` asks for is just an `execute` method, which
/// executes the specific logic for the `Command`.
pub trait Command: AsAny {
    fn execute(&self);
}

/// `MetaCommand` is the `enum` that contains all meta commands for `scoolite`.
/// An example of meta command is `.exit`, it does not belong to the `SQL` specification
/// however it is used to close the program/REPL.
#[derive(Debug, PartialEq)]
enum MetaCommand {
    Exit,
}

impl MetaCommand {
    /// Tries to parse an `&str` `input` into a `Box<Command>`, if
    /// it fails it returns a `"not implemented error"` `Error`.
    ///
    /// All of the possibilities are just the variants on the `enum`.
    fn from_str(input: &str) -> Result<Box<Command>, Error> {
        match input {
            ".exit" => Ok(Box::new(MetaCommand::Exit)),
            _ => Err(build_not_implemented_error(input)),
        }
    }
}

impl Command for MetaCommand {
    /// Executes an different logic for each variant of the `enum`.
    fn execute(&self) {
        match *self {
            MetaCommand::Exit => process::exit(0),
        }
    }
}

impl AsAny for MetaCommand {
    fn as_any(&self) -> &Any {
        self
    }
}

/// `Statement` is the `enum` that contains all of the statements for `scoolite`.
/// An example of a statement is `insert`, it does belong to the `SQL` specification
/// and it is used to add a row to a table.
#[derive(Debug, PartialEq)]
enum Statement {
    Insert,
    Select,
}

impl Statement {
    /// Tries to parse an `&str` `input` into a `Box<Command>`, if
    /// it fails it returns a `"not implemented error"` `Error`.
    ///
    /// All of the possibilities are just the variants on the `enum`.
    fn from_str(input: &str) -> Result<Box<Command>, Error> {
        match input {
            "insert" => Ok(Box::new(Statement::Insert)),
            "select" => Ok(Box::new(Statement::Select)),
            _ => Err(build_not_implemented_error(input)),
        }
    }
}

impl Command for Statement {
    /// Executes an different logic for each variant of the `enum`.
    fn execute(&self) {
        match *self {
            Statement::Insert => println!("insert statement executed"),
            Statement::Select => println!("select statement executed"),
        }
    }
}

impl AsAny for Statement {
    fn as_any(&self) -> &Any {
        self
    }
}

#[cfg(test)]
mod test {
    use super::{build_command, MetaCommand, Statement};

    #[test]
    fn build_command_meta_command() {
        let input = ".exit".to_string();

        let command = build_command(input).unwrap();

        // stupid necessary casting, because command is a Command trait object
        let command = command.as_any().downcast_ref::<MetaCommand>().unwrap();

        assert_eq!(*command, MetaCommand::Exit);
    }

    #[test]
    fn build_command_statement() {
        let input = "insert".to_string();

        let command = build_command(input).unwrap();

        // stupid necessary casting, because command is a Command trait object
        let command = command.as_any().downcast_ref::<Statement>().unwrap();

        assert_eq!(*command, Statement::Insert);
    }
}
