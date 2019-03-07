use std::process;

use super::Error;

pub fn build_command(input: String) -> Result<Box<Command>, Error> {
    if input.chars().next() == Some('.') {
        MetaCommand::from_str(&input.trim())
    } else {
        Statement::from_str(&input.trim())
    }
}

fn build_not_implemented_error(input: &str) -> Error {
    let message = format!("Command '{}' does not exist nor is implemented", input);
    Error::new(&message)
}

pub trait Command {
    fn execute(&self);
}

#[derive(Debug, PartialEq)]
enum MetaCommand {
    Exit,
}

impl MetaCommand {
    fn from_str(input: &str) -> Result<Box<Command>, Error> {
        match input {
            ".exit" => Ok(Box::new(MetaCommand::Exit)),
            _ => Err(build_not_implemented_error(input)),
        }
    }
}

impl Command for MetaCommand {
    fn execute(&self) {
        match *self {
            MetaCommand::Exit => process::exit(0),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Statement {
    Insert,
    Select,
}

impl Statement {
    fn from_str(input: &str) -> Result<Box<Command>, Error> {
        match input {
            "insert" => Ok(Box::new(Statement::Insert)),
            "select" => Ok(Box::new(Statement::Select)),
            _ => Err(build_not_implemented_error(input)),
        }
    }
}

impl Command for Statement {
    fn execute(&self) {
        match *self {
            Statement::Insert => println!("insert statement executed"),
            Statement::Select => println!("select statement executed"),
        }
    }
}
