use crate::manager::models::FilterId;
use crate::storage::entities::localisation::FilterLangKey;

#[derive(Eq, PartialEq, Hash, Clone)]
pub(crate) struct FilterLocaleEntity {
    pub(crate) filter_id: FilterId,
    pub(crate) lang: FilterLangKey,
}
