use crate::manager::models::FilterId;

pub(crate) struct FilterLocalisationEntity {
    pub(crate) filter_id: FilterId,
    pub(crate) lang: String,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
}