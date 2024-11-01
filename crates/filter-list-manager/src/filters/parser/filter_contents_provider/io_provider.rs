use super::FilterContentsProvider;
use crate::io::fetch_by_schemes::fetch_by_scheme;
use crate::io::get_scheme;
use crate::FilterParserError;

/// Used for downloading filters for filters parser purposes
/// Can download from remote servers (`https?:`) or from local machine, using `file:` scheme
pub(crate) struct IOProvider {
    request_timeout: Option<i32>,
}

impl IOProvider {
    pub(crate) const fn new() -> Self {
        Self {
            request_timeout: None,
        }
    }
}

impl FilterContentsProvider for IOProvider {
    fn get_filter_contents(&self, root_filter_url: &str) -> Result<String, FilterParserError> {
        let scheme = get_scheme(root_filter_url).unwrap_or_default();

        fetch_by_scheme(root_filter_url, scheme.into(), self.get_request_timeout())
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
