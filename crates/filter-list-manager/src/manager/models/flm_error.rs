//! General purpose error enum for all [`crate::FilterListManager`] methods
use crate::filters::parser::parser_error::FilterParserErrorContext;
use crate::io::error::IOError;
use crate::io::http::error::HttpClientError;
use crate::storage::error::DatabaseError;
use std::fmt::Display;

/// General purpose error enum for all [`crate::FilterListManager`] methods
#[cfg_attr(test, derive(PartialEq))]
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum FLMError {
    #[error(transparent)]
    /// Database errors
    Database(DatabaseError),

    /// Some entity not found
    #[error("Entity `{0}` not found")]
    EntityNotFound(i64),

    /// I/O errors. [`IOError`]
    #[error(transparent)]
    Io(IOError),

    /// Network client errors
    #[error(transparent)]
    Network(HttpClientError),

    /// Parse filter error. It represents `path:lineno` information about error location and [`crate::FilterParserError`] object.
    #[error(transparent)]
    ParseFilterError(FilterParserErrorContext),

    /// Form-like error. You may specify first parameter as a field key
    #[error("FieldEmpty: {0}")]
    FieldIsEmpty(&'static str),

    /// Invalid configuration error
    #[error("InvalidConfiguration: {0}")]
    InvalidConfiguration(&'static str),

    /// Other errors
    #[error("{0}")]
    Other(String),
}

impl FLMError {
    /// Makes [`Self::Database`] error from [`rusqlite::Error`]
    #[inline]
    pub(crate) fn from_database(error: rusqlite::Error) -> Self {
        Self::Database(error.into())
    }
    /// Makes [`Self::Io`] error from [`std::io::Error`]
    #[inline]
    pub(crate) fn from_io(error: std::io::Error) -> Self {
        Self::Io(error.into())
    }

    /// Makes [`Self::Other`] error with copy of [`std::str`], wrapped with [`Result::Err`]
    #[inline]
    pub(crate) fn make_err<T>(error: impl Into<String>) -> Result<T, Self> {
        Err(Self::from_str(error))
    }

    /// Makes [`Self::Other`] error with copy of [`std::str`]
    #[inline]
    pub(crate) fn from_str(error: impl Into<String>) -> Self {
        Self::Other(error.into())
    }

    /// Makes [`Self::Other`] error from [`Display`] value
    #[inline]
    pub(crate) fn from_display<T>(err: T) -> Self
    where
        T: Display,
    {
        Self::Other(err.to_string())
    }

    /// Makes [`Self::ParseFilterError`] error with [`FilterParserErrorContext`]
    #[inline]
    pub(crate) fn from_parser_error(error: FilterParserErrorContext) -> Self {
        Self::ParseFilterError(error)
    }
}

impl From<rusqlite::Error> for FLMError {
    fn from(value: rusqlite::Error) -> Self {
        Self::from_database(value)
    }
}
