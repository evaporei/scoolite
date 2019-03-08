extern crate scoolite;

use scoolite::{build_command, print_error, print_prompt, read_input};

/// scoolite REPL implementation.
fn main() {
    loop {
        print_prompt();

        let input = read_input();

        let command_result = build_command(input);

        match command_result {
            Ok(command) => command.execute(),
            Err(error) => print_error(&error),
        };
    }
}
