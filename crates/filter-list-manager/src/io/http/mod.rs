pub mod error;

use self::error::HttpClientError;
use reqwest::blocking::{Client as BlockingClient, ClientBuilder as BlockingClientBuilder};
use reqwest::{Client as AsyncClient, ClientBuilder as AsyncClientBuilder, StatusCode};
use std::time::Duration;

/// Just namespace for http client functions
pub(crate) struct HttpClient;

impl HttpClient {
    pub(crate) async fn get_json<T: serde::de::DeserializeOwned>(
        url: &str,
        timeout_ms: i32,
    ) -> Result<T, HttpClientError> {
        let async_client =
            Self::build_async_client(timeout_ms).map_err(HttpClientError::make_network)?;

        let response = async_client
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

    /// Downloads file from remote server.
    ///
    /// Note! This method will fail if status_code != 200
    pub(crate) fn sync_get_file_contents(
        url: &String,
        timeout_ms: i32,
    ) -> Result<String, HttpClientError> {
        let client_builder =
            Self::build_blocking_client(timeout_ms).map_err(HttpClientError::make_network)?;

        let response = client_builder
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

    fn build_blocking_client(timeout_ms: i32) -> reqwest::Result<BlockingClient> {
        let mut blocking_client_builder = BlockingClientBuilder::new();

        if timeout_ms > 0 {
            blocking_client_builder =
                blocking_client_builder.timeout(Duration::from_millis(timeout_ms as u64));
        }

        blocking_client_builder.build()
    }

    fn build_async_client(timeout_ms: i32) -> reqwest::Result<AsyncClient> {
        let mut async_client_builder = AsyncClientBuilder::new();

        if timeout_ms > 0 {
            async_client_builder =
                async_client_builder.timeout(Duration::from_millis(timeout_ms as u64));
        }

        async_client_builder.build()
    }
}
