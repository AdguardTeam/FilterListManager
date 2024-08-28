use reqwest::StatusCode;

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

    /// Should have only 200 OK successful status code. e.g. 204 would be an error.
    #[error("Expected strictly 200 status code, but {0} given for url: {1}")]
    Strict200Response(u16, String),
}

impl HttpClientError {
    #[inline]
    pub(crate) fn make_network(error: reqwest::Error) -> Self {
        Self::NetworkError(error.to_string())
    }

    #[inline]
    pub(crate) fn make_body_recovery(error: reqwest::Error) -> Self {
        Self::BodyRecoveryFailed(error.to_string())
    }

    #[inline]
    pub(crate) fn make_only_200_strict(actual_code: StatusCode, url: String) -> Self {
        Self::Strict200Response(actual_code.as_u16(), url)
    }
}
