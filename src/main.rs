extern crate scoolite;

use scoolite::command::{build_command, Command};
use scoolite::error::Error;
use scoolite::io::{print_error, print_prompt, read_input};
use scoolite::table::Table;

/// scoolite REPL implementation.
fn main() {
    let mut table = Table::new();

    loop {
        print_prompt();

        let input = read_input();

        let command_result = build_command(&input);

        if let Err(error) = try_execute_command(command_result, &mut table) {
            print_error(error);
        }
    }
}

fn try_execute_command(
    command_result: Result<Box<Command>, Error>,
    table: &mut Table,
) -> Result<(), Error> {
    command_result?.execute(table)
}
