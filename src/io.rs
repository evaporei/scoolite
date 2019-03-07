use std::error;
use std::io::{self, Write};

use super::{Command, ProcessError};

pub fn print_prompt() {
    print!("db > ");
}

pub fn read_input() -> String {
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input
}

pub fn process_input(input: String) -> Result<Command, ProcessError> {
    let input = input.trim();

    match input.as_ref() {
        ".exit" => Ok(Command::Exit),
        _ => {
            let message = format!(
                "Error processing input, command '{}' does not exist nor is implemented",
                input
            );
            Err(ProcessError::new(&message))
        }
    }
}

pub fn print_error(error: &error::Error) {
    println!("{}", error.description());
}

#[cfg(test)]
mod test {
    use super::super::{Command, ProcessError};
    use super::process_input;

    #[test]
    fn process_input_exit() {
        let input = ".exit\n".to_string();

        let command = process_input(input).unwrap();

        assert_eq!(command, Command::Exit);
    }

    #[test]
    fn process_input_process_error() {
        let input = "non existing command\n".to_string();

        let error = process_input(input.clone()).unwrap_err();

        let message = format!(
            "Error processing input, command '{}' does not exist nor is implemented",
            input.trim()
        );

        assert_eq!(error, ProcessError::new(&message));
    }
}
