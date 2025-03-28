use rusqlite::{Result, Row};

use crate::manager::models::disabled_rules_raw::DisabledRulesRaw;
use crate::manager::models::FilterId;
use crate::storage::entities::hydrate::Hydrate;

/// Disabled rules entity
#[derive(Clone)]
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub(crate) struct DisabledRulesEntity {
    pub(crate) filter_id: FilterId,
    pub(crate) disabled_text: String,
}

impl Hydrate for DisabledRulesEntity {
    fn hydrate(row: &Row) -> Result<DisabledRulesEntity> {
        Ok(DisabledRulesEntity {
            filter_id: row.get(0)?,
            disabled_text: row.get(1)?,
        })
    }
}

impl From<DisabledRulesEntity> for DisabledRulesRaw {
    fn from(value: DisabledRulesEntity) -> DisabledRulesRaw {
        DisabledRulesRaw {
            filter_id: value.filter_id,
            text: value.disabled_text,
        }
    }
}
