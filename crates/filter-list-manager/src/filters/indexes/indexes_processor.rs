use super::entities::{IndexEntity, IndexI18NEntity};
use crate::filters::indexes::index_consistency_checker::check_consistency;
use crate::io::http::blocking_client::BlockingClient;
use crate::io::url_schemes::UrlSchemes;
use crate::io::{get_scheme, read_file_by_url};
use crate::manager::models::{MovedFilterInfo, PullMetadataResult};
use crate::storage::entities::filter::filter_entity::FilterEntity;
use crate::storage::entities::filter_filter_tag_entity::FilterFilterTagEntity;
use crate::storage::entities::filter_locale_entity::FilterLocaleEntity;
use crate::storage::repositories::db_metadata_repository::DBMetadataRepository;
use crate::storage::repositories::filter_includes_repository::FilterIncludesRepository;
use crate::storage::repositories::rules_list_repository::RulesListRepository;
use crate::storage::repositories::BulkDeleteRepository;
use crate::storage::spawn_transaction;
use crate::storage::DbConnectionManager;
use crate::utils::integrity::{derive_key_if_needed, sign_filter_count};
use crate::{
    storage::repositories::filter_filter_tag_repository::FilterFilterTagRepository,
    storage::repositories::filter_locale_repository::FilterLocaleRepository,
    storage::repositories::{
        filter_group_repository::FilterGroupRepository, filter_repository::FilterRepository,
        filter_tag_repository::FilterTagRepository,
        localisation::filter_localisations_repository::FilterLocalisationRepository,
        localisation::filter_tag_localisation_repository::FilterTagLocalisationRepository,
        localisation::group_localisation_repository::GroupLocalisationRepository, Repository,
    },
    string, Configuration, FLMError, FLMResult, FilterId, CUSTOM_FILTERS_GROUP_ID,
};
use rusqlite::{Connection, Transaction};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::mem::take;
use std::sync::Arc;
use std::thread::scope as thread_scope;

/// The class responsible for updating filters and rules from indexes
pub struct IndexesProcessor<'a> {
    connection_source: &'a DbConnectionManager,
    loaded_index: Option<IndexEntity>,
    loaded_index_i18n: Option<IndexI18NEntity>,
    http_client: Arc<BlockingClient>,
    /// Derived integrity key for resigning filter metadata after index merge.
    /// `None` when integrity protection is disabled in configuration.
    derived_key: Option<[u8; 32]>,
}

/// Public methods
impl<'a> IndexesProcessor<'a> {
    /// Default ctor
    pub fn factory(
        connection_source: &'a DbConnectionManager,
        configuration: &Configuration,
    ) -> FLMResult<Self> {
        Ok(Self {
            connection_source,
            loaded_index: None,
            loaded_index_i18n: None,
            http_client: Arc::new(BlockingClient::new(configuration)?),
            derived_key: derive_key_if_needed(configuration),
        })
    }

