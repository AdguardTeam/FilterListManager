use crate::FilterId;

/// List of disabled rules separated by line breaks + filter id
pub struct DisabledRulesRaw {
    /// Associated filter id.
    pub filter_id: FilterId,
    /// List of only disabled rules as string.
    pub text: String,
}
