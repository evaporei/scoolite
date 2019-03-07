extern crate scoolite;

use scoolite::{print_error, print_prompt, process_input, read_input};

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
