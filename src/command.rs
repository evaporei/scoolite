use std::process;

use super::Error;

#[derive(Debug, PartialEq)]
pub enum Command {
    Exit,
}

impl Command {
    pub fn from_str(input: &str) -> Result<Self, Error> {
        match input {
            ".exit" => Ok(Command::Exit),
            _ => {
                let message = format!("Command '{}' does not exist nor is implemented", input);
                Err(Error::new(&message))
            }
        }
    }

    pub fn execute(&self) {
        match *self {
            Command::Exit => process::exit(0),
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::Error;
    use super::Command;

    #[test]
    fn from_str_exit() {
        let input = ".exit";

        let command = Command::from_str(input).unwrap();

        assert_eq!(command, Command::Exit);
    }

    #[test]
    fn from_str_command_error() {
        let input = "non existing command";

        let error = Command::from_str(input).unwrap_err();

        let message = format!("Command '{}' does not exist nor is implemented", input);

        assert_eq!(error, Error::new(&message));
    }
}
