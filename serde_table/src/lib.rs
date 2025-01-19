// Re-export from the proc-macro crate.
pub use serde_table_internals::serde_table;
pub use serde_table_internals::serde_table_expr;

use csv;
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub enum TableError {
    Csv(csv::Error),
    Utf8(std::string::FromUtf8Error),
}

impl std::error::Error for TableError {}

impl std::fmt::Display for TableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableError::Csv(e) => write!(f, "CSV error: {}", e),
            TableError::Utf8(e) => write!(f, "UTF-8 conversion error: {}", e),
        }
    }
}

impl From<csv::Error> for TableError {
    fn from(err: csv::Error) -> Self {
        TableError::Csv(err)
    }
}

impl From<std::string::FromUtf8Error> for TableError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        TableError::Utf8(err)
    }
}

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
        eprintln!("row: {:?}", row);
        is_empty = false;
        writer.write_record(row)?;
    }

    if is_empty {
        return Ok(Vec::new());
    }

    writer.flush().unwrap();

    let data = String::from_utf8(writer.into_inner().unwrap())?;
    let mut reader = csv::Reader::from_reader(data.as_bytes());
    let items: Result<Vec<T>, _> = reader.deserialize().collect();
    items.map_err(TableError::Csv)
}
