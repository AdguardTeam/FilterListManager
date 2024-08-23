use crate::manager::models::filter_group::FilterGroup;
use serde::Deserialize;

#[derive(Eq, PartialEq, Hash, Clone, Debug, Deserialize)]
pub struct FilterGroupEntity {
    #[serde(alias = "groupId")]
    pub group_id: i32,
    #[serde(alias = "groupName")]
    pub name: String,
    #[serde(alias = "displayNumber")]
    pub display_number: i32,
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
