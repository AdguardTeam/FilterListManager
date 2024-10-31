use super::entities::{IndexEntity, IndexI18NEntity};
use crate::filters::indexes::index_consistency_checker::check_consistency;
use crate::storage::entities::filter_entity::FilterEntity;
use crate::storage::entities::filter_filter_tag_entity::FilterFilterTagEntity;
use crate::storage::entities::filter_locale_entity::FilterLocaleEntity;
use crate::storage::repositories::rules_list_repository::RulesListRepository;
use crate::storage::repositories::BulkDeleteRepository;
use crate::storage::spawn_transaction;
use crate::storage::DbConnectionManager;
use crate::{
    io::http::HttpClient,
    storage::repositories::filter_filter_tag_repository::FilterFilterTagRepository,
    storage::repositories::filter_locale_repository::FilterLocaleRepository,
    storage::repositories::{
        filter_group_repository::FilterGroupRepository, filter_repository::FilterRepository,
        filter_tag_repository::FilterTagRepository,
        localisation::filter_localisations_repository::FilterLocalisationRepository,
        localisation::filter_tag_localisation_repository::FilterTagLocalisationRepository,
        localisation::group_localisation_repository::GroupLocalisationRepository, Repository,
    },
    FLMError, FLMResult, FilterId, CUSTOM_FILTERS_GROUP_ID,
};
use rusqlite::{Connection, Transaction};
use std::collections::HashMap;
use std::mem::take;

/// The class responsible for updating filters and rules from indexes
pub struct IndexesProcessor<'a> {
    connection_source: &'a DbConnectionManager,
    loaded_index: Option<IndexEntity>,
    loaded_index_i18n: Option<IndexI18NEntity>,
    connect_timeout: i32,
}

/// Public methods
impl<'a> IndexesProcessor<'a> {
    /// Default ctor
    pub fn factory(connection_source: &'a DbConnectionManager, connect_timeout: i32) -> Self {
        Self {
            connection_source,
            connect_timeout,
            loaded_index: None,
            loaded_index_i18n: None,
        }
    }

    /// Synchronizes filters metadata (with groups, locales, etc...) with remote server
    /// If database is empty or is not exist it will be created.
    ///
    /// * `index_url` - Remote server URL of filters index
    /// * `index_locales_url` - Remote server URL of filters index localisation info
    /// * `with_filters` - Filters from index will be downloaded after
    pub fn sync_metadata(
        &mut self,
        index_url: &String,
        index_locales_url: &String,
    ) -> FLMResult<()> {
        let async_rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(FLMError::from_io)?;

        // Load indices and check consistency
        async_rt.block_on(self.fetch_indices(index_url, index_locales_url))?;

        // TODO: Write to readme when we get rid of metadata
        self.connection_source
            .execute_db(move |mut conn: Connection| {
                let filters_optional = FilterRepository::new()
                    .select_filters_except_bootstrapped(&conn)
                    .map_err(FLMError::from_database)?;

                if let Some(loaded_filters) = filters_optional {
                    self.save_index_on_existing_database(&mut conn, loaded_filters)
                } else {
                    self.save_indices_on_empty_database(&mut conn)
                }
            })
    }
}

