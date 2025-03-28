use crate::manager::models::filter_group::FilterGroup;
use rusqlite::{Result, Row};
use serde::Deserialize;

use super::hydrate::Hydrate;

/// Entity for filter_group table
#[derive(Eq, PartialEq, Hash, Clone, Debug, Deserialize)]
pub struct FilterGroupEntity {
    #[serde(alias = "groupId")]
    pub group_id: i32,
    #[serde(alias = "groupName")]
    pub name: String,
    #[serde(alias = "displayNumber")]
    pub display_number: i32,
}

impl Hydrate for FilterGroupEntity {
    fn hydrate(row: &Row) -> Result<FilterGroupEntity> {
        Ok(FilterGroupEntity {
            group_id: row.get(0)?,
            name: row.get(1)?,
            display_number: row.get(2)?,
        })
    }
}

impl From<FilterGroupEntity> for FilterGroup {
    fn from(value: FilterGroupEntity) -> Self {
        FilterGroup {
            id: value.group_id,
            name: value.name,
            display_number: value.display_number,
        }
    }
}
