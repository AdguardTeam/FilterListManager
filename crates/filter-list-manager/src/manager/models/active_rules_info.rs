//! Represents list of active (not disabled) rules with extra filter data
use crate::FilterId;

/// Represents list of active (not disabled) rules with extra filter data
pub struct ActiveRulesInfo {
    /// Filter id for these rules
    pub filter_id: FilterId,
    /// Group id of the filter
    pub group_id: i32,
    /// Is this filter trusted?
    pub is_trusted: bool,
    /// List of active rules.
    /// There rules is the difference between all rules of filter and disabled rules of filter
    pub rules: Vec<String>,
}
