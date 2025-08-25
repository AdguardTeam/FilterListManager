use crate::storage::entities::hydrate::Hydrate;
use crate::FilterId;
use rusqlite::Row;
use std::ops::Not;

/// This entity represents an include from main filter text.
/// Recursive includes were resolved due to compilation
#[derive(Clone)]
pub(crate) struct FilterIncludeEntity {
    pub(crate) row_id: Option<i64>,
    pub(crate) filter_id: FilterId,
    pub(crate) absolute_url: String,
    pub(crate) body: String,
    pub(crate) rules_count: i32,
    pub(crate) body_hash: Option<String>,
}

impl FilterIncludeEntity {
    /// Makes new entity
    pub(crate) fn make(
        filter_id: FilterId,
        absolute_url: String,
        rules_count: i32,
        body: String,
    ) -> Self {
        FilterIncludeEntity {
            row_id: None,
            filter_id,
            absolute_url,
            body_hash: body
                .is_empty()
                .not()
                .then(|| blake3::hash(body.as_bytes()).to_string()),
            body,
            rules_count,
        }
    }

    pub(crate) fn get_body_hash(&self) -> Option<&String> {
        self.body_hash.as_ref()
    }
}

impl Hydrate for FilterIncludeEntity {
    fn hydrate(row: &Row) -> rusqlite::Result<Self> {
        Ok(FilterIncludeEntity {
            row_id: row.get(0)?,
            filter_id: row.get(1)?,
            absolute_url: row.get(2)?,
            body: row.get(3)?,
            rules_count: row.get(4)?,
            body_hash: row.get(5)?,
        })
    }
}