/// Save strategies
impl IndexesProcessor<'_> {
    /// Saves new indexes into existing database
    fn save_index_on_existing_database(
        &mut self,
        mut conn: &mut Connection,
        filters_from_storage: Vec<FilterEntity>,
    ) -> FLMResult<()> {
        let mut filters_must_be_deleted: Vec<FilterId> = vec![];
        let mut new_or_updated_filters: Vec<FilterEntity> = vec![];
        let mut tags_of_filters: Vec<Vec<FilterFilterTagEntity>> = vec![];
        let mut locales_of_filters: Vec<Vec<FilterLocaleEntity>> = vec![];
        let mut filters_map: HashMap<FilterId, FilterEntity> = HashMap::new();

        let index = self.exchange_index()?;

        for filter in index.filters {
            if filter.deprecated {
                continue;
            }

            let filter_id = filter.filterId;
            let storage_entities = filter.to_storage_entities();

            tags_of_filters.push(storage_entities.tags);
            locales_of_filters.push(storage_entities.locales);
            filters_map.insert(filter_id, storage_entities.filter);
        }

        for mut filter in filters_from_storage {
            // Do not work with custom filters
            if filter.is_custom() {
                continue;
            }

            let filter_id = match filter.filter_id {
                None => {
                    return FLMError::make_err(format!(
                    "That can't be, but i cannot determine filter_id for filter with url: \"{}\"",
                    filter.download_url
                ))
                }
                Some(filter_id) => filter_id,
            };

            // This is a special filter. Skip
            if !filter_id.is_positive() {
                continue;
            }

            if let Some(filter_from_index) = filters_map.remove(&filter_id) {
                filter.display_number = filter_from_index.display_number;
                filter.title = filter_from_index.title;
                filter.description = filter_from_index.description;
                filter.homepage = filter_from_index.homepage;
                filter.expires = filter_from_index.expires;
                filter.download_url = filter_from_index.download_url;
                filter.last_update_time = filter_from_index.last_update_time;

                new_or_updated_filters.push(filter);
            } else {
                // If filter is not in index
                if filter.is_enabled {
                    filter.group_id = CUSTOM_FILTERS_GROUP_ID;
                    filter.filter_id = None;

                    new_or_updated_filters.push(filter);
                }

                filters_must_be_deleted.push(filter_id);
            }
        }

        let (transaction, _) = spawn_transaction(&mut conn, |transaction: &Transaction| {
            let filter_repository = FilterRepository::new();
            let group_repo = FilterGroupRepository::new();
            let tags_repo = FilterTagRepository::new();
            let rules_repository = RulesListRepository::new();
            let locales_repository = FilterLocaleRepository::new();
            let filter_filter_tag_repository = FilterFilterTagRepository::new();
            let filter_localisation_repository = FilterLocalisationRepository::new();
            let filter_tag_localisation_repository = FilterTagLocalisationRepository::new();
            let group_localisation_repository = GroupLocalisationRepository::new();

            // Clear filter dependencies
            locales_repository.clear(&transaction)?;
            group_repo.delete_index_groups(&transaction)?;
            tags_repo.clear(&transaction)?;
            // Remove old filters mappings and non-needed filters itself
            filter_filter_tag_repository.bulk_delete(&transaction, &filters_must_be_deleted)?;
            rules_repository.bulk_delete(&transaction, &filters_must_be_deleted)?;
            filter_repository.bulk_delete(&transaction, &filters_must_be_deleted)?;
            // Clear filter dependencies localisations
            filter_localisation_repository.clear(&transaction)?;
            filter_tag_localisation_repository.clear(&transaction)?;
            group_localisation_repository.clear(&transaction)?;

            // Save new or updated filters and all filter deps
            locales_repository.insert(
                &transaction,
                &locales_of_filters
                    .into_iter()
                    .flatten()
                    .collect::<Vec<FilterLocaleEntity>>(),
            )?;
            filter_filter_tag_repository.insert(
                &transaction,
                &tags_of_filters
                    .into_iter()
                    .flatten()
                    .collect::<Vec<FilterFilterTagEntity>>(),
            )?;
            group_repo.insert(&transaction, &index.groups)?;
            tags_repo.insert(&transaction, &index.tags)?;

            filter_repository.insert(&transaction, &new_or_updated_filters)?;

            Ok(())
        })
        .map_err(FLMError::from_database)?;

        // Save localisations
        self.save_index_localisations(&transaction)?;

        transaction.commit().map_err(FLMError::from_database)?;

        Ok(())
    }

    /// Saves new indexes on empty database
    fn save_indices_on_empty_database(&mut self, mut conn: &mut Connection) -> FLMResult<()> {
        let index = self.exchange_index()?;

        let (transaction, _) = spawn_transaction(&mut conn, |transaction: &Transaction| {
            FilterGroupRepository::new().insert(&transaction, &index.groups)?;

            FilterTagRepository::new().insert(&transaction, &index.tags)?;

            let mut filters = Vec::with_capacity(index.filters.len());
            let filter_repository = FilterRepository::new();
            let mut locales = Vec::new();
            let mut tags = Vec::new();

            for filter_index_entity in index.filters {
                if filter_index_entity.deprecated {
                    continue;
                }

                let storage_entities = filter_index_entity.to_storage_entities();

                filters.push(storage_entities.filter);
                locales.push(storage_entities.locales);
                tags.push(storage_entities.tags);
            }

            filter_repository.insert(&transaction, &filters)?;

            let flattened_locales = locales
                .into_iter()
                .flatten()
                .collect::<Vec<FilterLocaleEntity>>();
            FilterLocaleRepository::new().insert(&transaction, &flattened_locales)?;

            let flattened_tags = tags
                .into_iter()
                .flatten()
                .collect::<Vec<FilterFilterTagEntity>>();
            FilterFilterTagRepository::new().insert(&transaction, &flattened_tags)?;

            Ok(filters)
        })
        .map_err(FLMError::from_database)?;

        self.save_index_localisations(&transaction)?;
        transaction.commit().map_err(FLMError::from_database)?;

        Ok(())
    }
}

