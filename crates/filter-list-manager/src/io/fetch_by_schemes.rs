use crate::filters::parser::parser_error::FilterParserError;
use crate::io::{http::HttpClient, read_filter_file, url_schemes::UrlSchemes};

/// Synchronously fetch contents from absolute url
pub(crate) fn fetch_by_scheme(
    absolute_url: &str,
    scheme: UrlSchemes,
    request_timeout: i32,
) -> Result<String, FilterParserError> {
    match scheme {
        UrlSchemes::File => read_filter_file(absolute_url).map_err(Into::into),
        UrlSchemes::Https | UrlSchemes::Http => {
            HttpClient::sync_get_file_contents(absolute_url, request_timeout)
                .map_err(FilterParserError::Network)
        }
        _ => Err(FilterParserError::SchemeIsIncorrect(format!(
            "Got unknown scheme from {}",
            absolute_url
        ))),
    }
}
