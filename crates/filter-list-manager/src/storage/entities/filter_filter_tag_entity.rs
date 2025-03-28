use rusqlite::{Result, Row};

use crate::manager::models::FilterId;

use super::hydrate::Hydrate;

/// Filter Tag relation entity
pub(crate) struct FilterFilterTagEntity {
    pub(crate) filter_id: FilterId,
    pub(crate) tag_id: i32,
}

impl Hydrate for FilterFilterTagEntity {
    fn hydrate(row: &Row) -> Result<FilterFilterTagEntity> {
        Ok(FilterFilterTagEntity {
            tag_id: row.get(0)?,
            filter_id: row.get(1)?,
        })
    }
}
