//! [`FilterListRules`]-counterpart represents `rules` and `disabled rules` as strings.
use crate::FilterId;

/// This contains `rules` and `disabled_rules` just as strings instead of vectors in the base type.
/// See the [`FilterListRules`] for more info.
#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct FilterListRulesRaw {
    /// Associated filter id.
    pub filter_id: FilterId,
    /// List of all rules in the filter list.
    pub rules: String,
    /// List of only disabled rules.
    pub disabled_rules: String,
    /// Rules count in this filter list. Simply a number of non-empty lines
    /// and does not start with a comment marker.
    pub rules_count: i32,
}
