use crate::manager::models::FilterId;
use rusqlite::{Result, Row};

/// Lightweight metadata entity for filter_includes table.
/// Contains only metadata fields — does NOT load body.
pub(crate) struct FilterIncludeMetadataEntity {
    pub(crate) row_id: i64,
    pub(crate) filter_id: FilterId,
    pub(crate) absolute_url: String,
    pub(crate) integrity_signature: Option<String>,
}

impl FilterIncludeMetadataEntity {
    pub(crate) fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            row_id: row.get(0)?,
            filter_id: row.get(1)?,
            absolute_url: row.get(2)?,
            integrity_signature: row.get(3)?,
        })
    }
}
