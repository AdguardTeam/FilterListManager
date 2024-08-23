use crate::manager::models::filter_tag::FilterTag;
use serde::Deserialize;

#[derive(Eq, PartialEq, Hash, Clone, Debug, Deserialize)]
pub(crate) struct FilterTagEntity {
    #[serde(alias = "tagId")]
    pub(crate) tag_id: i32,
    pub(crate) keyword: String,
}

impl From<FilterTagEntity> for FilterTag {
    fn from(value: FilterTagEntity) -> Self {
        FilterTag {
            id: value.tag_id,
            keyword: value.keyword,
        }
    }
}
