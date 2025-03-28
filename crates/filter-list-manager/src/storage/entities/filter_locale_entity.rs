use rusqlite::{Result, Row};

use crate::manager::models::FilterId;
use crate::storage::entities::localisation::FilterLangKey;

use super::hydrate::Hydrate;

/// Entity for filter_locale table
#[derive(Eq, PartialEq, Hash, Clone)]
pub(crate) struct FilterLocaleEntity {
    pub(crate) filter_id: FilterId,
    pub(crate) lang: FilterLangKey,
}

impl Hydrate for FilterLocaleEntity {
    fn hydrate(row: &Row) -> Result<FilterLocaleEntity> {
        Ok(FilterLocaleEntity {
            filter_id: row.get(0)?,
            lang: row.get(1)?,
        })
    }
}
