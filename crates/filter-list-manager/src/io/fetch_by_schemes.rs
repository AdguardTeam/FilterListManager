use crate::filters::parser::parser_error::FilterParserError;
use crate::io::http::blocking_client::BlockingClient;
use crate::io::{read_file_by_url, url_schemes::UrlSchemes};
use crate::{FLMError, FLMResult};
use serde::de::DeserializeOwned;

/// Synchronously fetch contents from absolute url
pub(crate) fn fetch_by_scheme(
    absolute_url: &str,
    scheme: UrlSchemes,
    shared_http_client: &BlockingClient,
) -> Result<String, FilterParserError> {
    match scheme {
        UrlSchemes::File => read_file_by_url(absolute_url).map_err(Into::into),
        UrlSchemes::Https | UrlSchemes::Http => shared_http_client
            .get_filter(absolute_url)
            .map_err(FilterParserError::Network),
        _ => Err(FilterParserError::SchemeIsIncorrect(format!(
            "Got unknown scheme from {}",
            absolute_url
        ))),
    }
}

/// Fetches json by scheme from url
pub(crate) fn fetch_json_by_scheme<T>(
    absolute_url: &str,
    scheme: UrlSchemes,
    shared_http_client: &BlockingClient,
) -> FLMResult<T>
where
    T: DeserializeOwned,
{
    match scheme {
        UrlSchemes::File => {
            let contents = read_file_by_url(absolute_url).map_err(FLMError::from)?;

            serde_json::from_str::<T>(contents.as_str()).map_err(FLMError::from_display)
        }
        UrlSchemes::Https | UrlSchemes::Http => shared_http_client
            .get_json::<T>(absolute_url)
            .map_err(FLMError::Network),
        _ => FLMError::make_err(format!("Got unknown scheme from {}", absolute_url)),
    }
}
