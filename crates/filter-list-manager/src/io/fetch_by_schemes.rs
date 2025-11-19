use crate::filters::parser::parser_error::FilterParserError;
use crate::io::content_checkers::{check_contents_is_filter_contents, is_likely_media};
use crate::io::http::blocking_client::BlockingClient;
use crate::io::{
    read_binary_by_url, read_file_by_url, url_schemes::UrlSchemes, ReadFilterFileError,
};
use crate::{FLMError, FLMResult, HttpClientError, IOError};
use bytes::Bytes;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

#[derive(Eq, PartialEq)]
pub(crate) enum FilterFetchPolicy {
    /// Regular filter.
    /// Should be requested only if 200 status code returned
    RegularFilter,

    /// Content is unavailable yet
    /// See: https://github.com/ameshkov/diffupdates?tab=readme-ov-file#1-check-for-update
    DiffUpdates,
}

/// Synchronously fetch filter contents from absolute url, and makes content check
pub(crate) fn fetch_filter_by_scheme_with_content_check(
    absolute_url: &str,
    scheme: UrlSchemes,
    shared_http_client: &BlockingClient,
    fetch_policy: FilterFetchPolicy,
) -> Result<String, FilterParserError> {
    let contents = match scheme {
        UrlSchemes::File => {
            let result = read_binary_by_url(absolute_url);

            if fetch_policy == FilterFetchPolicy::DiffUpdates {
                if matches!(result, Err(ReadFilterFileError::Io(IOError::NotFound(_))))
                    || result.as_ref().is_ok_and(|vec| vec.is_empty())
                {
                    return Err(FilterParserError::NoContent);
                }
            }

            result.map(Bytes::from).map_err(Into::into)
        }

        UrlSchemes::Https | UrlSchemes::Http => {
            let (bytes, status) = shared_http_client
                .get_filter_bytes(absolute_url)
                .map_err(FilterParserError::Network)?;

            if fetch_policy == FilterFetchPolicy::DiffUpdates {
                if matches!(status, StatusCode::NOT_FOUND | StatusCode::NO_CONTENT)
                    || status == StatusCode::OK && bytes.is_empty()
                {
                    return Err(FilterParserError::NoContent);
                }

                if status.is_client_error() || status.is_server_error() {
                    return Err(FilterParserError::Network(HttpClientError::NetworkError(
                        format!(
                            "Got HTTP status code \"{}\" while requesting filter",
                            status
                        ),
                    )));
                }
            } else {
                // Regular filter policy
                if status != StatusCode::OK {
                    return Err(FilterParserError::Network(
                        HttpClientError::make_only_200_strict(status, absolute_url.to_owned()),
                    ));
                }
            }

            Ok(bytes)
        }
        _ => Err(FilterParserError::SchemeIsIncorrect(format!(
            "Got unknown scheme from {}",
            absolute_url
        ))),
    }?;

    if is_likely_media(contents.as_ref()) {
        return Err(FilterParserError::FilterContentIsLikelyNotAFilter);
    }

    let string =
        String::from_utf8(contents.to_vec()).map_err(FilterParserError::other_from_to_string)?;

    check_contents_is_filter_contents(&string)?;

    Ok(string)
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
