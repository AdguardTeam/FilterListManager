use adguard_flm::storage::error::DatabaseError;
use adguard_flm::{FLMError, FilterParserError, HttpClientError, IOError};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum AGOuterError {
    /// Cannot open file by the following path
    #[error("Cannot open database")]
    CannotOpenDatabase,

    /// File opened that is not a database file
    #[error("This file is not a database")]
    NotADatabase,

    #[error("Database is busy")]
    DatabaseBusy,

    /// Cannot operate, because disc is full
    #[error("Disk is full")]
    DiskFull,

    /// Cannot find some entity in database
    #[error("Entity `{0}` not found")]
    EntityNotFound(i64),

    /// Path not found
    #[error("PathNotFound: {0}")]
    PathNotFound(String),

    /// Path permission denied
    #[error("Permission denied: {0}")]
    PathHasDeniedPermission(String),

    /// Path already exists
    #[error("Path already exists: {0}")]
    PathAlreadyExists(String),

    /// Timeout
    #[error("Timeout: {0}")]
    TimedOut(String),

    /// Http client network error
    #[error("HttpClientNetworkError: {0}")]
    HttpClientNetworkError(String),

    /// For a few requests we strictly check response code 200. 204, for example, considered erroneous
    #[error("Expected strictly 200 status code, but {0} given for url: {1}")]
    HttpStrict200Response(u16, String),

    /// Http client deserialization/body reading filed
    #[error("HttpClientBodyError: {0}")]
    HttpClientBodyRecoveryFailed(String),

    /// Downloaded filter body likely is not a filter. This might be a html page, for example
    #[error("{0}")]
    FilterContentIsLikelyNotAFilter(String),

    /// Filter parser/compiler error
    #[error("ParserError: {0}")]
    FilterParserError(String),

    #[error("Field is empty: {0}")]
    FieldIsEmpty(&'static str),

    /// Mutex poisoned
    #[error("Mutex failed: {0}")]
    Mutex(String),

    /// Invalid configuration error
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(&'static str),

    /// Other errors
    #[error("{0}")]
    Other(String),
}

impl From<FLMError> for AGOuterError {
    fn from(value: FLMError) -> Self {
        match value {
            FLMError::Database(variant) => match variant {
                DatabaseError::CannotOpen => Self::CannotOpenDatabase,
                DatabaseError::NotADatabase => Self::NotADatabase,
                DatabaseError::DatabaseBusy => Self::DatabaseBusy,
                DatabaseError::DiskFull => Self::DiskFull,
                DatabaseError::Other(str) => Self::Other(format!("Other database error: {}", str)),
                _ => Self::Other(String::from("Unknown db error")),
            },
            FLMError::Io(variant) => match variant {
                IOError::NotFound(msg) => Self::PathNotFound(msg),
                IOError::PermissionDenied(msg) => Self::PathHasDeniedPermission(msg),
                IOError::AlreadyExists(msg) => Self::PathAlreadyExists(msg),
                IOError::TimedOut(msg) => Self::TimedOut(msg),
                IOError::Other(msg) => Self::Other(format!("Other I/O error: {}", msg)),
                _ => Self::Other(String::from("Unknown i/o error")),
            },
            FLMError::FieldIsEmpty(value) => Self::FieldIsEmpty(value),
            FLMError::InvalidConfiguration(value) => Self::InvalidConfiguration(value),
            FLMError::EntityNotFound(id) => Self::EntityNotFound(id),
            FLMError::Other(msg) => Self::Other(msg),
            FLMError::Network(variant) => match variant {
                HttpClientError::NetworkError(err) => Self::HttpClientNetworkError(err),
                HttpClientError::BodyRecoveryFailed(err) => Self::HttpClientBodyRecoveryFailed(err),
                HttpClientError::Strict200Response(code, url) => {
                    Self::HttpStrict200Response(code, url)
                }
                _ => Self::Other(String::from("Unknown network error")),
            },
            FLMError::ParseFilterError(error) => match error.error {
                FilterParserError::FilterContentIsLikelyNotAFilter => {
                    Self::FilterContentIsLikelyNotAFilter(error.to_string())
                }
                _ => Self::FilterParserError(error.to_string()),
            },
            _ => Self::Other(String::from("Unknown error")),
        }
    }
}
