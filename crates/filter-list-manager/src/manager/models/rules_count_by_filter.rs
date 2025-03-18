use crate::FilterId;

/// List of rules count + filter id
pub struct RulesCountByFilter {
    /// Associated filter id.
    pub filter_id: FilterId,
    /// Rules count in this filter list. Simply a number of non-empty lines
    /// and does not start with a comment marker. See [`RulesListService::is_line_is_rule`](`crate::storage::services::rules_list_service::RulesListService::is_line_is_rule`).
    pub rules_count: i32,
}