    /// Synchronizes filters metadata (with groups, locales, etc...) with remote server
    /// If database is empty or is not exist it will be created.
    ///
    /// * `index_url` - Remote server URL of filters index
    /// * `index_locales_url` - Remote server URL of filters index localisation info
    /// * `with_filters` - Filters from index will be downloaded after
    pub fn sync_metadata(
        &mut self,
        index_url: &str,
        index_locales_url: &str,
    ) -> FLMResult<PullMetadataResult> {
        // Load indices and check consistency
        self.fetch_indices(string!(index_url), string!(index_locales_url))?;

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
        conn: &mut Connection,
        filters_from_storage: Vec<FilterEntity>,
    ) -> FLMResult<PullMetadataResult> {
        let mut filters_must_be_deleted: Vec<FilterId> = vec![];
        let mut new_or_updated_filters: Vec<FilterEntity> = vec![];
        let mut tags_of_filters: Vec<Vec<FilterFilterTagEntity>> = vec![];
        let mut locales_of_filters: Vec<Vec<FilterLocaleEntity>> = vec![];
        let mut index_filters_map: HashMap<FilterId, FilterEntity> = HashMap::new();
        let mut out = PullMetadataResult::new();

        let index = self.exchange_index()?;

        for filter in index.filters {
            if filter.deprecated {
                continue;
            }

            let filter_id = filter.filterId;
            let storage_entities = filter.into_storage_entities();

            tags_of_filters.push(storage_entities.tags);
            locales_of_filters.push(storage_entities.locales);
            index_filters_map.insert(filter_id, storage_entities.filter);
        }

        for mut filter in filters_from_storage {
            // Do not work with custom filters
            if filter.is_custom() {
                continue;
            }

            let filter_id = match filter.filter_id {
                None => {
                    return FLMError::make_err(format!(
                        "Couldn't determine filter_id for filter with url: \"{}\"",
                        filter.download_url
                    ))
                }
                Some(filter_id) => filter_id,
            };

            // This is a special filter. Skip
            if !filter_id.is_positive() {
                continue;
            }

            if let Some(filter_from_index) = index_filters_map.remove(&filter_id) {
                filter.display_number = filter_from_index.display_number;
                filter.title = filter_from_index.title;
                filter.description = filter_from_index.description;
                filter.homepage = filter_from_index.homepage;
                filter.expires = filter_from_index.expires;
                filter.download_url = filter_from_index.download_url;
                filter.last_update_time = filter_from_index.last_update_time;
                filter.subscription_url = filter_from_index.subscription_url;

                new_or_updated_filters.push(filter);
            } else {
                // If filter is not in index
                if filter.is_enabled {
                    // Filter id will be updated right before insert
                    filter.group_id = CUSTOM_FILTERS_GROUP_ID;

                    new_or_updated_filters.push(filter);
                } else {
                    // Filter will just be removed, not moved
                    out.removed_filters.push(filter_id);
                }

                filters_must_be_deleted.push(filter_id);
            }
        }

        // Add new filters
        for (filter_id, filter) in index_filters_map {
            // Filter id will be updated right before insert
            new_or_updated_filters.push(filter);
            out.added_filters.push(filter_id);
        }

        let (transaction, _) = spawn_transaction(conn, |transaction: &Transaction| {
            let filter_repository = FilterRepository::new();
            let group_repo = FilterGroupRepository::new();
            let tags_repo = FilterTagRepository::new();
            let rules_repository = RulesListRepository::new();
            let includes_repository = FilterIncludesRepository::new();
            let locales_repository = FilterLocaleRepository::new();
            let filter_filter_tag_repository = FilterFilterTagRepository::new();
            let filter_localisation_repository = FilterLocalisationRepository::new();
            let filter_tag_localisation_repository = FilterTagLocalisationRepository::new();
            let group_localisation_repository = GroupLocalisationRepository::new();

            // Clear filter dependencies
            locales_repository.clear(transaction)?;
            group_repo.delete_index_groups(transaction)?;
            tags_repo.clear(transaction)?;
            // Remove old filters mappings and non-needed filters itself
            filter_filter_tag_repository.bulk_delete(transaction, &filters_must_be_deleted)?;
            rules_repository.bulk_delete(transaction, &filters_must_be_deleted)?;
            includes_repository.delete_for_filters(
                transaction,
                filters_must_be_deleted.iter(),
                filters_must_be_deleted.len(),
            )?;
            filter_repository.bulk_delete(transaction, &filters_must_be_deleted)?;
            // Clear filter dependencies localisations
            filter_localisation_repository.clear(transaction)?;
            filter_tag_localisation_repository.clear(transaction)?;
            group_localisation_repository.clear(transaction)?;

            // Save new or updated filters and all filter deps
            locales_repository.insert(
                transaction,
                &locales_of_filters
                    .into_iter()
                    .flatten()
                    .collect::<Vec<FilterLocaleEntity>>(),
            )?;
            filter_filter_tag_repository.insert(
                transaction,
                &tags_of_filters
                    .into_iter()
                    .flatten()
                    .collect::<Vec<FilterFilterTagEntity>>(),
            )?;
            group_repo.insert(transaction, &index.groups)?;
            tags_repo.insert(transaction, &index.tags)?;

            let mut affected_ids: Vec<FilterId> = Vec::with_capacity(new_or_updated_filters.len());
            filter_repository.insert_with_chosen_filters_callback(
                transaction,
                &new_or_updated_filters,
                |entity_will_insert, chosen_id| {
                    // Collect ids for resign
                    if let Some(next_id) = chosen_id.as_ref() {
                        affected_ids.push(*next_id);
                    }
                    // Has both ids
                    if let Some((previous_id, next_id)) = entity_will_insert
                        .filter_id
                        .as_ref()
                        .zip(chosen_id.as_ref())
                    {
                        // They aren't equal, so filter was moved
                        if previous_id != next_id {
                            out.moved_filters
                                .push(MovedFilterInfo::new(*previous_id, *next_id));
                        }
                    }
                },
            )?;

            // Index merge mutated signed fields and may have added
            // brand-new filters with no signature. Re-sign affected rows and
            // refresh filter_count_signature so the next verify_*-call sees
            // valid integrity.
            self.resign_integrity_signatures_if_needed(
                transaction,
                &filter_repository,
                &affected_ids,
            )?;

            Ok(())
        })
        .map_err(FLMError::from_database)?;

        // Save localisations
        self.save_index_localisations(&transaction)?;

        transaction.commit().map_err(FLMError::from_database)?;

        Ok(out)
    }

