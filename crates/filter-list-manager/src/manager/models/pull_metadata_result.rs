//! Models about index metadata update process

use crate::FilterId;

/// Result of index metadata update
pub struct PullMetadataResult {
    /// List of filters added in the update
    pub added_filters: Vec<FilterId>,
    /// List of filters removed in the update
    pub removed_filters: Vec<FilterId>,
    /// List of filters moved in the update
    pub moved_filters: Vec<MovedFilterInfo>,
}

impl PullMetadataResult {
    /// Creates a new empty `PullMetadataResult`
    pub const fn new() -> Self {
        Self {
            added_filters: vec![],
            removed_filters: vec![],
            moved_filters: vec![],
        }
    }

    /// Creates a new `PullMetadataResult` with the given `added_filters`
    ///
    /// The `removed_filters` and `moved_filters` fields are initialized as empty vectors.
    ///
    pub fn new_with_added_filters(added_filters: Vec<FilterId>) -> Self {
        Self {
            added_filters,
            removed_filters: vec![],
            moved_filters: vec![],
        }
    }
}

/// Information about filter movement during index metadata update
pub struct MovedFilterInfo {
    /// Previous id of the filter
    pub previous_id: FilterId,
    /// New id of the filter
    pub new_id: FilterId,
}

impl MovedFilterInfo {
    /// Creates a new `MovedFilterInfo` with the given `previous_id` and `new_id`
    ///
    /// # Arguments
    ///
    /// * `previous_id` - Previous id of the filter
    /// * `new_id` - New id of the filter
    ///
    /// # Returns
    ///
    /// A new `MovedFilterInfo`
    pub(crate) fn new(previous_id: FilterId, new_id: FilterId) -> Self {
        Self {
            previous_id,
            new_id,
        }
    }
}
