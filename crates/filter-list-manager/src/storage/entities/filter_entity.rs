use crate::manager::models::FilterId;
use crate::CUSTOM_FILTERS_GROUP_ID;

/// Default value for [filter].[is_user_title] column.
pub(crate) const DEFAULT_IS_USER_TITLE_VALUE: bool = false;

/// Default value for [filter].[is_user_description] column.
pub(crate) const DEFAULT_IS_USER_DESCRIPTION_VALUE: bool = false;

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
    pub(in crate::storage) is_user_title: Option<bool>,
    pub(in crate::storage) is_user_description: Option<bool>,
}

impl FilterEntity {
    /// Filter is custom
    pub(crate) fn is_custom(&self) -> bool {
        self.group_id < 1
    }

    /// `is_user_title` getter with default value
    pub(crate) fn is_user_title(&self) -> bool {
        self.is_user_title.unwrap_or(DEFAULT_IS_USER_TITLE_VALUE)
    }

    /// `is_user_description` getter with default value
    pub(crate) fn is_user_description(&self) -> bool {
        self.is_user_description
            .unwrap_or(DEFAULT_IS_USER_DESCRIPTION_VALUE)
    }

    /// Sets `is_user_title` explicitly for changing in database
    pub(crate) fn set_is_user_title(&mut self, is_user_title: bool) {
        self.is_user_title = Some(is_user_title);
    }

    /// Sets `is_user_description` explicitly for changing in database
    pub(crate) fn set_is_user_description(&mut self, is_user_description: bool) {
        self.is_user_description = Some(is_user_description);
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
            is_user_title: None,
            is_user_description: None,
        }
    }
}
