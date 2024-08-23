use super::FilterContentsProvider;
use crate::FilterParserError;

/// Used for providing `root_filter` from string.
/// Note that includes in that provider will be resolved like in [`super::io_provider::IOProvider`]
pub(crate) struct StringProvider {
    filter_body: String,
    request_timeout: Option<i32>,
}

impl StringProvider {
    pub(crate) fn new(filter_body: String) -> Self {
        Self {
            filter_body,
            request_timeout: None,
        }
    }
}

impl FilterContentsProvider for StringProvider {
    fn get_filter_contents(&self, _: &String) -> Result<String, FilterParserError> {
        Ok(self.filter_body.clone())
    }

    fn get_request_timeout(&self) -> i32 {
        self.request_timeout.clone().unwrap_or_default()
    }

    fn set_request_timeout_once(&mut self, request_timeout: i32) {
        if self.request_timeout.is_none() {
            self.request_timeout = Some(request_timeout);
        }
    }
}
