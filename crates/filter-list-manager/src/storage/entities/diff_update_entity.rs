use rusqlite::{Result, Row};

use crate::FilterId;

use super::hydrate::Hydrate;

/// Entity for diff_updates table
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct DiffUpdateEntity {
    /// Related filter entity id
    pub(crate) filter_id: FilterId,
    /// Next patch path
    pub(crate) next_path: String,
    /// The time, when we should go after patch contents
    pub(crate) next_check_time: i64,
}

impl Hydrate for DiffUpdateEntity {
    fn hydrate(row: &Row) -> Result<DiffUpdateEntity> {
        Ok(DiffUpdateEntity {
            filter_id: row.get(0)?,
            next_path: row.get(1)?,
            next_check_time: row.get(2)?,
        })
    }
}
