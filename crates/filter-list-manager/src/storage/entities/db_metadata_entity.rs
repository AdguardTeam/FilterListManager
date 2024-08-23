use crate::{FilterId, MAXIMUM_CUSTOM_FILTER_ID};

pub(crate) struct DBMetadataEntity {
    /// Database version
    pub(crate) version: i32,
    /// Last autoincrement value for custom filters
    /// Value between [`crate::MINIMUM_CUSTOM_FILTER_ID`] and [`crate::MAXIMUM_CUSTOM_FILTER_ID`]
    pub(crate) custom_filters_autoincrement_value: FilterId,
}

impl Default for DBMetadataEntity {
    fn default() -> Self {
        DBMetadataEntity {
            version: 0,
            custom_filters_autoincrement_value: MAXIMUM_CUSTOM_FILTER_ID,
        }
    }
}
