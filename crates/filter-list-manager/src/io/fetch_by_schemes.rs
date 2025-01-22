use crate::filters::parser::parser_error::FilterParserError;
use crate::io::http::blocking_client::BlockingClient;
use crate::io::{read_file_by_url, url_schemes::UrlSchemes};

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
