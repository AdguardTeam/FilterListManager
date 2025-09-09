use crate::io::fetch_by_schemes::fetch_by_scheme_with_content_check;
use crate::io::url_schemes::UrlSchemes;
use crate::FilterParserError;

pub(crate) mod diff_path_provider;
pub(in crate::filters) mod io_provider;
pub(crate) mod string_provider;

use crate::io::http::blocking_client::BlockingClient;

/// Provides filters contents.
/// It can provide filter by `root_filter_url` and resolves its includes
pub(crate) trait FilterContentsProvider {
    /// Get root filter contents by root filter url.
    /// It can be only absolute url and do not need a parent_url
    fn get_filter_contents(&self, root_filter_url: &str) -> Result<String, FilterParserError>;

    /// Get included filter contents
    fn get_included_filter_contents(
        &self,
        absolute_url: &str,
        scheme: UrlSchemes,
    ) -> Result<String, FilterParserError> {
        fetch_by_scheme_with_content_check(absolute_url, scheme, self.get_http_client())
    }

    /// Gets blocking client. Every provider needs it
    fn get_http_client(&self) -> &BlockingClient;
}
