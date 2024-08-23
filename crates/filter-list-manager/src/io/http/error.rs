/// Common HTTP client errors enum
#[non_exhaustive]
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum HttpClientError {
    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Deserialization/Body reading failed
    #[error("Body recovery failed: {0}")]
    BodyRecoveryFailed(String),
}

impl HttpClientError {
    pub(crate) fn make_network(error: reqwest::Error) -> Self {
        Self::NetworkError(error.to_string())
    }

    pub(crate) fn make_body_recovery(error: reqwest::Error) -> Self {
        Self::BodyRecoveryFailed(error.to_string())
    }
}
