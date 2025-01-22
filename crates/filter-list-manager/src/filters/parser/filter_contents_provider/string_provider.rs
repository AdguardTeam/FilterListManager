use super::FilterContentsProvider;
use crate::io::http::blocking_client::BlockingClient;
use crate::FilterParserError;

/// Used for providing `root_filter` from string.
/// Note that includes in that provider will be resolved like in [`super::io_provider::IOProvider`]
pub(crate) struct StringProvider<'a> {
    /// Provided filter's body
    filter_body: String,
    /// Shared sync http client
    shared_http_client: &'a BlockingClient,
}

impl<'a> StringProvider<'a> {
    pub(crate) fn new(filter_body: String, shared_http_client: &'a BlockingClient) -> Self {
        Self {
            filter_body,
            shared_http_client,
        }
    }
}

impl FilterContentsProvider for StringProvider<'_> {
    fn get_filter_contents(&self, _: &str) -> Result<String, FilterParserError> {
        Ok(self.filter_body.clone())
    }

    fn get_http_client(&self) -> &BlockingClient {
        self.shared_http_client
    }
}

#[cfg(test)]
impl StringProvider<'_> {
    pub(crate) fn factory_test(body: String) -> Self {
        Self {
            filter_body: body,
            shared_http_client: &crate::test_utils::SHARED_TEST_BLOCKING_HTTP_CLIENT,
        }
    }
}
