//! A macro for parsing tables into Rust structs.
//!
//! ```rust
//! use serde::Deserialize;
//! use serde_table::serde_table;
//!
//! #[derive(Deserialize)]
//! struct Person {
//!     name: String,
//!     age: u32,
//!     city: String,
//! }
//!
//! let people: Vec<Person> = serde_table! {
//!     name    age   city
//!     John    30    NewYork
//!     Jane    25    LosAngeles
//! }.unwrap();
//! ```
//!
//! ## Advanced Usage
//! While `serde_table` ought to do the right thing in general,
//! you can use `serde_table_expr` if you need to avoid the automatic quoting of bare variable-names (identifiers).
//!
//! ## Installation
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! serde_table = "0.1.0"
//! ```
//!

// Re-export from the proc-macro crate.
pub use serde_table_internals::serde_table;
pub use serde_table_internals::serde_table_expr;

use csv;
use serde::de::DeserializeOwned;

/// Errors that can occur when using `serde_table`.
#[derive(Debug)]
pub enum SerdeTableError {
    /// Error when writing a row to the in-memory CSV.
    CsvWriteRow(String, csv::Error),
    /// Error when trying to parse the in-memory CSV row.
    /// Often wrong  column type or number of elements in this row.
    CsvRead(String, csv::Error),
    /// Error when converting the in-memory CSV to a string.
    Utf8(std::string::FromUtf8Error),
}

impl std::error::Error for SerdeTableError {}

impl std::fmt::Display for SerdeTableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerdeTableError::CsvWriteRow(row, e) => {
                write!(f, "CSV Write Row error: {}\nRow: {}", e, row)
            }
            SerdeTableError::CsvRead(data, e) => write!(f, "CSV Read error: {}\nData: {}", e, data),
            SerdeTableError::Utf8(e) => write!(f, "UTF-8 conversion error: {}", e),
        }
    }
}

impl From<std::string::FromUtf8Error> for SerdeTableError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        SerdeTableError::Utf8(err)
    }
}

/// Given a `Vec<Vec<String>>`, parse it into a `Vec<T>>`.
///
/// We do this by writing rows to a CSV in memory and then using serde to read them back.
pub fn parse<T, I, J, S>(rows: I) -> Result<Vec<T>, SerdeTableError>
where
    T: DeserializeOwned,
    I: IntoIterator<Item = J>,
    J: IntoIterator<Item = S>,
    J: std::fmt::Debug,
    S: AsRef<[u8]>,
{
    let mut writer = csv::Writer::from_writer(Vec::new());
    let mut is_empty = true;

    for row in rows {
        is_empty = false;
        let log_row = format!("{:?}", row);
        writer
            .write_record(row)
            .map_err(|e| SerdeTableError::CsvWriteRow(format!("{:?}", log_row), e))?;
    }

    if is_empty {
        return Ok(Vec::new());
    }

    writer.flush().unwrap();

    let data = String::from_utf8(writer.into_inner().unwrap())?;
    let mut reader = csv::Reader::from_reader(data.as_bytes());
    let items: Result<Vec<T>, _> = reader.deserialize().collect();
    items.map_err(|e| SerdeTableError::CsvRead(data, e))
}
