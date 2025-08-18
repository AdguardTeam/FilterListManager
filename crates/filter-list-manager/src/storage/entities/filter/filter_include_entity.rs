use crate::storage::entities::hydrate::Hydrate;
use crate::FilterId;
use rusqlite::Row;

/// This entity represents an include from main filter text.
/// Recursive includes were resolved due to compilation
#[derive(Clone)]
pub(crate) struct FilterIncludeEntity {
    pub(crate) row_id: Option<i64>,
    pub(crate) filter_id: FilterId,
    pub(crate) absolute_url: String,
    pub(crate) body: String,
    pub(crate) rules_count: i32,
}

impl Hydrate for FilterIncludeEntity {
    fn hydrate(row: &Row) -> rusqlite::Result<Self> {
        Ok(FilterIncludeEntity {
            row_id: row.get(0)?,
            filter_id: row.get(1)?,
            absolute_url: row.get(2)?,
            body: row.get(3)?,
            rules_count: row.get(4)?,
        })
    }
}