/// Load indexes from server
impl IndexesProcessor<'_> {
    /// Fetches indices from remote server, checks index consistency fills `self` object fields.
    ///
    /// * `index_url` - Remote server URL of filters index
    /// * `index_locales_url` - Remote server URL of filters index localisation info
    ///
    /// # Failure
    ///
    /// May return an [`Err`] if the request to the remote server is unsuccessful
    /// or if the index consistency is violated.
    async fn fetch_indices(
        &mut self,
        index_url: &String,
        index_locales_url: &String,
    ) -> FLMResult<()> {
        let (index_result, index_localisations_result) = tokio::join!(
            self.load_main_index(index_url),
            self.load_index_localisations(index_locales_url)
        );

        let index = match index_result {
            Ok(index) => index,
            Err(err) => return Err(err),
        };

        let index_localisations = match index_localisations_result {
            Ok(index_localisations) => index_localisations,
            Err(err) => return Err(err),
        };

        check_consistency(&index)?;

        self.loaded_index = Some(index);
        self.loaded_index_i18n = Some(index_localisations);

        Ok(())
    }

    async fn load_main_index(&self, url: &String) -> FLMResult<IndexEntity> {
        HttpClient::get_json::<IndexEntity>(url, self.connect_timeout)
            .await
            .map_err(FLMError::Network)
    }

    async fn load_index_localisations(&self, url: &String) -> FLMResult<IndexI18NEntity> {
        HttpClient::get_json::<IndexI18NEntity>(url, self.connect_timeout)
            .await
            .map_err(FLMError::Network)
    }
}

/// Misc methods
impl IndexesProcessor<'_> {
    /// Saves data from index localisation
    fn save_index_localisations(&mut self, transaction: &Transaction) -> FLMResult<()> {
        let localisations = match take(&mut self.loaded_index_i18n) {
            None => {
                return FLMError::make_err(
                    "Index localisations is empty. You should fetch them first",
                );
            }
            Some(localisations) => localisations,
        };

        let (group_vec, tags_vec, filters_vec) = localisations.exchange()?;

        GroupLocalisationRepository::new()
            .insert(&transaction, &group_vec)
            .map_err(FLMError::from_database)?;

        FilterTagLocalisationRepository::new()
            .insert(&transaction, &tags_vec)
            .map_err(FLMError::from_database)?;

        FilterLocalisationRepository::new()
            .insert(&transaction, &filters_vec)
            .map_err(FLMError::from_database)
    }

    /// Tries to take index value from `self` object
    ///
    /// # Failure
    ///
    /// Returns [`Err`] if index value is none
    #[inline]
    fn exchange_index(&mut self) -> FLMResult<IndexEntity> {
        match take(&mut self.loaded_index) {
            None => return FLMError::make_err("Empty index. You must fetch index before save"),
            Some(index) => Ok(index),
        }
    }
}

#[cfg(test)]
impl<'a> IndexesProcessor<'a> {
    /// Ctor for tests
    pub(crate) const fn factory_test(
        connection_source: &'a DbConnectionManager,
        loaded_index: IndexEntity,
        loaded_index_i18n: IndexI18NEntity,
    ) -> Self {
        Self {
            connection_source,
            connect_timeout: 0,
            loaded_index: Some(loaded_index),
            loaded_index_i18n: Some(loaded_index_i18n),
        }
    }

    pub(crate) fn fill_empty_db(&mut self, conn: &mut Connection) -> FLMResult<()> {
        self.save_indices_on_empty_database(conn)
    }
}

#[cfg(test)]
mod tests {
    use crate::filters::indexes::indexes_processor::IndexesProcessor;
    use crate::storage::entities::rules_list_entity::RulesListEntity;
    use crate::storage::repositories::filter_filter_tag_repository::FilterFilterTagRepository;
    use crate::storage::repositories::filter_group_repository::FilterGroupRepository;
    use crate::storage::repositories::filter_repository::FilterRepository;
    use crate::storage::repositories::rules_list_repository::RulesListRepository;
    use crate::storage::repositories::Repository;
    use crate::storage::sql_generators::operator::SQLOperator::*;
    use crate::storage::with_transaction;
    use crate::storage::DbConnectionManager;
    use crate::test_utils::do_with_tests_helper;
    use crate::test_utils::indexes_fixtures::build_filters_indices_fixtures;
    use crate::utils::memory::heap;
    use crate::{
        FLMError, FilterId, CUSTOM_FILTERS_GROUP_ID, MAXIMUM_CUSTOM_FILTER_ID,
        MINIMUM_CUSTOM_FILTER_ID,
    };
    use rand::seq::SliceRandom;
    use rand::{thread_rng, Rng};
    use rusqlite::Connection;
    use std::cell::RefCell;
    use std::rc::Rc;

