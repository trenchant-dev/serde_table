// Re-export from the proc-macro crate.
pub use serde_table_internals::serde_table;
pub use serde_table_internals::serde_table_expr;

use csv;
use serde::de::DeserializeOwned;

pub fn parse<T>(rows: Vec<Vec<String>>) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    if rows.is_empty() {
        return Ok(Vec::new());
    }

    let mut writer = csv::Writer::from_writer(Vec::new());
    for row in &rows {
        writer.write_record(row)?;
    }
    writer.flush()?;

    let data = String::from_utf8(writer.into_inner()?)?;
    let mut reader = csv::Reader::from_reader(data.as_bytes());
    let items: Result<Vec<T>, _> = reader.deserialize().collect();
    items.map_err(|e| e.into())
}
