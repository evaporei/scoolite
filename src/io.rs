use std::error;
use std::io::{self, Write};

pub fn print_prompt() {
    print!("db > ");
}

pub fn read_input() -> String {
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input
}

pub fn print_error(error: &error::Error) {
    println!("{}", error.description());
}
