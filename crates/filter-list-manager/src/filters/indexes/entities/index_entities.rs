#![allow(non_snake_case)]

use crate::manager::models::FilterId;
use chrono::Utc;
use serde::Deserialize;

use crate::storage::entities::{
    filter_entity::FilterEntity, filter_filter_tag_entity::FilterFilterTagEntity,
    filter_locale_entity::FilterLocaleEntity,
};

/// Filter representation from index
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Clone))]
pub(crate) struct FilterIndexEntity {
    pub(crate) filterId: FilterId,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) homepage: String,
    pub(crate) expires: i32,
    pub(crate) displayNumber: i32,
    pub(crate) groupId: i32,
    pub(crate) downloadUrl: String,
    pub(crate) subscriptionUrl: String,
    pub(crate) deprecated: bool,
    pub(crate) version: String,
    pub(crate) timeUpdated: chrono::DateTime<Utc>,
    pub(crate) languages: Vec<String>,
    pub(crate) tags: Vec<i32>,
}

impl FilterIndexEntity {
    #[allow(clippy::field_reassign_with_default)]
    /// Transforms index filter entity to storage entities
    pub(crate) fn into_storage_entities(self) -> FilterIndexEntityComponents {
        let mut filter_list = FilterEntity::default();

        filter_list.filter_id = Some(self.filterId);
        filter_list.title = self.name;
        filter_list.group_id = self.groupId;
        filter_list.description = self.description;
        filter_list.last_update_time = self.timeUpdated.timestamp();
        filter_list.download_url = self.downloadUrl;
        filter_list.subscription_url = self.subscriptionUrl;
        filter_list.version = self.version;
        filter_list.display_number = self.displayNumber;
        filter_list.is_trusted = true;
        filter_list.expires = self.expires;
        filter_list.homepage = self.homepage;

        FilterIndexEntityComponents {
            filter: filter_list,
            locales: self
                .languages
                .into_iter()
                .map(|locale| FilterLocaleEntity {
                    filter_id: self.filterId,
                    lang: locale,
                })
                .collect(),
            tags: self
                .tags
                .into_iter()
                .map(|tag_id| FilterFilterTagEntity {
                    filter_id: self.filterId,
                    tag_id,
                })
                .collect(),
        }
    }
}

/// Container helper for transformation index filter entity into storage filter entity
pub(crate) struct FilterIndexEntityComponents {
    pub(crate) filter: FilterEntity,
    pub(crate) locales: Vec<FilterLocaleEntity>,
    pub(crate) tags: Vec<FilterFilterTagEntity>,
}

#[cfg(test)]
mod tests {
    use crate::filters::indexes::entities::IndexEntity;

    #[test]
    fn test_that_deserialization_allow_unknown_fields() {
        let json_string = include_str!("../../../../tests/fixtures/filters.json");
        let index = serde_json::from_str::<IndexEntity>(json_string).unwrap();

        // We have trust level in our json, but not fail
        assert!(json_string.contains("\"trustLevel\": \"low\""));
        assert!(index.filters.len() > 0);
    }
}
