use crate::manager::models::configuration::request_proxy_mode::RequestProxyMode;
use crate::{Configuration, FLMError, FLMResult, HttpClientError};
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::{Proxy, StatusCode};
use std::time::Duration;

/// Standard blocking client wrapper
pub(crate) struct BlockingClient {
    inner: Client,
}

impl BlockingClient {
    /// Blocking clients factory
    ///
    /// * `configuration` - FLM [`Configuration`]
    pub(crate) fn new(configuration: &Configuration) -> FLMResult<Self> {
        let mut builder = ClientBuilder::new().timeout(Duration::from_millis(
            configuration.request_timeout_ms as u64,
        ));

        match configuration.request_proxy_mode {
            RequestProxyMode::UseSystemProxy => {}
            RequestProxyMode::NoProxy => {
                builder = builder.no_proxy();
            }
            RequestProxyMode::UseCustomProxy { ref addr } => {
                builder = builder.proxy(Proxy::all(addr).map_err(FLMError::from_display)?)
            }
        }

        let client = builder.build().map_err(FLMError::from_display)?;

        Ok(Self { inner: client })
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
