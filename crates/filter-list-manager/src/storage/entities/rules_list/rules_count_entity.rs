use rusqlite::{Result, Row};

use crate::manager::models::rules_count_by_filter::RulesCountByFilter;
use crate::manager::models::FilterId;
use crate::storage::entities::hydrate::Hydrate;

/// Rules count entity
#[derive(Clone)]
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub(crate) struct RulesCountEntity {
    pub(crate) filter_id: FilterId,
    pub(crate) rules_count: i32,
}

impl Hydrate for RulesCountEntity {
    fn hydrate(row: &Row) -> Result<RulesCountEntity> {
        Ok(RulesCountEntity {
            filter_id: row.get(0)?,
            rules_count: row.get(1)?,
        })
    }
}

impl From<RulesCountEntity> for RulesCountByFilter {
    fn from(value: RulesCountEntity) -> RulesCountByFilter {
        RulesCountByFilter {
            filter_id: value.filter_id,
            rules_count: value.rules_count,
        }
    }
}
