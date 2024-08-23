//! Common database errors
use rusqlite::{Error, ErrorCode};

/// Common database errors enum
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[cfg_attr(test, derive(PartialEq))]
pub enum DatabaseError {
    /// Cannot open file by the following path
    #[error("CannotOpen")]
    CannotOpen,

    /// File opened that is not a database file
    #[error("NotADatabase")]
    NotADatabase,

    /// Cannot operate, because disc is full
    #[error("DiskFull")]
    DiskFull,

    /// Other error
    #[error("{0}")]
    Other(String),
}

impl From<Error> for DatabaseError {
    fn from(value: Error) -> Self {
        match value {
            Error::SqliteFailure(code, _) => match code.code {
                ErrorCode::DiskFull => Self::DiskFull,
                ErrorCode::CannotOpen => Self::CannotOpen,
                ErrorCode::NotADatabase => Self::NotADatabase,
                _ => Self::Other(value.to_string()),
            },
            _ => Self::Other(value.to_string()),
        }
    }
}
