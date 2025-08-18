//! Represents list of active (not disabled) rules with extra filter data
use crate::FilterId;

/// Represents list of active (not disabled) rules with extra filter data as string
pub struct ActiveRulesInfoRaw {
    /// Filter id for these rules
    pub filter_id: FilterId,
    /// Group id of the filter
    pub group_id: i32,
    /// Is this filter trusted?
    pub is_trusted: bool,
    /// List of active rules as string separated by \n.
    /// There rules is the difference between all rules of filter and disabled rules of filter
    pub rules: String,
}
