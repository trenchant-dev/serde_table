// Re-export from the proc-macro crate.
pub use serde_table_internals::serde_table;
pub use serde_table_internals::serde_table_expr;

use csv;
use serde::de::DeserializeOwned;

/// You should never need to interact with this struct directly.
pub struct WhitespaceData(String);

impl WhitespaceData {
    pub fn new(content: &str) -> Self {
        Self(content.to_string())
    }

    // TODO Return non box dyn.
    pub fn parse<T>(&self) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
    {
        // Convert whitespace-delimited data to CSV
        let mut csv_data = String::new();
        for line in self.0.lines() {
            // eprintln!("Line: {}", line);
            if line.trim().is_empty() {
                continue;
            }
            let quoted = line
                .split_whitespace()
                .map(|s| format!("\"{}\"", s))
                .collect::<Vec<_>>()
                .join(",");
            if !quoted.is_empty() {
                csv_data.push_str(&quoted);
                csv_data.push('\n');
            }
        }

        // If we have no data, return empty vector
        if csv_data.is_empty() {
            return Ok(Vec::new());
        }

        // eprintln!("CSV data:\n{}", csv_data);

        // Parse using csv reader
        let mut reader = csv::Reader::from_reader(csv_data.as_bytes());
        let items: Result<Vec<T>, _> = reader.deserialize().collect();
        items.map_err(|e| e.into())
    }
}
