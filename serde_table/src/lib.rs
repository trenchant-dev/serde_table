//! [![github]](https://github.com/trenchant-dev/serde_table)&ensp;[![crates-io]](https://crates.io/crates/serde_table)&ensp;[![docs-rs]](https://docs.rs/serde_table)
//! //! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
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

#[derive(Debug)]
pub enum TableError {
    CsvWriteRow(String, csv::Error),
    CsvRead(String, csv::Error),
    Utf8(std::string::FromUtf8Error),
}

impl std::error::Error for TableError {}

impl std::fmt::Display for TableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableError::CsvWriteRow(row, e) => {
                write!(f, "CSV Write Row error: {}\nRow: {}", e, row)
            }
            TableError::CsvRead(data, e) => write!(f, "CSV Read error: {}\nData: {}", e, data),
            TableError::Utf8(e) => write!(f, "UTF-8 conversion error: {}", e),
        }
    }
}

impl From<std::string::FromUtf8Error> for TableError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        TableError::Utf8(err)
    }
}

/// Given a Vec<Vec<String>>, parse it into a Vec<T>>
/// We do this by writing rows to a CSV in memory and then using serde to read them back.
pub fn parse<T, I, J, S>(rows: I) -> Result<Vec<T>, TableError>
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
            .map_err(|e| TableError::CsvWriteRow(format!("{:?}", log_row), e))?;
    }

    if is_empty {
        return Ok(Vec::new());
    }

    writer.flush().unwrap();

    let data = String::from_utf8(writer.into_inner().unwrap())?;
    let mut reader = csv::Reader::from_reader(data.as_bytes());
    let items: Result<Vec<T>, _> = reader.deserialize().collect();
    items.map_err(|e| TableError::CsvRead(data, e))
}
