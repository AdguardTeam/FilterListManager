use crate::{FLMError, FLMResult, HttpClientError};
use reqwest::{Client, ClientBuilder};
use serde::de::DeserializeOwned;
use std::time::Duration;

/// Standard async client wrapper
pub(crate) struct AsyncHTTPClient {
    inner: Client,
}

impl AsyncHTTPClient {
    /// Async clients factory
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

    /// Gets a json asynchronously from `url` and constructs type `T`
    pub(crate) async fn get_json<T: DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T, HttpClientError> {
        let response = self
            .inner
            .get(url)
            .send()
            .await
            .map_err(HttpClientError::make_network)?
            .error_for_status()
            .map_err(HttpClientError::make_network)?;

        response
            .json::<T>()
            .await
            .map_err(HttpClientError::make_body_recovery)
    }
}
