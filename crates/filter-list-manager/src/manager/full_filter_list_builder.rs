use crate::manager::models::filter_list_rules::FilterListRules;
use crate::manager::models::full_filter_list::FullFilterList;
use crate::storage::entities::filter_entity::FilterEntity;
use crate::storage::repositories::filter_locale_repository::FilterLocaleRepository;
use crate::storage::repositories::filter_tag_repository::FilterTagRepository;
use crate::storage::repositories::rules_list_repository::{
    MapFilterIdOnRulesList, RulesListRepository,
};
use crate::{FLMError, FLMResult};
use rusqlite::Connection;
use std::mem::take;

/// Struct helps enrich array of FilterEntity with needed sub-entities
pub(super) struct FullFilterListBuilder {
    rules_map: Option<MapFilterIdOnRulesList>,
}

impl FullFilterListBuilder {
    pub(super) const fn new() -> Self {
        Self { rules_map: None }
    }

    /// Sets a preloaded map {filterId => RulesList}, so it won't be requested from database
    pub(super) fn set_rules_map(&mut self, map: MapFilterIdOnRulesList) {
        self.rules_map = Some(map);
    }

    /// Builds a list of FullFilterList objects from entities
    pub(super) fn build(
        mut self,
        conn: Connection,
        entities: Vec<FilterEntity>,
    ) -> FLMResult<Vec<FullFilterList>> {
        let locales_map = FilterLocaleRepository::new()
            .select_mapped(&conn)
            .map_err(FLMError::from_database)?;

        let tags_map = FilterTagRepository::new()
            .select_with_filter_tag(&conn)
            .map_err(FLMError::from_database)?;

        let mut rules_map = self.take_rules_map(&conn)?;

        let mut full_filters: Vec<FullFilterList> = Vec::with_capacity(entities.len());
        for filter in entities {
            let id = match filter.filter_id {
                None => return FLMError::make_err("Cannot resolve filter_id"),
                Some(id) => id,
            };

            let tags = match tags_map.get(&id) {
                None => vec![],
                Some(tags) => tags.iter().map(|tag| tag.clone().into()).collect(),
            };

            let languages = match locales_map.get(&id) {
                None => vec![],
                Some(languages) => languages.iter().map(|locale| locale.lang.clone()).collect(),
            };

            let rules: Option<FilterListRules> = rules_map.remove(&id).map(|e| e.into());

            match FullFilterList::from_filter_entity(filter, tags, languages, rules) {
                None => return FLMError::make_err(format!("Cannot build filter_id: {}", id)),
                Some(full_filter_list) => full_filters.push(full_filter_list),
            }
        }

        Ok(full_filters)
    }

    /// Takes rules_map from builder, if they were set earlier, or loads them from DB
    fn take_rules_map(&mut self, conn: &Connection) -> FLMResult<MapFilterIdOnRulesList> {
        if self.rules_map.is_some() {
            let map: Option<MapFilterIdOnRulesList> = take(&mut self.rules_map);
            Ok(map.unwrap())
        } else {
            RulesListRepository::new()
                .select_mapped(&conn, None)
                .map_err(FLMError::from_database)
        }
    }
}
