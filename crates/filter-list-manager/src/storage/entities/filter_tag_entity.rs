use crate::manager::models::filter_tag::FilterTag;
use rusqlite::{Result, Row};
use serde::Deserialize;

use super::hydrate::Hydrate;

/// Entity for filter_tag table
#[derive(Eq, PartialEq, Hash, Clone, Debug, Deserialize)]
pub(crate) struct FilterTagEntity {
    #[serde(alias = "tagId")]
    pub(crate) tag_id: i32,
    pub(crate) keyword: String,
}

impl Hydrate for FilterTagEntity {
    fn hydrate(row: &Row) -> Result<FilterTagEntity> {
        Ok(FilterTagEntity {
            tag_id: row.get(0)?,
            keyword: row.get(1)?,
        })
    }
}

impl From<FilterTagEntity> for FilterTag {
    fn from(value: FilterTagEntity) -> Self {
        FilterTag {
            id: value.tag_id,
            keyword: value.keyword,
        }
    }
}
