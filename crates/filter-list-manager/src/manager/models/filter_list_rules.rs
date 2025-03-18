//! Filter list rules container.
use crate::FilterId;

/// FilterListRules represents filter list rules. Note, that we should store
/// both the rules and disabled rules. This is required to be able to update the
/// rules without caring about individual rule status. I.e. if you once
/// disabled `||example.org^` it should stay disabled even the new version of
/// the list has new lines with the same rule.
#[derive(Debug, Clone)]
pub struct FilterListRules {
    /// Associated filter id.
    pub filter_id: FilterId,
    /// List of all rules in the filter list.
    pub rules: Vec<String>,
    /// List of only disabled rules.
    pub disabled_rules: Vec<String>,
    /// Rules count in this filter list. Simply a number of non-empty lines
    /// and does not start with a comment marker.
    pub rules_count: i32,
}
