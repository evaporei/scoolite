use std::error;
use std::io::{self, Write};

/// Just prints `db > ` to the terminal.
pub fn print_prompt() {
    print!("db > ");
}

/// This function flushes the stdout, so
/// that no outputs to the terminal get's
/// stuck. Then it reads a line from stdin and
/// returns it as a `String`.
pub fn read_input() -> String {
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input
}

/// Just prints an `Error`'s `description` to the terminal.
pub fn print_error<E: error::Error>(error: E) {
    println!("{}", error.description());
}
