use crate::FilterListType;

/// Build database filename from [`FilterListType`]
pub fn build_database_name_for_filter_list_type(t: FilterListType) -> String {
    format!("agflm_{}.db", t.to_name())
}
