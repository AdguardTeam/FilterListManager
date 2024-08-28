use crate::io::error::IOError;
use crate::io::ReadFilterFileError;
use crate::HttpClientError;
use std::fmt::{Display, Formatter};

/// Errors for filter parser module
#[non_exhaustive]
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum FilterParserError {
    /// Proxy value for [`std::io::Error`]
    #[error(transparent)]
    Io(IOError),

    /// Proxy value for http client errors
    #[error(transparent)]
    Network(HttpClientError),

    /// Token `!#if` has no condition expression after that
    #[error("EmptyIf")]
    EmptyIf,

    /// At least one `!#else` directive is redundant,
    /// because it is not balanced with the `!#if` directive.
    #[error("UnbalancedElse")]
    UnbalancedElse,

    /// At least one `!#endif` directive is redundant,
    /// because it is not balanced with the `!#if` directive.
    #[error("UnbalancedEndIf")]
    UnbalancedEndIf,

    /// We've found unbalanced if/else/endif expressions after compiling the filter
    #[error("UnbalancedIf")]
    UnbalancedIf,

    /// Recursive filter inclusion detected
    #[error("RecursiveInclusion")]
    RecursiveInclusion,

    /// Stack is corrupted. This is internal error
    #[error("StackIsCorrupted")]
    StackIsCorrupted,

    /// Scheme problems marker
    #[error("SchemeIsIncorrect: {0}")]
    SchemeIsIncorrect(String),

    /// Expression passed into `!#endif` is invalid
    #[error("InvalidBooleanExpression")]
    InvalidBooleanExpression,

    /// Calculated invalid checksum
    #[error("Invalid checksum given: {0}, expected: {1}")]
    InvalidChecksum(String, String),

    /// Next patch is not compiled yet. So the remote file is empty
    #[error("NoContent")]
    NoContent,

    /// When filter's body is not valid filters body by parser's heuristics
    #[error("Filter content is likely not a filter")]
    FilterContentIsLikelyNotAFilter,

    /// Other errors
    #[error("{0}")]
    Other(String),
}

impl FilterParserError {
    #[inline]
    pub(crate) fn other_err_from_to_string<R, S>(error_source: S) -> Result<R, FilterParserError>
    where
        S: ToString,
    {
        Err(FilterParserError::Other(error_source.to_string()))
    }

    #[inline]
    pub(crate) fn other_from_to_string<S>(error_source: S) -> FilterParserError
    where
        S: ToString,
    {
        FilterParserError::Other(error_source.to_string())
    }

    #[inline]
    pub(crate) fn invalid_checksum<R>(
        actual: String,
        expected: String,
    ) -> Result<R, FilterParserError> {
        Err(FilterParserError::InvalidChecksum(actual, expected))
    }
}

impl From<ReadFilterFileError> for FilterParserError {
    fn from(value: ReadFilterFileError) -> Self {
        match value {
            ReadFilterFileError::Io(io_err) => Self::Io(io_err),
            ReadFilterFileError::Other(other_err) => Self::Other(other_err),
        }
    }
}

/// A structure containing an error that occurred during filter parsing and additional context.
#[derive(Debug, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
pub struct FilterParserErrorContext {
    /// Absolute url
    pub file: String,
    /// Lineno
    pub line: usize,
    /// Error
    pub error: FilterParserError,
}

impl FilterParserErrorContext {
    pub(crate) fn new(error: FilterParserError, line: usize, file: String) -> Self {
        Self { error, line, file }
    }
}

impl Display for FilterParserErrorContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parser error: \"{}\" encountered in {}:{}",
            self.error, self.file, self.line
        )
    }
}