    /// Saves new indexes on empty database
    fn save_indices_on_empty_database(
        &mut self,
        conn: &mut Connection,
    ) -> FLMResult<PullMetadataResult> {
        let index = self.exchange_index()?;

        let (transaction, added_filters) = spawn_transaction(conn, |transaction: &Transaction| {
            FilterGroupRepository::new().insert(transaction, &index.groups)?;

            FilterTagRepository::new().insert(transaction, &index.tags)?;

            let mut filters = Vec::with_capacity(index.filters.len());
            let filter_repository = FilterRepository::new();
            let mut locales = Vec::new();
            let mut tags = Vec::new();

            for filter_index_entity in index.filters {
                if filter_index_entity.deprecated {
                    continue;
                }

                let storage_entities = filter_index_entity.into_storage_entities();

                filters.push(storage_entities.filter);
                locales.push(storage_entities.locales);
                tags.push(storage_entities.tags);
            }

            let added_filters = filters
                .iter()
                .filter_map(|filter| filter.filter_id)
                .collect::<Vec<FilterId>>();

            filter_repository.insert(transaction, &filters)?;

            let flattened_locales = locales
                .into_iter()
                .flatten()
                .collect::<Vec<FilterLocaleEntity>>();
            FilterLocaleRepository::new().insert(transaction, &flattened_locales)?;

            let flattened_tags = tags
                .into_iter()
                .flatten()
                .collect::<Vec<FilterFilterTagEntity>>();
            FilterFilterTagRepository::new().insert(transaction, &flattened_tags)?;

            self.resign_integrity_signatures_if_needed(
                transaction,
                &filter_repository,
                &added_filters,
            )?;

            Ok(added_filters)
        })
        .map_err(FLMError::from_database)?;

        self.save_index_localisations(&transaction)?;
        transaction.commit().map_err(FLMError::from_database)?;

        Ok(PullMetadataResult::new_with_added_filters(added_filters))
    }

    /// Updated filters should be resigned. Also count hash should be updated.
    fn resign_integrity_signatures_if_needed(
        &self,
        tx: &Transaction,
        filter_repository: &FilterRepository,
        affected_ids: &[FilterId],
    ) -> rusqlite::Result<()> {
        if let Some(ref key) = self.derived_key {
            filter_repository.resign_filters_in_tx(tx, affected_ids, key)?;

            let count = filter_repository.count_all(tx)?;
            let count_sig = sign_filter_count(key, count);
            let mut meta = DBMetadataRepository::read(tx)?.unwrap_or_default();
            meta.filter_count_signature = Some(count_sig);

            return DBMetadataRepository::save(tx, &meta);
        }

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
    fn fetch_indices(&mut self, index_url: String, index_locales_url: String) -> FLMResult<()> {
        let http_client = Arc::clone(&self.http_client);

        let index_result: FLMResult<IndexEntity>;
        let mut index_localisations_result: Option<FLMResult<IndexI18NEntity>> = None;

        // Localizations are optional
        if !index_locales_url.is_empty() {
            let scope = thread_scope(|s| {
                let client1 = Arc::clone(&http_client);
                let h1 = s.spawn(move || Self::load_data::<IndexEntity>(&index_url, &client1));

                let client2 = Arc::clone(&http_client);
                let h2 = s.spawn(move || {
                    Self::load_data::<IndexI18NEntity>(&index_locales_url, &client2)
                });

                (h1.join(), h2.join())
            });

            index_result = scope
                .0
                .map_err(|_| FLMError::from_display("Thread panicked while loading index"))?;
            index_localisations_result = Some(scope.1.map_err(|_| {
                FLMError::from_display("Thread panicked while loading index localisations")
            })?);
        } else {
            index_result = Self::load_data::<IndexEntity>(&index_url, &http_client);
        }

        // Index operations
        let index = index_result?;
        check_consistency(&index)?;
        self.loaded_index = Some(index);

        // Localizations operations
        if let Some(result) = index_localisations_result {
            let index_localisations = result?;

            self.loaded_index_i18n = Some(index_localisations);
        }

        Ok(())
    }

    /// Loads indices data
    fn load_data<I>(url: &str, http_client: &BlockingClient) -> FLMResult<I>
    where
        I: DeserializeOwned,
    {
        let scheme: UrlSchemes = get_scheme(url).into();

        match scheme {
            UrlSchemes::File => {
                let contents = read_file_by_url(url).map_err::<FLMError, _>(Into::into)?;
                serde_json::from_str::<I>(&contents).map_err(FLMError::from_display)
            }
            UrlSchemes::Https | UrlSchemes::Http => {
                http_client.get_json::<I>(url).map_err(FLMError::Network)
            }
            _ => FLMError::make_err(format!("Unknown scheme for url: {}", url)),
        }
    }
}

/// Misc methods
impl IndexesProcessor<'_> {
    /// Saves data from index localisation
    fn save_index_localisations(&mut self, transaction: &Transaction) -> FLMResult<()> {
        if let Some(localisations) = take(&mut self.loaded_index_i18n) {
            let (group_vec, tags_vec, filters_vec) = localisations.exchange()?;

            GroupLocalisationRepository::new()
                .insert(transaction, &group_vec)
                .map_err(FLMError::from_database)?;

            FilterTagLocalisationRepository::new()
                .insert(transaction, &tags_vec)
                .map_err(FLMError::from_database)?;

            FilterLocalisationRepository::new()
                .insert(transaction, &filters_vec)
                .map_err(FLMError::from_database)?;
        }

        Ok(())
    }

