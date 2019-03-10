use super::Error;
use std::fmt;

/// This struct contains the first table
/// available for storage. It will be
/// defined by the user eventually.
pub struct Row {
    id: usize,
    username: String,
    email: String,
}

impl Row {
    /// Receives an input like `1 john john@mailbox.com` and
    /// it builds a `Row` with these fields. If any errors happen
    /// on the parse step, it will return an `Error`.
    pub fn from_str(input: &str) -> Result<Self, Error> {
        let data: Vec<&str> = input.split(' ').skip(1).collect();

        Ok(Row {
            id: data
                .get(0)
                .and_then(|id_string| id_string.parse::<usize>().ok())
                .ok_or(Error::new("Failed to parse 'id' of input"))?,
            username: data
                .get(1)
                .map(|a| a.to_string())
                .ok_or(Error::new("Failed to parse 'username' of input"))?,
            email: data
                .get(2)
                .map(|a| a.to_string())
                .ok_or(Error::new("Failed to parse 'email' of input"))?,
        })
    }
}

impl fmt::Display for Row {
    /// A row like:
    /// `Row { id: 1, username: "john".to_string(), email: "john@mailbox.com".to_string() }`
    ///
    /// Will be printed like:
    /// `(1 john john@mailbox.com)`
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.id, self.username, self.email)
    }
}