    const DEPRECATED_FILTER_ID: FilterId = 1;

    #[test]
    fn test_save_indices_in_empty_db() {
        do_with_tests_helper(|mut helper| helper.increment_postfix());

        let (mut index, index_localisation) = build_filters_indices_fixtures().unwrap();

        {
            let deprecated_filter_index_entity = index
                .filters
                .iter_mut()
                .find(|f| f.filterId == DEPRECATED_FILTER_ID)
                .unwrap();
            deprecated_filter_index_entity.deprecated = true;
            assert_eq!(
                deprecated_filter_index_entity.filterId,
                DEPRECATED_FILTER_ID
            );
        }

        let connection_source = DbConnectionManager::factory_test().unwrap();
        let mut indexes = IndexesProcessor::factory_test(
            &connection_source,
            // Do clone here, because of indexes.exchange_index()
            index.clone(),
            index_localisation.clone(),
        );

        unsafe {
            connection_source.lift_up_database().unwrap();
        }

        let filters_list = connection_source
            .execute_db(|mut conn: Connection| {
                indexes.save_indices_on_empty_database(&mut conn).unwrap();

                let filter_repository = FilterRepository::new();

                let filters_list = filter_repository
                    .select_filters_except_bootstrapped(&conn)
                    .unwrap()
                    .unwrap();
                Ok(filters_list)
            })
            .unwrap();

        assert_ne!(filters_list.is_empty(), true);

        let mut rng = thread_rng();
        for filter in filters_list.choose_multiple(&mut rng, 3) {
            let found_by_download_url = &index
                .filters
                .iter()
                .find(|index_filter| index_filter.downloadUrl == filter.download_url);

            assert!(found_by_download_url.is_some());
        }

        // Deprecated filter must not be saved
        let deprecated_filter = filters_list
            .iter()
            .find(|filter| filter.filter_id == Some(DEPRECATED_FILTER_ID));
        assert!(deprecated_filter.is_none());

        let filter_filter_tag_entities = connection_source
            .execute_db(|conn: Connection| {
                let filter_filter_tag_entities = FilterFilterTagRepository::new()
                    .select(
                        &conn,
                        Some(Not(heap(FieldEqualValue(
                            "filter_id",
                            DEPRECATED_FILTER_ID.into(),
                        )))),
                    )
                    .unwrap();

                Ok(filter_filter_tag_entities)
            })
            .unwrap();

        {
            let first = filter_filter_tag_entities.first().unwrap();
            let is_found = &index.filters.iter().find(|index_filter| {
                index_filter.filterId == first.filter_id
                    && index_filter
                        .tags
                        .iter()
                        .find(|tag| *tag == &first.tag_id)
                        .is_some()
            });

            assert!(is_found.is_some())
        }
    }

