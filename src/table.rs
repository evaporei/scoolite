use super::Row;

/// In memory storage of `Row`s.
pub struct Table {
    rows: Vec<Row>,
}

impl Table {
    /// Creates a new table.
    pub fn new() -> Self {
        Table { rows: vec![] }
    }

    /// Adds a `Row` into the `rows` `Vec`.
    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    /// Returns a reference to all `Row`s inside the table.
    pub fn list_rows(&self) -> &Vec<Row> {
        &self.rows
    }
}
