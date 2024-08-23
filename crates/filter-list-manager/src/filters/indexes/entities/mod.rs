pub(crate) mod index_entities;
pub(crate) mod index_localisation_entities;

use crate::filters::indexes::entities::index_localisation_entities::{
    FilterLanguageMeta, GroupLanguageMeta, TagLanguageMeta,
};
use crate::manager::models::FilterId;
use crate::storage::entities::localisation::filter_group_localisation_entity::FilterGroupLocalisationEntity;
use crate::storage::entities::localisation::filter_localisation_entity::FilterLocalisationEntity;
use crate::storage::entities::localisation::filter_tag_localisation_entity::FilterTagLocalisationEntity;
use crate::storage::entities::{
    filter_group_entity::FilterGroupEntity, filter_tag_entity::FilterTagEntity,
};
use crate::{FLMError, FLMResult};
use index_entities::FilterIndexEntity;
use serde::Deserialize;
use std::collections::HashMap;

/// Alpha-2 language code for filter specific languages
type FilterLanguageCode = String;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Clone))]
pub(crate) struct IndexEntity {
    pub(crate) groups: Vec<FilterGroupEntity>,
    pub(crate) tags: Vec<FilterTagEntity>,
    pub(crate) filters: Vec<FilterIndexEntity>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Clone))]
pub(crate) struct IndexI18NEntity {
    pub(crate) groups: HashMap<String, HashMap<FilterLanguageCode, GroupLanguageMeta>>,
    pub(crate) tags: HashMap<String, HashMap<FilterLanguageCode, TagLanguageMeta>>,
    pub(crate) filters: HashMap<String, HashMap<FilterLanguageCode, FilterLanguageMeta>>,
}

impl IndexI18NEntity {
    /// Exchanges `this` object onto storage entities
    ///
    /// # Failure
    ///
    /// This function panics if problem will be encountered
    pub(in crate::filters::indexes) fn exchange(
        self,
    ) -> FLMResult<(
        Vec<FilterGroupLocalisationEntity>,
        Vec<FilterTagLocalisationEntity>,
        Vec<FilterLocalisationEntity>,
    )> {
        let mut group_vec: Vec<FilterGroupLocalisationEntity> = vec![];
        for (group_id, lang_map) in self.groups {
            for (language_code, meta) in lang_map {
                group_vec.push(FilterGroupLocalisationEntity {
                    group_id: group_id.parse::<i32>().map_err(FLMError::from_display)?,
                    lang: language_code,
                    name: meta.name,
                });
            }
        }

        let mut tags_vec: Vec<FilterTagLocalisationEntity> = vec![];
        for (tag_id, lang_map) in self.tags {
            for (language_code, meta) in lang_map {
                tags_vec.push(FilterTagLocalisationEntity {
                    tag_id: tag_id.parse::<i32>().map_err(FLMError::from_display)?,
                    lang: language_code,
                    name: meta.name,
                    description: meta.description,
                });
            }
        }

        let mut filters_vec: Vec<FilterLocalisationEntity> = vec![];
        for (filter_id, lang_map) in self.filters {
            for (language_code, meta) in lang_map {
                filters_vec.push(FilterLocalisationEntity {
                    filter_id: filter_id
                        .parse::<FilterId>()
                        .map_err(FLMError::from_display)?,
                    lang: language_code,
                    name: meta.name,
                    description: meta.description,
                });
            }
        }

        Ok((group_vec, tags_vec, filters_vec))
    }
}
