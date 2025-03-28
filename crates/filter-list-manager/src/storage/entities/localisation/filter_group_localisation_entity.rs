use crate::storage::entities::localisation::FilterLangKey;

/// Entity for filter_group_localisation table
pub(crate) struct FilterGroupLocalisationEntity {
    pub(crate) group_id: i32,
    pub(crate) lang: FilterLangKey,
    pub(crate) name: Option<String>,
}
