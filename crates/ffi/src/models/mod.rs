use adguard_flm::{
    FilterId, CUSTOM_FILTERS_GROUP_ID, SERVICE_GROUP_ID, SMALLEST_POSSIBLE_FILTER_ID,
    USER_RULES_FILTER_LIST_ID,
};

/// Structure used for passing constants through FFI
pub struct FilterListManagerConstants {
    /// Filter ID for *User rules* filter
    pub user_rules_id: FilterId,
    /// Group ID for special *custom filters group*
    pub custom_group_id: i32,
    /// Group ID for *special service filters*
    pub special_group_id: i32,
    /// Smallest possible filter_id. You can safely occupy any filter with an id lower than this number.
    /// The library is guaranteed to never create a filter with this id
    pub smallest_filter_id: FilterId,
}

impl Default for FilterListManagerConstants {
    fn default() -> Self {
        FilterListManagerConstants {
            user_rules_id: USER_RULES_FILTER_LIST_ID,
            custom_group_id: CUSTOM_FILTERS_GROUP_ID,
            special_group_id: SERVICE_GROUP_ID,
            smallest_filter_id: SMALLEST_POSSIBLE_FILTER_ID,
        }
    }
}
