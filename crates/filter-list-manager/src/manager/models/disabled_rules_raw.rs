use crate::FilterId;

/// List of disabled rules separated by line breaks + filter id
pub struct DisabledRulesRaw {
    pub filter_id: FilterId,
    pub text: String,
}
