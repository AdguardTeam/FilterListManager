use super::outer_error::AGOuterError;

/// External result repr
pub type AGResult<T> = Result<T, AGOuterError>;
