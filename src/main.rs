extern crate scoolite;

use scoolite::{print_prompt, read_input, process_input, print_error};

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
