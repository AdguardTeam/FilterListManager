use crate::storage::entities::localisation::FilterLangKey;

pub(crate) struct FilterGroupLocalisationEntity {
    pub(crate) group_id: i32,
    pub(crate) lang: FilterLangKey,
    pub(crate) name: Option<String>,
}
