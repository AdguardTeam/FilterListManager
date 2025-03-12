use crate::FilterId;

/// List of rules count + filter id
pub struct RulesCountByFilter {
    /// Associated filter id.
    pub filter_id: FilterId,
    /// Rules count in this filter list. Simply a number of non-empty lines
    /// and does not start with a comment marker.
    pub rules_count: i32,
}
