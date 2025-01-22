use crate::{FLMError, FLMResult, HttpClientError};
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::StatusCode;
use std::time::Duration;

/// Standard blocking client wrapper
pub(crate) struct BlockingClient {
    inner: Client,
}

impl BlockingClient {
    /// Blocking clients factory
    ///
    /// * `timeout_ms` - Requests timeout value in milliseconds
    pub(crate) fn new(timeout_ms: i32) -> FLMResult<Self> {
        Ok(Self {
            inner: ClientBuilder::new()
                .timeout(Duration::from_millis(timeout_ms as u64))
                .build()
                .map_err(FLMError::from_display)?,
        })
    }

    /// Gets filter with special rules processing:
    /// - StatusCode == 200, see below
    ///
    /// # Failure
    ///
    /// If status_code != 200, (e.g. 204), requests will fail with [`HttpClientError::Strict200Response`]
    pub(crate) fn get_filter(&self, url: &str) -> Result<String, HttpClientError> {
        let response = self
            .inner
            .get(url)
            .send()
            .map_err(HttpClientError::make_network)?
            .error_for_status()
            .map_err(HttpClientError::make_network)?;

        let status = response.status();

        if status != StatusCode::OK {
            return Err(HttpClientError::make_only_200_strict(
                status,
                url.to_owned(),
            ));
        }

        response.text().map_err(HttpClientError::make_body_recovery)
    }
}
