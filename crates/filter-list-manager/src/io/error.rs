use std::io::ErrorKind;

/// General I/O Errors enum
#[derive(Debug, thiserror::Error, PartialEq)]
#[non_exhaustive]
pub enum IOError {
    /// Resource not found
    #[error("Path not found: {0}")]
    NotFound(String),

    /// Resource permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Resource already exist
    #[error("Path already exists: {0}")]
    AlreadyExists(String),

    /// Timeout
    #[error("Timeout: {0}")]
    TimedOut(String),

    #[error("{0}")]
    /// Other errors
    Other(String),
}

impl From<std::io::Error> for IOError {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            ErrorKind::NotFound => Self::NotFound(value.to_string()),
            ErrorKind::PermissionDenied => Self::PermissionDenied(value.to_string()),
            ErrorKind::AlreadyExists => Self::AlreadyExists(value.to_string()),
            ErrorKind::TimedOut => Self::TimedOut(value.to_string()),
            _ => Self::Other(value.to_string()),
        }
    }
}
