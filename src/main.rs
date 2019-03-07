extern crate scoolite;

use scoolite::Command;
use scoolite::ProcessError;

use std::io::{self, Write};
use std::error;

fn print_prompt() {
    print!("db > ");
}

fn read_input() -> String {
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input
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

fn print_error(error: &error::Error) {
    println!("{}", error.description());
}

fn main() {
    loop {
        print_prompt();

        let input = read_input();

        let command_result = process_input(input);

        match command_result {
            Ok(command) => command.execute(),
            Err(error) => print_error(&error),
        };
    }
}
