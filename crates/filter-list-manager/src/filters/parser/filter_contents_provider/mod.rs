use crate::io::fetch_by_schemes::fetch_by_scheme;
use crate::io::url_schemes::UrlSchemes;
use crate::FilterParserError;

mod check_contents_is_filter_contents;
pub(crate) mod diff_path_provider;
pub(super) mod io_provider;
pub(crate) mod string_provider;
use check_contents_is_filter_contents::check_contents_is_filter_contents;

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
        fetch_by_scheme(absolute_url, scheme, self.get_request_timeout())
    }

    /// Value getter
    fn get_request_timeout(&self) -> i32;

    /// Sets request timeout if it is not set
    fn set_request_timeout_once(&mut self, request_timeout: i32);

    /// Tries to check the filter data to see if it is a filter or something else, using some heuristics
    fn pre_check_filter_contents(&self, filter_contents: &str) -> Result<(), FilterParserError> {
        check_contents_is_filter_contents(filter_contents)
    }
}
