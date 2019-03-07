use std::process;

#[derive(Debug)]
pub enum Command {
    Exit,
}

impl Command {
    pub fn execute(&self) {
        match *self {
            Command::Exit => process::exit(0),
        }
    }
}
