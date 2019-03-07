extern crate scoolite;

use scoolite::{print_error, print_prompt, read_input, Command};

fn main() {
    loop {
        print_prompt();

        let input = read_input();

        let command_result = Command::from_str(&input.trim());

        match command_result {
            Ok(command) => command.execute(),
            Err(error) => print_error(&error),
        };
    }
}
