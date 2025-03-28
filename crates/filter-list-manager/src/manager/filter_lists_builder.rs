use crate::manager::models::filter_list_rules::FilterListRules;
use crate::manager::models::full_filter_list::FullFilterList;
use crate::storage::entities::filter::filter_entity::FilterEntity;
use crate::storage::repositories::filter_locale_repository::FilterLocaleRepository;
use crate::storage::repositories::filter_tag_repository::FilterTagRepository;
use crate::storage::repositories::localisation::filter_localisations_repository::FilterLocalisationRepository;
use crate::storage::repositories::rules_list_repository::{
    MapFilterIdOnRulesList, RulesListRepository,
};
use crate::{FLMError, FLMResult, Locale, StoredFilterMetadata};
use rusqlite::Connection;
use std::mem::take;

/// Struct helps enrich array of FilterEntity with needed sub-entities
pub(super) struct FullFilterListBuilder<'a> {
    rules_map: Option<MapFilterIdOnRulesList>,
    locale: &'a Locale,
}

impl<'a> FullFilterListBuilder<'a> {
    pub(super) const fn new(locale: &'a Locale) -> Self {
        Self {
            rules_map: None,
            locale,
        }
    }

    /// Sets a preloaded map {filterId => RulesList}, so it won't be requested from database
    pub(super) fn set_rules_map(&mut self, map: MapFilterIdOnRulesList) {
        self.rules_map = Some(map);
    }

    /// Builds a list of [`StoredFilterMetadata`] objects from entities
    pub(super) fn build_stored_filter_metadata_lists(
        self,
        conn: Connection,
        entities: Vec<FilterEntity>,
    ) -> FLMResult<Vec<StoredFilterMetadata>> {
        self.build_filter_lists_with_block(&conn, entities, |metadata| metadata)
    }

    /// Builds a list of [`FullFilterList`] objects from entities
    pub(super) fn build_full_filter_lists(
        mut self,
        conn: Connection,
        entities: Vec<FilterEntity>,
    ) -> FLMResult<Vec<FullFilterList>> {
        let mut rules_map = self.take_rules_map(&conn)?;

        self.build_filter_lists_with_block(
            &conn,
            entities,
            |stored_filter_metadata: StoredFilterMetadata| {
                let filter_id = &stored_filter_metadata.id;

                let rules: Option<FilterListRules> = rules_map.remove(filter_id).map(|e| e.into());
                FullFilterList::from_stored_filter_metadata(stored_filter_metadata, rules)
            },
        )
    }

    /// Builds filter lists based on [`StoredFilterMetadata`] entities
    fn build_filter_lists_with_block<Out, Block>(
        &self,
        conn: &Connection,
        mut entities: Vec<FilterEntity>,
        mut block: Block,
    ) -> FLMResult<Vec<Out>>
    where
        Block: FnMut(StoredFilterMetadata) -> Out,
    {
        let locales_map = FilterLocaleRepository::new()
            .select_mapped(conn)
            .map_err(FLMError::from_database)?;

        let tags_map = FilterTagRepository::new()
            .select_with_filter_tag(conn)
            .map_err(FLMError::from_database)?;

        FilterLocalisationRepository::new()
            .enrich_filter_lists_with_localisation(conn, &mut entities, self.locale)
            .map_err(FLMError::from_database)?;

        let mut out = Vec::with_capacity(entities.len());
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

            let stored_entity =
                match StoredFilterMetadata::from_filter_entity(filter, tags, languages) {
                    None => return FLMError::make_err(format!("Cannot build filter_id: {}", id)),
                    Some(stored_filter_entity) => stored_filter_entity,
                };

            out.push(block(stored_entity));
        }

        Ok(out)
    }

    /// Takes rules_map from builder, if they were set earlier, or loads them from DB
    fn take_rules_map(&mut self, conn: &Connection) -> FLMResult<MapFilterIdOnRulesList> {
        if self.rules_map.is_some() {
            let map: Option<MapFilterIdOnRulesList> = take(&mut self.rules_map);
            Ok(map.unwrap())
        } else {
            RulesListRepository::new()
                .select_mapped(conn, None)
                .map_err(FLMError::from_database)
        }
    }
}
