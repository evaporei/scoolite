extern crate scoolite;

use scoolite::{build_command, print_error, print_prompt, read_input, Command, Error};

/// scoolite REPL implementation.
fn main() {
    loop {
        print_prompt();

        let input = read_input();

        let command_result = build_command(&input);

        if let Err(error) = try_execute_command(command_result) {
            print_error(&error);
        }
    }
}

fn try_execute_command(command_result: Result<Box<Command>, Error>) -> Result<(), Error> {
    command_result?.execute()
}
