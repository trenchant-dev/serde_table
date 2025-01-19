// Re-export from the proc-macro crate.
pub use serde_table_internals::serde_table;
pub use serde_table_internals::serde_table_expr;

use csv;
use serde::de::DeserializeOwned;

pub fn parse<T, I, J, S>(rows: I) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
    I: IntoIterator<Item = J>,
    J: IntoIterator<Item = S>,
    S: AsRef<[u8]>,
{
    let mut writer = csv::Writer::from_writer(Vec::new());
    let mut is_empty = true;

    for row in rows {
        is_empty = false;
        writer.write_record(row)?;
    }

    if is_empty {
        return Ok(Vec::new());
    }

    writer.flush()?;

    let data = String::from_utf8(writer.into_inner()?)?;
    let mut reader = csv::Reader::from_reader(data.as_bytes());
    let items: Result<Vec<T>, _> = reader.deserialize().collect();
    items.map_err(|e| e.into())
}
