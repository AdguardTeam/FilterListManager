use crate::manager::models::FilterId;
use crate::CUSTOM_FILTERS_GROUP_ID;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct FilterEntity {
    pub filter_id: Option<FilterId>,
    pub title: String,
    pub group_id: i32,
    pub description: String,
    pub last_update_time: i64,
    pub last_download_time: i64,
    pub download_url: String,
    pub subscription_url: String,
    pub version: String,
    pub display_number: i32,
    pub expires: i32,
    pub homepage: String,
    pub license: String,
    pub checksum: String,
    pub is_enabled: bool,
    pub is_installed: bool,
    pub is_trusted: bool,
}

impl FilterEntity {
    /// Filter is custom
    pub(crate) fn is_custom(&self) -> bool {
        self.group_id < 1
    }
}

impl Default for FilterEntity {
    fn default() -> Self {
        FilterEntity {
            filter_id: None,
            title: String::new(),
            // By default, filter is custom
            group_id: CUSTOM_FILTERS_GROUP_ID,
            description: String::new(),
            last_update_time: 0i64,
            last_download_time: 0i64,
            download_url: String::new(),
            subscription_url: String::new(),
            is_enabled: false,
            version: String::new(),
            display_number: 0,
            is_trusted: false,
            expires: 0,
            homepage: String::new(),
            license: String::new(),
            checksum: String::new(),
            is_installed: false,
        }
    }
}
