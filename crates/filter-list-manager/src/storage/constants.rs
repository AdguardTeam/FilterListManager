//! Module of constants related to the peculiarities of storing filters and their metadata in the repository.
use crate::FilterId;

/// Filter ID for special filter for user rules
pub const USER_RULES_FILTER_LIST_ID: FilterId = i32::MIN;

/// Group ID for custom filters
pub const CUSTOM_FILTERS_GROUP_ID: i32 = i32::MIN;

/// Service group id for special filters
pub const SERVICE_GROUP_ID: i32 = 0;

/// Custom filters ids must be in range
pub const MAXIMUM_CUSTOM_FILTER_ID: FilterId = -10000;

/// Custom filters ids must be in range
pub const MINIMUM_CUSTOM_FILTER_ID: FilterId = -1_000_000_000;

/// Smallest possible filter id value. -2^53
/// You can safely occupy any filter with an id lower than this number.
/// The library is guaranteed to never create a filter with this id.
pub const SMALLEST_POSSIBLE_FILTER_ID: FilterId = -2_000_000_000;

/// Database filename for [`crate::FilterListType::STANDARD`]
pub const STANDARD_FILTERS_DATABASE_FILENAME: &str = "agflm_standard.db";
/// Database filename for [`crate::FilterListType::DNS`]
pub const DNS_FILTERS_DATABASE_FILENAME: &str = "agflm_dns.db";

#[cfg(test)]
mod tests {
    use crate::{MINIMUM_CUSTOM_FILTER_ID, SMALLEST_POSSIBLE_FILTER_ID};

    #[test]
    fn test_smallest_possible_filter_id_must_be_a_smallest() {
        assert!(SMALLEST_POSSIBLE_FILTER_ID < MINIMUM_CUSTOM_FILTER_ID)
    }
}
