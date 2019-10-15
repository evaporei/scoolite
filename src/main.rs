use scoolite::command::run_command;
use scoolite::io::{print_error, print_prompt, read_input};
use scoolite::table::Table;

/// scoolite REPL implementation.
fn main() {
    let mut table = Table::new();

    loop {
        print_prompt();

        let input = read_input();

        match run_command(&mut table, input) {
            Ok(output) => print!("{}", output),
            Err(error) => print_error(error),
        }
    }
}