    /// Tries to take index value from `self` object
    ///
    /// # Failure
    ///
    /// Returns [`Err`] if index value is none
    #[inline]
    fn exchange_index(&mut self) -> FLMResult<IndexEntity> {
        match take(&mut self.loaded_index) {
            None => FLMError::make_err("Empty index. You must fetch index before save"),
            Some(index) => Ok(index),
        }
    }
}

#[cfg(test)]
impl<'a> IndexesProcessor<'a> {
    /// Ctor for tests
    pub(crate) fn factory_test(
        connection_source: &'a DbConnectionManager,
        loaded_index: IndexEntity,
        loaded_index_i18n: IndexI18NEntity,
    ) -> Self {
        use lazy_static::lazy_static;

        lazy_static! {
            static ref TEST_CONFIG: Configuration = Configuration::default();
        }

        Self {
            connection_source,
            loaded_index: Some(loaded_index),
            loaded_index_i18n: Some(loaded_index_i18n),
            http_client: Arc::new(BlockingClient::new(&TEST_CONFIG).unwrap()),
            derived_key: derive_key_if_needed(&TEST_CONFIG),
        }
    }

    pub(crate) fn fill_empty_db(&mut self, conn: &mut Connection) -> FLMResult<()> {
        self.save_indices_on_empty_database(conn).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use crate::filters::indexes::indexes_processor::IndexesProcessor;
    use crate::manager::managers::integrity_control_manager::IntegrityControlManager;
    use crate::storage::entities::rules_list::rules_list_entity::RulesListEntity;
    use crate::storage::repositories::db_metadata_repository::DBMetadataRepository;
    use crate::storage::repositories::filter_filter_tag_repository::FilterFilterTagRepository;
    use crate::storage::repositories::filter_group_repository::FilterGroupRepository;
    use crate::storage::repositories::filter_repository::FilterRepository;
    use crate::storage::repositories::rules_list_repository::RulesListRepository;
    use crate::storage::repositories::Repository;
    use crate::storage::sql_generators::operator::SQLOperator::*;
    use crate::storage::with_transaction;
    use crate::storage::DbConnectionManager;
    use crate::test_utils::indexes_fixtures::build_filters_indices_fixtures;
    use crate::test_utils::tests_path;
    use crate::utils::integrity;
    use crate::utils::memory::heap;
    use crate::{
        string, Configuration, FLMError, FilterId, CUSTOM_FILTERS_GROUP_ID,
        MAXIMUM_CUSTOM_FILTER_ID, MINIMUM_CUSTOM_FILTER_ID,
    };
    use rand::seq::SliceRandom;
    use rand::{thread_rng, Rng};
    use rusqlite::Connection;
    use std::cell::RefCell;
    use std::rc::Rc;
    use url::Url;

    const DEPRECATED_FILTER_ID: FilterId = 1;

    #[test]
    fn test_save_indices_in_empty_db() {
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
        let filter_repository = FilterRepository::new();
        let rules_repository = RulesListRepository::new();
        let groups_repository = FilterGroupRepository::new();

        let connection_manager = DbConnectionManager::factory_test().unwrap();
        let config = Configuration::default();
        let mut indexes = IndexesProcessor::factory(&connection_manager, &config).unwrap();

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
                        let new_rules_entity = RulesListEntity::make(
                            new_chosen_filter.filter_id.clone().unwrap(),
                            string!(),
                            0,
                        );

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
            let config2 = Configuration::default();
            let mut second_indexes =
                IndexesProcessor::factory(&connection_manager, &config2).unwrap();

            let (mut index_second, index_localisation_second) =
                build_filters_indices_fixtures().unwrap();

            // Should deprecate here as well
            let deprecated = index_second
                .filters
                .iter_mut()
                .find(|e| e.filterId == DEPRECATED_FILTER_ID);
            deprecated.unwrap().deprecated = true;

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

                    // All index filters must have empty versions
                    filter_repository
                        .select_filters_except_bootstrapped(&conn)
                        .unwrap()
                        .unwrap()
                        .iter()
                        .for_each(|entity| {
                            assert!(entity.version.is_empty());
                        });

                    Ok(())
                })
                .unwrap();
        }
        // endregion

        // region test deprecated filter + test versions is empty for index filters
        connection_manager
            .execute_db(|conn: Connection| {
                let list = filter_repository
                    .select_filters_except_bootstrapped(&conn)
                    .unwrap()
                    .unwrap();

                let found = list
                    .iter()
                    .find(|f| f.filter_id.unwrap() == DEPRECATED_FILTER_ID);

                list.iter()
                    .for_each(|entity| assert!(entity.version.is_empty()));

                assert!(found.is_none());

                Ok(())
            })
            .unwrap();
    }

    #[test]
    fn test_load_indexes_from_local_paths() {
        let index_path = tests_path("fixtures/filters.json");
        let index_i18n_path = tests_path("fixtures/filters_i18n.json");

        let index_url = Url::from_file_path(index_path).unwrap().to_string();
        let index_i18_url = Url::from_file_path(index_i18n_path).unwrap().to_string();

        let connection_manager = DbConnectionManager::factory_test().unwrap();

        unsafe {
            connection_manager.lift_up_database().unwrap();
        };
        let config = Configuration::default();
        let mut processor = IndexesProcessor::factory(&connection_manager, &config).unwrap();

        processor.sync_metadata(&index_url, &index_i18_url).unwrap();
    }

    /// Regression test for AG-* (Storage Integrity Part II).
    ///
    /// In agflm_dns-{PRE,POST}.db on AdGuard mac we observed: after
    /// `pull_metadata` (which routes through `save_index_on_existing_database`)
    /// the filter row keeps its old `integrity_signature` while signed fields
    /// (`expires`, `last_update_time`, `download_url`, `subscription_url`) get
    /// overwritten from the fresh index. The next read that calls
    /// `verify_filter_entities` then fails with `FilterIntegrityCheckFailed`.
    ///
    /// The fix must make `pull_metadata` re-sign filter metadata using
    /// `Configuration::integrity_key`, so the test asserts the row's
    /// signature is still valid after the update.
    #[test]
    fn test_save_index_on_existing_database_keeps_filter_metadata_integrity() {
        const TEST_FILTER_ID: FilterId = 1;
        const INTEGRITY_KEY: &str = "test-integrity-key";

        let connection_manager = DbConnectionManager::factory_test().unwrap();
        unsafe { connection_manager.lift_up_database().unwrap() }

        // Build a single filter from the index fixture and sign its metadata
        // — this is exactly the state the DB is in after a host app calls
        // `IntegrityControlManager::sign_all_data` post-migration.
        let (initial_index, _) = build_filters_indices_fixtures().unwrap();
        let initial_index_filter = initial_index
            .filters
            .iter()
            .find(|f| f.filterId == TEST_FILTER_ID)
            .cloned()
            .expect("fixture must contain filter_id=1");
        let original_expires = initial_index_filter.expires;

        let mut filter_entity = initial_index_filter.into_storage_entities().filter;
        let derived_key = integrity::derive_key(INTEGRITY_KEY);
        integrity::sign_filter_entity(&derived_key, &mut filter_entity);
        assert!(
            filter_entity.integrity_signature().is_some(),
            "filter must be signed before the test",
        );

        let filter_repository = FilterRepository::new();
        connection_manager
            .execute_db(|mut conn: Connection| {
                with_transaction(&mut conn, |tx| {
                    filter_repository.insert(tx, &[filter_entity.clone()])
                })
            })
            .unwrap();

        // Sanity check: stored row has a valid signature.
        connection_manager
            .execute_db(|conn: Connection| {
                let filters = filter_repository
                    .select(
                        &conn,
                        Some(FieldEqualValue("filter_id", TEST_FILTER_ID.into())),
                    )
                    .unwrap()
                    .unwrap();
                assert_eq!(filters.len(), 1);
                integrity::verify_filter_entities(&derived_key, &filters)
                    .expect("signature must be valid right after sign_filter_entity");
                Ok(())
            })
            .unwrap();

        // Prepare the index for the next `pull_metadata`: same filter, but
        // with a different `expires` (mirrors the 0 → 345600 change observed
        // for filter_id=1 in the real PRE/POST databases).
        let (mut modified_index, modified_i18n) = build_filters_indices_fixtures().unwrap();
        let new_expires = original_expires + 12_345;
        assert_ne!(new_expires, original_expires);
        {
            let entry = modified_index
                .filters
                .iter_mut()
                .find(|f| f.filterId == TEST_FILTER_ID)
                .unwrap();
            entry.expires = new_expires;
        }

        let mut config = Configuration::default();
        config.integrity_key = Some(INTEGRITY_KEY.to_string());
        let mut processor = IndexesProcessor::factory(&connection_manager, &config).unwrap();
        processor.loaded_index = Some(modified_index);
        processor.loaded_index_i18n = Some(modified_i18n);

        connection_manager
            .execute_db(|mut conn: Connection| {
                let existing = filter_repository
                    .select_filters_except_bootstrapped(&conn)
                    .unwrap()
                    .unwrap();
                processor.save_index_on_existing_database(&mut conn, existing)
            })
            .unwrap();

        // After pull_metadata the row must have the new `expires` AND a
        // signature that matches the new metadata. Today the second part
        // fails because save_index_on_existing_database does not re-sign.
        connection_manager
            .execute_db(|conn: Connection| {
                let filters = filter_repository
                    .select(
                        &conn,
                        Some(FieldEqualValue("filter_id", TEST_FILTER_ID.into())),
                    )
                    .unwrap()
                    .unwrap();
                assert_eq!(filters.len(), 1);
                assert_eq!(
                    filters[0].expires, new_expires,
                    "pull_metadata must have applied the new `expires` from index",
                );

                integrity::verify_filter_entities(&derived_key, &filters).expect(
                    "pull_metadata must keep filter metadata integrity valid \
                     (re-sign after mutating signed fields)",
                );
                Ok(())
            })
            .unwrap();
    }

    /// Regression test for the empty-DB path of `pull_metadata`.
    ///
    /// A fresh DB has only the bootstrap user-rules row (unsigned by design
    /// — the host must call `sign_all_data()` once before any reads). The
    /// first `pull_metadata` then inserts dozens of new index filters. With
    /// integrity enabled, those new rows AND `filter_count_signature` must
    /// be signed in the same transaction so a subsequent `sign_all_data()`
    /// is enough for the host to land in a fully consistent state — or, as
    /// asserted here, the new rows themselves are already valid.
    #[test]
    fn test_save_indices_on_empty_database_signs_new_filter_metadata() {
        const INTEGRITY_KEY: &str = "test-integrity-key-empty-db";

        let connection_manager = DbConnectionManager::factory_test().unwrap();
        unsafe { connection_manager.lift_up_database().unwrap() }

        // Bootstrap inserts the user-rules row unsigned (documented contract).
        // Sign it manually so we can isolate what `save_indices_on_empty_database`
        // is responsible for — otherwise `verify_integrity` would fail on the
        // bootstrap row regardless of this test.
        let mut config = Configuration::default();
        config.integrity_key = Some(INTEGRITY_KEY.to_string());
        IntegrityControlManager::new()
            .sign_all_data(&connection_manager, &config)
            .expect("manual sign_all_data on the bootstrap row must succeed");

        // Populate the DB via the empty-DB path with integrity enabled.
        let (index, index_i18n) = build_filters_indices_fixtures().unwrap();
        let mut processor = IndexesProcessor::factory(&connection_manager, &config).unwrap();
        processor.loaded_index = Some(index);
        processor.loaded_index_i18n = Some(index_i18n);
        connection_manager
            .execute_db(|mut conn: Connection| processor.fill_empty_db(&mut conn))
            .unwrap();

        // Every newly-inserted filter metadata row must carry a valid
        // signature, and `filter_count_signature` must reflect the new count.
        let derived_key = integrity::derive_key(INTEGRITY_KEY);
        let filter_repository = FilterRepository::new();
        connection_manager
            .execute_db(|conn: Connection| {
                let filters = filter_repository
                    .select_filters_except_bootstrapped(&conn)
                    .unwrap()
                    .unwrap();
                assert!(
                    !filters.is_empty(),
                    "fixture must produce at least one filter",
                );
                for f in &filters {
                    assert!(
                        f.integrity_signature().is_some(),
                        "filter_id={:?} must be signed after empty-DB pull_metadata",
                        f.filter_id,
                    );
                }
                integrity::verify_filter_entities(&derived_key, &filters)
                    .expect("all freshly-inserted filter metadata must verify");

                let meta = DBMetadataRepository::read(&conn).unwrap().unwrap();
                let count_sig = meta
                    .filter_count_signature
                    .expect("filter_count_signature must be set");
                let count = filter_repository.count_all(&conn).unwrap();
                assert!(
                    integrity::verify_filter_count(&derived_key, count, &count_sig),
                    "filter_count_signature must match the post-insert count",
                );
                Ok(())
            })
            .unwrap();

        // Verifier (the one host apps call) must agree.
        IntegrityControlManager::new()
            .verify_integrity(&connection_manager, &config)
            .expect("verify_integrity must succeed end-to-end after empty-DB sync");
    }

    /// Regression test for the move-to-custom branch of `pull_metadata`.
    ///
    /// When an enabled index filter disappears from the next index, the row
    /// is reinserted under a new negative id (custom group) and its old
    /// `rules_list` / `filter_includes` are deliberately dropped (current
    /// design: dependent data may have changed alongside the index, so we
    /// re-fetch everything on the next `update_filters`).
    ///
    /// The integrity contract for that "wipe-and-reinsert" path:
    /// 1. The new negative-id filter row has a valid metadata signature.
    /// 2. No orphan `rules_list` / `filter_includes` rows survive under the
    ///    old positive id.
    /// 3. `filter_count_signature` matches the post-transaction count.
    /// 4. `IntegrityControlManager::verify_integrity()` passes end-to-end.
    #[test]
    fn test_save_index_on_existing_database_move_to_custom_keeps_integrity() {
        use crate::storage::entities::filter::filter_include_entity::FilterIncludeEntity;
        use crate::storage::repositories::filter_includes_repository::FilterIncludesRepository;

        const TEST_FILTER_ID: FilterId = 1;
        const INTEGRITY_KEY: &str = "test-integrity-key-move-to-custom";

        let connection_manager = DbConnectionManager::factory_test().unwrap();
        unsafe { connection_manager.lift_up_database().unwrap() }

        let mut config = Configuration::default();
        config.integrity_key = Some(INTEGRITY_KEY.to_string());
        let derived_key = integrity::derive_key(INTEGRITY_KEY);
        let filter_repository = FilterRepository::new();
        let rules_repository = RulesListRepository::new();
        let includes_repository = FilterIncludesRepository::new();

        // 1. Bring DB into a "fully synced + signed" state: empty-DB sync
        //    populates the index filters (already signed by our empty-DB fix).
        let (index, index_i18n) = build_filters_indices_fixtures().unwrap();
        let mut processor = IndexesProcessor::factory(&connection_manager, &config).unwrap();
        processor.loaded_index = Some(index);
        processor.loaded_index_i18n = Some(index_i18n);
        connection_manager
            .execute_db(|mut conn: Connection| processor.fill_empty_db(&mut conn))
            .unwrap();

        // 2. Make TEST_FILTER_ID look like a real, in-use index filter:
        //    enable it and attach a rules_list row + a filter_includes row.
        //    Then run sign_all_data so everything (including those rows and
        //    the freshly-toggled `is_enabled`) carries valid signatures.
        let downloaded_url: String = connection_manager
            .execute_db(|mut conn: Connection| {
                let url: String = conn
                    .query_row(
                        "SELECT download_url FROM [filter] WHERE filter_id = ?1",
                        rusqlite::params![TEST_FILTER_ID],
                        |row| row.get(0),
                    )
                    .unwrap();

                with_transaction(&mut conn, |tx| {
                    filter_repository.toggle_filter_lists(tx, &[TEST_FILTER_ID], true)?;

                    let rules =
                        RulesListEntity::make(TEST_FILTER_ID, "||example.com^".to_string(), 1);
                    rules_repository.insert(tx, &[rules])?;

                    let include = FilterIncludeEntity::make(
                        TEST_FILTER_ID,
                        "https://example.com/inc.txt".to_string(),
                        1,
                        "||tracker.example.com^".to_string(),
                    );
                    includes_repository.replace_entities_for_filters(tx, &[include])
                })
                .map(|_| url)
            })
            .unwrap();

        IntegrityControlManager::new()
            .sign_all_data(&connection_manager, &config)
            .expect("sign_all_data must succeed on the fully-populated DB");
        IntegrityControlManager::new()
            .verify_integrity(&connection_manager, &config)
            .expect("baseline verify_integrity must pass before the move");

        // 3. Run pull_metadata with an index that marks TEST_FILTER_ID as
        //    deprecated — same effect as "disappeared from index" for the
        //    move-to-custom branch.
        let (mut modified_index, modified_i18n) = build_filters_indices_fixtures().unwrap();
        modified_index
            .filters
            .iter_mut()
            .find(|f| f.filterId == TEST_FILTER_ID)
            .expect("fixture must contain filter_id=1")
            .deprecated = true;

        let mut processor2 = IndexesProcessor::factory(&connection_manager, &config).unwrap();
        processor2.loaded_index = Some(modified_index);
        processor2.loaded_index_i18n = Some(modified_i18n);
        connection_manager
            .execute_db(|mut conn: Connection| {
                let existing = filter_repository
                    .select_filters_except_bootstrapped(&conn)
                    .unwrap()
                    .unwrap();
                processor2.save_index_on_existing_database(&mut conn, existing)
            })
            .unwrap();

        // 4. Post-conditions:
        connection_manager
            .execute_db(|conn: Connection| {
                // Old positive id must be gone everywhere.
                let old_filter_count: i32 = conn
                    .query_row(
                        "SELECT COUNT(*) FROM [filter] WHERE filter_id = ?1",
                        rusqlite::params![TEST_FILTER_ID],
                        |row| row.get(0),
                    )
                    .unwrap();
                assert_eq!(
                    old_filter_count, 0,
                    "old positive filter row must be deleted"
                );

                let orphan_rules: i32 = conn
                    .query_row(
                        "SELECT COUNT(*) FROM [rules_list] WHERE filter_id = ?1",
                        rusqlite::params![TEST_FILTER_ID],
                        |row| row.get(0),
                    )
                    .unwrap();
                assert_eq!(orphan_rules, 0, "no orphan rules_list rows under old id");

                let orphan_includes: i32 = conn
                    .query_row(
                        "SELECT COUNT(*) FROM [filter_includes] WHERE filter_id = ?1",
                        rusqlite::params![TEST_FILTER_ID],
                        |row| row.get(0),
                    )
                    .unwrap();
                assert_eq!(
                    orphan_includes, 0,
                    "no orphan filter_includes rows under old id"
                );

                // The filter must have been re-keyed to a new negative id in
                // the custom group, retaining its original download_url.
                let mut moved = filter_repository
                    .select(
                        &conn,
                        Some(FieldEqualValue("download_url", downloaded_url.into())),
                    )
                    .unwrap()
                    .unwrap();
                assert_eq!(
                    moved.len(),
                    1,
                    "exactly one row must carry the original url"
                );
                let moved = moved.pop().unwrap();
                let new_id = moved.filter_id.unwrap();
                assert!(
                    new_id <= crate::MAXIMUM_CUSTOM_FILTER_ID
                        && new_id >= crate::MINIMUM_CUSTOM_FILTER_ID,
                    "moved filter must land in the custom id range, got {new_id}",
                );
                assert_eq!(moved.group_id, CUSTOM_FILTERS_GROUP_ID);
                assert!(moved.is_enabled);

                // Drop-everything policy: new id has no rules/includes yet.
                let new_rules: i32 = conn
                    .query_row(
                        "SELECT COUNT(*) FROM [rules_list] WHERE filter_id = ?1",
                        rusqlite::params![new_id],
                        |row| row.get(0),
                    )
                    .unwrap();
                assert_eq!(new_rules, 0);
                let new_includes: i32 = conn
                    .query_row(
                        "SELECT COUNT(*) FROM [filter_includes] WHERE filter_id = ?1",
                        rusqlite::params![new_id],
                        |row| row.get(0),
                    )
                    .unwrap();
                assert_eq!(new_includes, 0);

                // Filter metadata signature on the new id must be valid.
                integrity::verify_filter_entities(&derived_key, &[moved])
                    .expect("moved filter's metadata signature must be valid");

                // filter_count_signature must agree with current count.
                let meta = DBMetadataRepository::read(&conn).unwrap().unwrap();
                let sig = meta
                    .filter_count_signature
                    .expect("filter_count_signature must be set");
                let count = filter_repository.count_all(&conn).unwrap();
                assert!(
                    integrity::verify_filter_count(&derived_key, count, &sig),
                    "filter_count_signature must match the post-move count",
                );
                Ok(())
            })
            .unwrap();

        // 5. End-to-end streaming verifier (rules, includes, metadata, count).
        IntegrityControlManager::new()
            .verify_integrity(&connection_manager, &config)
            .expect("verify_integrity must succeed after move-to-custom");
    }
}
