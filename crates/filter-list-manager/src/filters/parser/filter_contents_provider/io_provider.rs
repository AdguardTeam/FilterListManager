use super::FilterContentsProvider;
use crate::io::fetch_by_schemes::fetch_by_scheme_with_content_check;
use crate::io::get_scheme;
use crate::io::http::blocking_client::BlockingClient;
use crate::FilterParserError;

/// Used for downloading filters for filters parser purposes
/// Can download from remote servers (`https?:`) or from local machine, using `file:` scheme
pub(crate) struct IOProvider<'a> {
    /// Shared sync http client
    shared_http_client: &'a BlockingClient,
}

impl<'a> IOProvider<'a> {
    pub(crate) const fn new(shared_http_client: &'a BlockingClient) -> Self {
        Self { shared_http_client }
    }
}

impl FilterContentsProvider for IOProvider<'_> {
    fn get_filter_contents(&self, root_filter_url: &str) -> Result<String, FilterParserError> {
        let scheme = get_scheme(root_filter_url).unwrap_or_default();

        fetch_by_scheme_with_content_check(root_filter_url, scheme.into(), self.get_http_client())
    }

    fn get_http_client(&self) -> &BlockingClient {
        self.shared_http_client
    }
}
