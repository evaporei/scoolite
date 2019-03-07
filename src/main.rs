use std::io::{self, Write};
use std::error;
use std::fmt;
use std::process;

fn print_prompt() {
    print!("db > ");
}

fn read_input() -> String {
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input
}

#[derive(Debug)]
enum Command {
    Exit,
}

impl Command {
    fn execute(&self) {
        match *self {
            Command::Exit => process::exit(0),
        }
    }
}

#[derive(Debug, Clone)]
struct ProcessError {
    message: String,
}

impl ProcessError {
    fn new(message: &str) -> Self {
        ProcessError { message: message.to_string() }
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

fn process_input(input: String) -> Result<Command, ProcessError> {
    let input = input.trim();

    match input.as_ref() {
        ".exit" => Ok(Command::Exit),
        _ => {
            let message = format!("Error processing input, command '{}' does not exist nor is implemented", input);
            Err(ProcessError::new(&message))
        },
    }
}

fn print_error<T: error::Error>(error: T) {
    println!("{}", error.description());
}

fn main() {
    loop {
        print_prompt();

        let input = read_input();

        let command_result = process_input(input);

        match command_result {
            Ok(command) => command.execute(),
            Err(error) => print_error(error),
        };
    }
}