    #[test]
    fn test_save_indices_in_existent_db() {
        do_with_tests_helper(|mut helper| helper.increment_postfix());

        let filter_repository = FilterRepository::new();
        let rules_repository = RulesListRepository::new();
        let groups_repository = FilterGroupRepository::new();

        let connection_manager = DbConnectionManager::factory_test().unwrap();
        let mut indexes = IndexesProcessor::factory(&connection_manager, 0);

        let mut rng = thread_rng();
        let (mut index, index_localisation) = build_filters_indices_fixtures().unwrap();

        {
            let deprecated_filter_index_entity = index
                .filters
                .iter_mut()
                .find(|f| f.filterId == DEPRECATED_FILTER_ID)
                .unwrap();
            deprecated_filter_index_entity.deprecated = true;
            assert_eq!(
                deprecated_filter_index_entity.filterId,
                DEPRECATED_FILTER_ID
            );
        }

        unsafe {
            connection_manager.lift_up_database().unwrap();
        }

        let container = Rc::new(RefCell::new(index));

        // region Modify testing data
        let chosen_filter_id: FilterId = rng.gen_range(99999..999999);
        let chosen_filter_download_url = String::from("https://example.nonexistent");

        let (chosen_group, chosen_filter, chosen_group_id) = {
            let index_ref = Rc::clone(&container);
            let borrowed = index_ref.borrow();
            let mut chosen_group = borrowed.groups.choose(&mut rng).unwrap().clone();

            // New id for new group
            chosen_group.group_id = rng.gen_range(99999..999999);

            // Creates filter, which must be moved to custom filters after second update
            let mut chosen_filter = index_ref.borrow().filters.choose(&mut rng).unwrap().clone();
            chosen_filter.filterId = chosen_filter_id;
            chosen_filter.downloadUrl = chosen_filter_download_url.clone();
            // Assign group, which will be removed
            chosen_filter.groupId = chosen_group.group_id;
            let chosen_group_id = chosen_group.group_id;

            (chosen_group, chosen_filter, chosen_group_id)
        };

        {
            let mut_index = Rc::clone(&container);
            let mut borrowed_mut = mut_index.borrow_mut();
            borrowed_mut.filters.push(chosen_filter);
            borrowed_mut.groups.push(chosen_group);
        }
        // endregion

        let index_final = Rc::try_unwrap(container).unwrap().into_inner();

        indexes.loaded_index = Some(index_final);
        indexes.loaded_index_i18n = Some(index_localisation.clone());

        connection_manager
            .execute_db(|mut conn: Connection| indexes.save_indices_on_empty_database(&mut conn))
            .unwrap();

        {
            // Make chosen filter fully enabled
            // Now, fully enabled filter must have status = true, and have own Rules entity in DB
            connection_manager
                .execute_db(|mut conn: Connection| {
                    let mut new_chosen_filter = filter_repository
                        .select(
                            &conn,
                            Some(FieldEqualValue("filter_id", chosen_filter_id.into())),
                        )
                        .unwrap()
                        .unwrap()
                        .pop()
                        .unwrap();

                    new_chosen_filter.is_enabled = true;

                    with_transaction(&mut conn, |transaction| {
                        let new_rules_entity = RulesListEntity {
                            filter_id: new_chosen_filter.filter_id.clone().unwrap(),
                            text: "".to_string(),
                            disabled_text: "".to_string(),
                        };

                        let _ = &rules_repository
                            .insert(&transaction, &[new_rules_entity])
                            .unwrap();

                        let _ = &filter_repository
                            .insert(&transaction, &[new_chosen_filter])
                            .unwrap();

                        Ok(())
                    })
                })
                .unwrap()
        }

        // region Check testing_data
        let groups_map = connection_manager
            .execute_db(|conn: Connection| {
                groups_repository
                    .select_mapped(&conn)
                    .map_err(FLMError::from_database)
            })
            .unwrap();

        // Chosen group must be present in database
        assert!(groups_map.contains_key(&chosen_group_id));

        // region second update
        {
            let mut second_indexes = IndexesProcessor::factory(&connection_manager, 0);

            let (index_second, index_localisation_second) =
                build_filters_indices_fixtures().unwrap();

            second_indexes.loaded_index = Some(index_second);
            second_indexes.loaded_index_i18n = Some(index_localisation_second);

            connection_manager
                .execute_db(|mut conn: Connection| {
                    {
                        let existed_indexes =
                            filter_repository.select(&conn, None).unwrap().unwrap();
                        second_indexes
                            .save_index_on_existing_database(&mut conn, existed_indexes)
                            .unwrap();
                    }

                    let second_groups_mapped = groups_repository.select_mapped(&conn).unwrap();

                    assert!(!second_groups_mapped.contains_key(&chosen_group_id));
                    // Try to get chosen filter info one more time
                    // It must have a new id, because it was moved to custom filters
                    let mut chosen_filters_list = filter_repository
                        .select(
                            &conn,
                            Some(FieldEqualValue(
                                "download_url",
                                chosen_filter_download_url.into(),
                            )),
                        )
                        .unwrap()
                        .unwrap();

                    // Must be only one filter with this download url
                    assert_eq!(chosen_filters_list.len(), 1);

                    let chosen_filter_new_info = chosen_filters_list.pop().unwrap();

                    let filter_id = chosen_filter_new_info.filter_id.unwrap();

                    // This filter must be moved into custom group
                    assert_eq!(chosen_filter_new_info.group_id, CUSTOM_FILTERS_GROUP_ID);

                    // FilterId must be in designated range
                    assert!(filter_id >= MINIMUM_CUSTOM_FILTER_ID);
                    assert!(filter_id <= MAXIMUM_CUSTOM_FILTER_ID);

                    // Custom group must be removed
                    assert!(!second_groups_mapped.contains_key(&chosen_group_id));

                    Ok(())
                })
                .unwrap();
        }
        // endregion

        // region test deprecated filter
        connection_manager
            .execute_db(|conn: Connection| {
                let list = filter_repository
                    .select_filters_except_bootstrapped(&conn)
                    .unwrap()
                    .unwrap();

                let found = list
                    .iter()
                    .find(|f| f.filter_id.unwrap() == DEPRECATED_FILTER_ID);

                assert!(found.is_none());

                Ok(())
            })
            .unwrap();
    }
}
