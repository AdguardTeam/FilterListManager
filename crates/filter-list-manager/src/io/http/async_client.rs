use crate::manager::models::configuration::request_proxy_mode::RequestProxyMode;
use crate::{Configuration, FLMError, FLMResult, HttpClientError};
use reqwest::{Client, ClientBuilder, Proxy};
use serde::de::DeserializeOwned;
use std::time::Duration;

/// Standard async client wrapper
pub(crate) struct AsyncHTTPClient {
    inner: Client,
}

impl AsyncHTTPClient {
    /// Async clients factory
    ///
    /// * `configuration` - FLM [`Configuration`]
    pub(crate) fn new(configuration: &Configuration) -> FLMResult<Self> {
        let mut builder = ClientBuilder::new()
            .timeout(Duration::from_millis(
                configuration.request_timeout_ms as u64,
            ))
            .user_agent(format!(
                "{}/{} {}/{}",
                configuration.app_name,
                configuration.version,
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
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
