// Re-export from the proc-macro crate.
pub use serde_table_internals::serde_table;
pub use serde_table_internals::serde_table_expr;

use csv;
use serde::de::DeserializeOwned;

/// You should never need to interact with this struct directly.
pub struct StringRows(Vec<Vec<String>>);

impl StringRows {
    pub fn new(content: &str) -> Self {
        let rows: Vec<Vec<String>> = content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.trim_matches('"').to_string())
                    .collect()
            })
            .collect();
        Self(rows)
    }

    pub fn from_rows(rows: Vec<Vec<String>>) -> Self {
        Self(rows)
    }

    pub fn parse<T>(&self) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
    {
        if self.0.is_empty() {
            return Ok(Vec::new());
        }

        // Convert to CSV format
        let mut writer = csv::Writer::from_writer(Vec::new());
        for row in &self.0 {
            writer.write_record(row)?;
        }
        writer.flush()?;

        // Parse the CSV data
        let data = String::from_utf8(writer.into_inner()?)?;
        let mut reader = csv::Reader::from_reader(data.as_bytes());
        let items: Result<Vec<T>, _> = reader.deserialize().collect();
        items.map_err(|e| e.into())
    }
}
