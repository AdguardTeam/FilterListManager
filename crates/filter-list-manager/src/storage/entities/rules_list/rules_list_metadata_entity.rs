use crate::manager::models::FilterId;
use rusqlite::{Result, Row};

/// Lightweight metadata entity for streaming rules from rules_list table.
/// Contains only metadata fields — does NOT load rules_text or disabled_rules_text.
pub(crate) struct RulesListMetadataEntity {
    pub(crate) row_id: i64,
    pub(crate) filter_id: FilterId,
    pub(crate) has_directives: bool,
    pub(crate) integrity_signature: Option<String>,
}

impl RulesListMetadataEntity {
    pub(crate) fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            row_id: row.get(0)?,
            filter_id: row.get(1)?,
            has_directives: row.get(2)?,
            integrity_signature: row.get(3)?,
        })
    }
}
