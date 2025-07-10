//! Default implementation for [`FilterListManager`]

use super::managers::configuration_update_manager::ConfigurationUpdateManager;
use super::managers::db_manager::DbManager;
use super::managers::filter_group_manager::FilterGroupManager;
use super::managers::filter_manager::FilterManager;
use super::managers::filter_metadata_grabber::FilterMetadataGrabber;
use super::managers::filter_tag_manager::FilterTagManager;
use super::managers::filter_update_manager::FilterUpdateManager;
use super::managers::rules_list_manager::RulesListManager;
use super::managers::streaming_rules_manager::StreamingRulesManager;
use super::models::{
    configuration::Configuration, FilterId, FilterListMetadata, FilterListMetadataWithBody,
    FullFilterList, PullMetadataResult, UpdateResult,
};
use crate::manager::models::configuration::request_proxy_mode::RequestProxyMode;
use crate::manager::models::configuration::Locale;
use crate::manager::models::disabled_rules_raw::DisabledRulesRaw;
use crate::manager::models::filter_group::FilterGroup;
use crate::manager::models::filter_list_rules::FilterListRules;
use crate::manager::models::filter_list_rules_raw::FilterListRulesRaw;
use crate::manager::models::filter_tag::FilterTag;
use crate::manager::models::rules_count_by_filter::RulesCountByFilter;
use crate::storage::sql_generators::operator::SQLOperator;
use crate::storage::DbConnectionManager;
use crate::{
    manager::FilterListManager, ActiveRulesInfo, ActiveRulesInfoRaw, FLMError, FLMResult,
    StoredFilterMetadata,
};
use std::path::Path;

/// Default implementation for [`FilterListManager`]
pub struct FilterListManagerImpl {
    configuration: Configuration,
    pub(crate) connection_manager: DbConnectionManager,
}

impl FilterListManager for FilterListManagerImpl {
    fn new(mut configuration: Configuration) -> FLMResult<Box<Self>> {
        if configuration.app_name.is_empty() {
            return Err(FLMError::InvalidConfiguration("app_name is empty"));
        }
        if configuration.version.is_empty() {
            return Err(FLMError::InvalidConfiguration("version is empty"));
        }

        configuration.normalized();

        let connection_manager = DbConnectionManager::from_configuration(&configuration)?;
        if configuration.auto_lift_up_database {
            unsafe { connection_manager.lift_up_database()? }
        }

        Ok(Box::new(Self {
            configuration,
            connection_manager,
        }))
    }

    fn install_custom_filter_list(
        &self,
        download_url: String,
        is_trusted: bool,
        title: Option<String>,
        description: Option<String>,
    ) -> FLMResult<FullFilterList> {
        FilterManager::new().install_custom_filter_list_from_url(
            &self.connection_manager,
            &self.configuration,
            download_url,
            is_trusted,
            title,
            description,
        )
    }

    fn fetch_filter_list_metadata(&self, url: String) -> FLMResult<FilterListMetadata> {
        FilterMetadataGrabber::new().fetch_filter_list_metadata(&self.configuration, url)
    }

    fn fetch_filter_list_metadata_with_body(
        &self,
        url: String,
    ) -> FLMResult<FilterListMetadataWithBody> {
        FilterMetadataGrabber::new().fetch_filter_list_metadata_with_body(&self.configuration, url)
    }

    fn enable_filter_lists(&self, ids: Vec<FilterId>, is_enabled: bool) -> FLMResult<usize> {
        FilterManager::new().enable_filter_lists(&self.connection_manager, ids, is_enabled)
    }

    fn install_filter_lists(&self, ids: Vec<FilterId>, is_installed: bool) -> FLMResult<usize> {
        FilterManager::new().install_filter_lists(&self.connection_manager, ids, is_installed)
    }

    fn delete_custom_filter_lists(&self, ids: Vec<FilterId>) -> FLMResult<usize> {
        FilterManager::new().delete_custom_filter_lists(&self.connection_manager, ids)
    }

    fn get_all_tags(&self) -> FLMResult<Vec<FilterTag>> {
        FilterTagManager::new().get_all_tags(&self.connection_manager)
    }

    fn get_all_groups(&self) -> FLMResult<Vec<FilterGroup>> {
        FilterGroupManager::new().get_all_groups(&self.connection_manager, &self.configuration)
    }

    fn get_full_filter_list_by_id(&self, filter_id: FilterId) -> FLMResult<Option<FullFilterList>> {
        FilterManager::new().get_full_filter_list_by_id(
            &self.connection_manager,
            &self.configuration,
            Some(SQLOperator::FieldEqualValue("filter_id", filter_id.into())),
        )
    }

    fn get_stored_filters_metadata(&self) -> FLMResult<Vec<StoredFilterMetadata>> {
        FilterManager::new().get_stored_filters_metadata(
            &self.connection_manager,
            &self.configuration,
            None,
        )
    }

    fn get_stored_filter_metadata_by_id(
        &self,
        filter_id: FilterId,
    ) -> FLMResult<Option<StoredFilterMetadata>> {
        FilterManager::new().get_stored_filter_metadata_by_id(
            &self.connection_manager,
            &self.configuration,
            Some(SQLOperator::FieldEqualValue("filter_id", filter_id.into())),
        )
    }

    fn save_custom_filter_rules(&self, rules: FilterListRules) -> FLMResult<()> {
        RulesListManager::new().save_custom_filter_rules(
            &self.connection_manager,
            &self.configuration,
            rules,
        )
    }

    fn save_disabled_rules(
        &self,
        filter_id: FilterId,
        disabled_rules: Vec<String>,
    ) -> FLMResult<()> {
        RulesListManager::new().save_disabled_rules(
            &self.connection_manager,
            filter_id,
            disabled_rules,
        )
    }

    fn update_filters(
        &self,
        ignore_filters_expiration: bool,
        loose_timeout: i32,
        ignore_filters_status: bool,
    ) -> FLMResult<Option<UpdateResult>> {
        FilterUpdateManager::new().update_filters(
            &self.connection_manager,
            &self.configuration,
            ignore_filters_expiration,
            loose_timeout,
            ignore_filters_status,
        )
    }

    fn force_update_filters_by_ids(
        &self,
        ids: Vec<FilterId>,
        loose_timeout: i32,
    ) -> FLMResult<Option<UpdateResult>> {
        FilterUpdateManager::new().force_update_filters_by_ids(
            &self.connection_manager,
            &self.configuration,
            ids,
            loose_timeout,
        )
    }

    fn change_locale(&mut self, suggested_locale: Locale) -> FLMResult<bool> {
        ConfigurationUpdateManager::new().change_locale(
            &self.connection_manager,
            &mut self.configuration,
            suggested_locale,
        )
    }

    fn pull_metadata(&self) -> FLMResult<PullMetadataResult> {
        FilterUpdateManager::new().pull_metadata(&self.connection_manager, &self.configuration)
    }

    fn update_custom_filter_metadata(
        &self,
        filter_id: FilterId,
        title: String,
        is_trusted: bool,
    ) -> FLMResult<bool> {
        FilterManager::new().update_custom_filter_metadata(
            &self.connection_manager,
            filter_id,
            title,
            is_trusted,
        )
    }

    fn get_database_path(&self) -> FLMResult<String> {
        DbManager::new().get_database_path(&self.connection_manager)
    }

    fn lift_up_database(&self) -> FLMResult<()> {
        // SAFETY: Safe, as long as the call to this function does not get inside the `execute_db` closure one way or another
        // @see DbConnectionManager
        unsafe { self.connection_manager.lift_up_database() }
    }

    fn get_database_version(&self) -> FLMResult<Option<i32>> {
        DbManager::new().get_database_version(&self.connection_manager)
    }

    fn install_custom_filter_from_string(
        &self,
        download_url: String,
        last_download_time: i64,
        is_enabled: bool,
        is_trusted: bool,
        filter_body: String,
        custom_title: Option<String>,
        custom_description: Option<String>,
    ) -> FLMResult<FullFilterList> {
        FilterManager::new().install_custom_filter_from_string(
            &self.connection_manager,
            &self.configuration,
            download_url,
            last_download_time,
            is_enabled,
            is_trusted,
            filter_body,
            custom_title,
            custom_description,
        )
    }

    fn get_active_rules(&self) -> FLMResult<Vec<ActiveRulesInfo>> {
        RulesListManager::new().get_active_rules(&self.connection_manager, &self.configuration)
    }

    fn get_active_rules_raw(&self, filter_by: Vec<FilterId>) -> FLMResult<Vec<ActiveRulesInfoRaw>> {
        RulesListManager::new().get_active_rules_raw(
            &self.connection_manager,
            &self.configuration,
            filter_by,
        )
    }

    fn get_filter_rules_as_strings(
        &self,
        ids: Vec<FilterId>,
    ) -> FLMResult<Vec<FilterListRulesRaw>> {
        RulesListManager::new().get_filter_rules_as_strings(
            &self.connection_manager,
            &self.configuration,
            ids,
        )
    }

    fn save_rules_to_file_blob<P: AsRef<Path>>(
        &self,
        filter_id: FilterId,
        file_path: P,
    ) -> FLMResult<()> {
        StreamingRulesManager::new().save_rules_to_file_blob(
            &self.connection_manager,
            filter_id,
            file_path,
        )
    }

    fn get_disabled_rules(&self, ids: Vec<FilterId>) -> FLMResult<Vec<DisabledRulesRaw>> {
        RulesListManager::new().get_disabled_rules(&self.connection_manager, ids)
    }

    fn set_proxy_mode(&mut self, mode: RequestProxyMode) {
        ConfigurationUpdateManager::new().set_proxy_mode(&mut self.configuration, mode)
    }

    fn get_rules_count(&self, ids: Vec<FilterId>) -> FLMResult<Vec<RulesCountByFilter>> {
        RulesListManager::new().get_rules_count(&self.connection_manager, ids)
    }
}

#[cfg(test)]
impl FilterListManagerImpl {
    pub(crate) fn get_configuration(&self) -> &Configuration {
        &self.configuration
    }
}

#[cfg(test)]
mod tests {
    use crate::manager::managers::filter_manager::FilterManager;
    use crate::storage::entities::rules_list::rules_list_entity::RulesListEntity;
    use crate::storage::repositories::filter_repository::FilterRepository;
    use crate::storage::repositories::rules_list_repository::RulesListRepository;
    use crate::storage::repositories::Repository;
    use crate::storage::sql_generators::operator::SQLOperator;
    use crate::storage::with_transaction;
    use crate::storage::DbConnectionManager;
    use crate::test_utils::spawn_test_db_with_metadata;
    use crate::{
        string, Configuration, FilterId, FilterListManager, FilterListManagerImpl, FilterListRules,
        USER_RULES_FILTER_LIST_ID,
    };
    use chrono::{Duration, Utc};
    use rand::prelude::SliceRandom;
    use rand::thread_rng;
    use rusqlite::Connection;
    use std::fs::File;
    use std::ops::Sub;
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::{env, fs};
    use url::Url;

    #[test]
    fn test_insert_custom_filter() {
        let source = DbConnectionManager::factory_test().unwrap();
        let _ = spawn_test_db_with_metadata(&source);

        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let flm = FilterListManagerImpl::new(conf).unwrap();

        let path = fs::canonicalize("./tests/fixtures/1.txt").unwrap();

        let first_filter_url = Url::from_file_path(path).unwrap();

        let title = String::from("first title");
        let description =
            String::from("Filter that enables ad blocking on websites in Russian language.");

        let current_time = Utc::now().timestamp();

        let full_filter_list = flm
            .install_custom_filter_list(
                first_filter_url.to_string(),
                true,
                Some(title.clone()),
                None,
            )
            .unwrap();

        assert!(full_filter_list.is_custom);
        assert!(full_filter_list.is_trusted);

        assert_eq!(full_filter_list.title, title);
        assert_eq!(full_filter_list.description, description);

        assert!(full_filter_list.last_download_time >= current_time);

        assert!(full_filter_list.is_enabled);
    }

    #[test]
    fn delete_filter_lists() {
        let source = DbConnectionManager::factory_test().unwrap();
        let (_, inserted_filters) = spawn_test_db_with_metadata(&source);

        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let flm = FilterListManagerImpl::new(conf).unwrap();

        let deleted = flm
            .delete_custom_filter_lists(vec![inserted_filters.first().unwrap().filter_id.unwrap()])
            .unwrap();

        // Do not delete index filters
        assert_eq!(deleted, 0);

        let path = fs::canonicalize("./tests/fixtures/1.txt").unwrap();
        let first_filter_url = Url::from_file_path(path).unwrap();

        let title = String::from("first title");

        let full_filter_list = flm
            .install_custom_filter_list(
                first_filter_url.to_string(),
                true,
                Some(title.clone()),
                None,
            )
            .unwrap();

        let custom_was_deleted = flm
            .delete_custom_filter_lists(vec![full_filter_list.id])
            .unwrap();

        assert_eq!(custom_was_deleted, 1)
    }

    #[test]
    fn test_install_local_custom_filter() {
        let source = DbConnectionManager::factory_test().unwrap();
        let _ = spawn_test_db_with_metadata(&source);

        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let flm = FilterListManagerImpl::new(conf).unwrap();

        let title = String::from("titleeee");
        let description = String::from("dessscrriptiiiioooonnn");

        let full_filter_list = flm
            .install_custom_filter_list(
                String::new(),
                true,
                Some(title.clone()),
                Some(description.clone()),
            )
            .unwrap();

        assert!(full_filter_list.id.is_negative());
        assert_eq!(full_filter_list.title, title);
        assert_eq!(full_filter_list.description, description);
        assert_eq!(full_filter_list.is_trusted, true);
    }

    #[test]
    fn test_save_disabled_rules() {
        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let flm = FilterListManagerImpl::new(conf).unwrap();
        let source = flm.connection_manager.as_ref();

        let _ = spawn_test_db_with_metadata(source);

        let title = String::from("titleeee");
        let description = String::from("dessscrriptiiiioooonnn");

        let full_filter_list = flm
            .install_custom_filter_list(
                String::new(),
                true,
                Some(title.clone()),
                Some(description.clone()),
            )
            .unwrap();

        let disabled_rules_vec: Vec<String> = vec!["first", "second", "third"]
            .into_iter()
            .map(|str| str.to_string())
            .collect();
        let disabled_rules_string = String::from("first\nsecond\nthird");

        flm.save_disabled_rules(full_filter_list.id, disabled_rules_vec)
            .unwrap();

        let binding = source
            .execute_db(|conn: Connection| {
                let binding = RulesListRepository::new()
                    .select(
                        &conn,
                        Some(SQLOperator::FieldEqualValue(
                            "filter_id",
                            full_filter_list.id.into(),
                        )),
                    )
                    .unwrap()
                    .unwrap();

                Ok(binding)
            })
            .unwrap();

        let rules_entity = binding.first().unwrap();

        assert_eq!(rules_entity.disabled_text, disabled_rules_string);
    }

    #[test]
    fn test_install_custom_filter_from_string() {
        let source = DbConnectionManager::factory_test().unwrap();
        let _ = spawn_test_db_with_metadata(&source);

        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let flm = FilterListManagerImpl::new(conf).unwrap();

        let download_url = String::from("http://install.custom.filter.list.from.string");
        let last_download_time = Utc::now().sub(Duration::days(5));
        let filter_body = include_str!("../../tests/fixtures/small_pseudo_custom_filter.txt");

        let filter_list = flm
            .install_custom_filter_from_string(
                download_url.clone(),
                last_download_time.timestamp(),
                true,
                false,
                String::from(filter_body),
                None,
                None,
            )
            .unwrap();

        assert_eq!(filter_list.is_enabled, true);
        assert_eq!(filter_list.is_trusted, false);
        assert_eq!(filter_list.title.as_str(), "Pseudo Custom Filter Title");
        assert_eq!(
            filter_list.description.as_str(),
            "Pseudo Custom Filter Description"
        );
        assert_eq!(filter_list.version.as_str(), "2.0.91.12");
        assert_eq!(filter_list.expires, 5 * 86400);
        assert_eq!(filter_list.is_custom, true);
        assert_eq!(
            filter_list.homepage.as_str(),
            "https://github.com/AdguardTeam/AdGuardFilters"
        );
        assert_eq!(
            filter_list.last_download_time,
            last_download_time.timestamp()
        );
        assert_eq!(filter_list.time_updated, 1716903061);
        assert_eq!(filter_list.checksum.as_str(), "GQRYLu/9jKZYam7zBiCudg");
        assert_eq!(
            filter_list.license.as_str(),
            "https://github.com/AdguardTeam/AdguardFilters/blob/master/LICENSE"
        );
        assert!(filter_list.rules.unwrap().rules.len() > 0);
    }

    #[test]
    fn test_we_can_understand_aliases_fields() {
        let source = DbConnectionManager::factory_test().unwrap();
        let _ = spawn_test_db_with_metadata(&source);

        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let flm = FilterListManagerImpl::new(conf).unwrap();

        let download_url = String::from("http://install.custom.filter.list.from.string");
        let last_download_time = Utc::now().sub(Duration::days(5));
        let filter_body =
            include_str!("../../tests/fixtures/small_pseudo_custom_filter_with_aliases.txt");

        let filter_list = flm
            .install_custom_filter_from_string(
                download_url.clone(),
                last_download_time.timestamp(),
                true,
                false,
                String::from(filter_body),
                None,
                None,
            )
            .unwrap();

        assert_eq!(filter_list.time_updated, 1719230481);
        assert_eq!(
            filter_list.last_download_time,
            last_download_time.timestamp()
        );
    }

    #[test]
    fn test_we_can_select_localised_filters() {
        {
            let mut conf = Configuration::default();
            conf.locale = String::from("el");
            conf.app_name = "FlmApp".to_string();
            conf.version = "1.2.3".to_string();

            let flm = FilterListManagerImpl::new(conf).unwrap();
            let source = flm.connection_manager.as_ref();
            let _ = spawn_test_db_with_metadata(&source);

            let filter = flm.get_full_filter_list_by_id(1).unwrap().unwrap();

            assert_eq!(filter.title.as_str(), "AdGuard Ρωσικό φίλτρο");
            assert_eq!(
                filter.description.as_str(),
                "Φίλτρο που επιτρέπει τον αποκλεισμό διαφημίσεων σε ιστότοπους στη ρωσική γλώσσα."
            );
        }

        {
            let mut conf = Configuration::default();
            // Nonexistent
            conf.locale = String::from("31");
            conf.app_name = "FlmApp".to_string();
            conf.version = "1.2.3".to_string();

            let flm = FilterListManagerImpl::new(conf).unwrap();
            let source = flm.connection_manager.as_ref();
            let _ = spawn_test_db_with_metadata(&source);

            let filter = flm.get_full_filter_list_by_id(1).unwrap().unwrap();

            assert_eq!(filter.title.as_str(), "AdGuard Russian filter");
            assert_eq!(
                filter.description.as_str(),
                "Filter that enables ad blocking on websites in Russian language."
            );
        }
    }

    #[test]
    fn test_select_index_filter() {
        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let flm = FilterListManagerImpl::new(conf).unwrap();
        let source = flm.connection_manager.as_ref();

        let _ = spawn_test_db_with_metadata(&source);

        let filter = flm.get_full_filter_list_by_id(257).unwrap().unwrap();

        assert_eq!(
            filter.subscription_url.as_str(),
            "https://raw.githubusercontent.com/uBlockOrigin/uAssets/master/filters/badware.txt"
        );
        assert_eq!(
            filter.download_url.as_str(),
            "https://example.org/extension/safari/filters/257_optimized.txt"
        );
        assert!(filter.subscription_url.len() > 0);
        assert!(filter.download_url.len() > 0);
    }

    #[test]
    fn test_save_custom_filter_rules_must_update_time() {
        let source = DbConnectionManager::factory_test().unwrap();
        let _ = spawn_test_db_with_metadata(&source);

        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let flm = FilterListManagerImpl::new(conf).unwrap();

        let rules = FilterListRules {
            filter_id: USER_RULES_FILTER_LIST_ID,
            rules: vec![String::from("example.com")],
            disabled_rules: vec![],
            rules_count: 0,
        };

        // Set a new time here
        flm.save_custom_filter_rules(rules.clone()).unwrap();

        let original_time_updated = flm
            .get_full_filter_list_by_id(USER_RULES_FILTER_LIST_ID)
            .unwrap()
            .unwrap()
            .time_updated;

        // Sleep a sec, then update once again
        std::thread::sleep(core::time::Duration::from_secs(1));

        // Set another time after sleeping a sec
        flm.save_custom_filter_rules(rules).unwrap();

        let user_rules = flm
            .get_full_filter_list_by_id(USER_RULES_FILTER_LIST_ID)
            .unwrap()
            .unwrap();

        assert_ne!(user_rules.time_updated, original_time_updated);
    }

    #[test]
    fn test_guard_rewrite_user_rules_filter_by_another_filter() {
        let source = DbConnectionManager::factory_test().unwrap();
        let _ = spawn_test_db_with_metadata(&source);

        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let flm = FilterListManagerImpl::new(conf).unwrap();

        let _ = flm
            .install_custom_filter_from_string(
                String::new(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64,
                true,
                true,
                String::from("JJ"),
                Some("FILTER".to_string()),
                Some("DESC".to_string()),
            )
            .unwrap();

        let list = source
            .execute_db(|connection: Connection| {
                let list = FilterRepository::new()
                    .select(
                        &connection,
                        Some(SQLOperator::FieldEqualValue(
                            "filter_id",
                            USER_RULES_FILTER_LIST_ID.into(),
                        )),
                    )
                    .unwrap()
                    .unwrap();
                Ok(list)
            })
            .unwrap();

        assert!(!list.is_empty());
    }

    #[test]
    fn test_database_is_automatically_lifted_in_constructor() {
        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let flm = FilterListManagerImpl::new(conf).unwrap();

        let lists = FilterManager::new()
            .get_full_filter_lists(&flm.connection_manager, &flm.configuration, None)
            .unwrap();

        assert!(lists.len() > 0);
    }

    #[test]
    fn test_get_filter_rules_as_strings() {
        const TEST_FILTERS_AMOUNT: usize = 3;
        const NONEXISTENT_ID: FilterId = 450_123_456;

        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let flm = FilterListManagerImpl::new(conf).unwrap();
        let source = &flm.connection_manager;
        let (_, index_filters) = spawn_test_db_with_metadata(source);

        let filter_repo = FilterRepository::new();
        let rules_repo = RulesListRepository::new();

        let guard_id = source
            .execute_db(|connection: Connection| {
                let guard_id = filter_repo
                    .count(
                        &connection,
                        Some(SQLOperator::FieldIn(
                            "filter_id",
                            vec![NONEXISTENT_ID.into()],
                        )),
                    )
                    .unwrap();

                Ok(guard_id)
            })
            .unwrap();

        assert_eq!(guard_id, 0);

        let mut rng = thread_rng();
        let mut ids = index_filters
            .choose_multiple(&mut rng, TEST_FILTERS_AMOUNT)
            .filter_map(|filter| filter.filter_id)
            .collect::<Vec<FilterId>>();

        source
            .execute_db(|mut connection: Connection| {
                // Add rules by ids
                with_transaction(&mut connection, |transaction| {
                    let entities = ids
                        .clone()
                        .into_iter()
                        .map(|id| {
                            RulesListEntity::with_disabled_text(
                                id,
                                string!("example.com\nexample.org"),
                                string!("example.com"),
                                0,
                            )
                        })
                        .collect::<Vec<RulesListEntity>>();

                    rules_repo.insert(&transaction, &entities).unwrap();

                    Ok(())
                })
            })
            .unwrap();

        ids.push(NONEXISTENT_ID);

        let rules = flm.get_filter_rules_as_strings(ids).unwrap();

        assert_eq!(rules.len(), TEST_FILTERS_AMOUNT);
        assert!(rules
            .iter()
            .find(|rules| rules.filter_id == NONEXISTENT_ID)
            .is_none())
    }

    #[test]
    fn test_save_rules_to_file_blob() {
        let mut path = env::current_dir().unwrap();
        path.push("fixtures");
        path.push(format!(
            "test_filter_rules_{}.txt",
            Utc::now().timestamp_micros()
        ));

        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let flm = FilterListManagerImpl::new(conf).unwrap();

        {
            File::create(&path).unwrap();
        }

        let rules = FilterListRules {
            filter_id: USER_RULES_FILTER_LIST_ID,
            rules: vec![
                String::from("first"),
                String::from("second"),
                String::from("third"),
                String::from("fourth"),
                String::from("fifth"),
            ],
            disabled_rules: vec![
                String::from("second"),
                String::from("fourth"),
                String::from("second"),
            ],
            rules_count: 0,
        };

        flm.save_custom_filter_rules(rules).unwrap();

        flm.save_rules_to_file_blob(USER_RULES_FILTER_LIST_ID, &path)
            .unwrap();

        let test_string = fs::read_to_string(&path).unwrap();
        fs::remove_file(&path).unwrap();

        assert_eq!(test_string.as_str(), "first\nthird\nfifth");
    }

    #[test]
    fn test_get_disabled_rules() {
        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let flm = FilterListManagerImpl::new(conf).unwrap();

        let source = &flm.connection_manager;
        let (_, index_filters) = spawn_test_db_with_metadata(source);

        let last_filter_id = index_filters.last().unwrap().filter_id.unwrap();
        let first_filter_id = index_filters.first().unwrap().filter_id.unwrap();

        source
            .execute_db(|mut connection: Connection| {
                let rules1 = RulesListEntity::with_disabled_text(
                    last_filter_id,
                    string!("Text\nDisabled Text\n123"),
                    string!("Disabled Text\n123"),
                    0,
                );

                let rules2 = RulesListEntity::with_disabled_text(
                    first_filter_id,
                    string!("Text2\nDisabled Text2"),
                    string!("Disabled Text2"),
                    0,
                );

                let tx = connection.transaction().unwrap();
                let repo = RulesListRepository::new();

                repo.insert(&tx, vec![rules1, rules2].as_slice()).unwrap();

                tx.commit().unwrap();

                Ok(())
            })
            .unwrap();

        let actual = flm
            .get_disabled_rules(vec![first_filter_id, last_filter_id])
            .unwrap();

        assert_eq!(actual[0].text.as_str(), "Disabled Text2");
        assert_eq!(actual[1].text.as_str(), "Disabled Text\n123");
    }

    #[test]
    fn test_change_locale() {
        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let mut flm = FilterListManagerImpl::new(conf).unwrap();

        let source = &flm.connection_manager;
        spawn_test_db_with_metadata(source);

        let mut res = flm.change_locale("ru".to_string()).unwrap();
        assert!(res);

        res = flm.change_locale("ru-RU".to_string()).unwrap();
        assert!(res);

        res = flm.change_locale("ru_RU".to_string()).unwrap();
        assert!(res);

        res = flm.change_locale("ruRU".to_string()).unwrap();
        assert!(!res);
    }

    #[test]
    fn test_get_rules_count() {
        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let flm = FilterListManagerImpl::new(conf).unwrap();

        let source = &flm.connection_manager;
        spawn_test_db_with_metadata(source);

        let user_rules_count_result = 5;

        source
            .execute_db(|mut connection: Connection| {
                let rules = RulesListEntity::new(
                    USER_RULES_FILTER_LIST_ID,
                    string!(),
                    user_rules_count_result,
                );

                let tx = connection.transaction().unwrap();
                let repo = RulesListRepository::new();

                repo.insert(&tx, vec![rules].as_slice()).unwrap();

                tx.commit().unwrap();

                Ok(())
            })
            .unwrap();

        let rules_count_by_filter = flm
            .get_rules_count(vec![USER_RULES_FILTER_LIST_ID])
            .unwrap();

        assert_eq!(
            rules_count_by_filter[0].filter_id,
            USER_RULES_FILTER_LIST_ID
        );
        assert_eq!(
            rules_count_by_filter[0].rules_count,
            user_rules_count_result
        );
    }

    #[test]
    fn test_save_custom_filter_rules_must_update_rules_count() {
        let source = DbConnectionManager::factory_test().unwrap();
        spawn_test_db_with_metadata(&source);

        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let flm = FilterListManagerImpl::new(conf).unwrap();

        let rules = FilterListRules {
            filter_id: USER_RULES_FILTER_LIST_ID,
            rules: "Text\n!Text\n# Text\n\n\nText"
                .split('\n')
                .map(str::to_string)
                .collect(),
            disabled_rules: "Disabled Text".split('\n').map(str::to_string).collect(),
            rules_count: 0,
        };

        let user_rules_count_result = 2;

        flm.save_custom_filter_rules(rules).unwrap();

        let rules_count_by_filter = flm
            .get_rules_count(vec![USER_RULES_FILTER_LIST_ID])
            .unwrap();

        assert_eq!(
            rules_count_by_filter[0].filter_id,
            USER_RULES_FILTER_LIST_ID
        );
        assert_eq!(
            rules_count_by_filter[0].rules_count,
            user_rules_count_result
        );
    }

    #[test]
    fn test_install_custom_filter_sets_is_user_title_and_description_flags() {
        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();

        let flm = FilterListManagerImpl::new(conf).unwrap();

        let source = &flm.connection_manager;
        spawn_test_db_with_metadata(source);

        // sets is_user_title flag
        let installed_filter_list = flm
            .install_custom_filter_list(
                "https://filters.adtidy.org/extension/safari/filters/101_optimized.txt".to_string(),
                true,
                Some("title".to_string()),
                None,
            )
            .unwrap();

        source
            .execute_db(|conn: Connection| {
                let filters = FilterRepository::new()
                    .select(
                        &conn,
                        Some(SQLOperator::FieldEqualValue(
                            "filter_id",
                            installed_filter_list.id.into(),
                        )),
                    )
                    .unwrap()
                    .unwrap();

                assert!(filters[0].is_user_title());
                assert!(!filters[0].is_user_description());

                Ok(())
            })
            .unwrap();

        // sets is_user_description flag
        let installed_filter_list = flm
            .install_custom_filter_list(
                "https://filters.adtidy.org/extension/safari/filters/101_optimized.txt".to_string(),
                true,
                None,
                Some("description".to_string()),
            )
            .unwrap();

        source
            .execute_db(|conn: Connection| {
                let filters = FilterRepository::new()
                    .select(
                        &conn,
                        Some(SQLOperator::FieldEqualValue(
                            "filter_id",
                            installed_filter_list.id.into(),
                        )),
                    )
                    .unwrap()
                    .unwrap();

                assert!(!filters[0].is_user_title());
                assert!(filters[0].is_user_description());

                Ok(())
            })
            .unwrap();
    }

    #[test]
    fn test_update_custom_filter_metadata_sets_is_user_title_flag() {
        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();

        let flm = FilterListManagerImpl::new(conf).unwrap();

        let source = &flm.connection_manager;
        spawn_test_db_with_metadata(source);

        // sets is_user_title flag
        let installed_filter_list = flm
            .install_custom_filter_list(
                "https://filters.adtidy.org/extension/safari/filters/101_optimized.txt".to_string(),
                true,
                None,
                None,
            )
            .unwrap();

        flm.update_custom_filter_metadata(installed_filter_list.id, "title".to_string(), true)
            .unwrap();

        source
            .execute_db(|conn: Connection| {
                let filters = FilterRepository::new()
                    .select(
                        &conn,
                        Some(SQLOperator::FieldEqualValue(
                            "filter_id",
                            installed_filter_list.id.into(),
                        )),
                    )
                    .unwrap()
                    .unwrap();

                assert!(filters[0].is_user_title());

                Ok(())
            })
            .unwrap();
    }

    #[test]
    fn test_update_filters_must_not_update_title_and_description() {
        let mut conf = Configuration::default();
        conf.metadata_url = "https://filters.adtidy.org/extension/safari/filters.json".to_string();
        conf.metadata_locales_url =
            "https://filters.adtidy.org/windows/filters_i18n.json".to_string();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();

        let flm = FilterListManagerImpl::new(conf).unwrap();

        let source = &flm.connection_manager;
        spawn_test_db_with_metadata(source);

        // must not update title
        let installed_filter_list = flm
            .install_custom_filter_list(
                "https://filters.adtidy.org/extension/safari/filters/101_optimized.txt".to_string(),
                true,
                Some("title".to_string()),
                None,
            )
            .unwrap();

        flm.update_filters(false, 0, false).unwrap();

        source
            .execute_db(|conn: Connection| {
                let filters = FilterRepository::new()
                    .select(
                        &conn,
                        Some(SQLOperator::FieldEqualValue(
                            "filter_id",
                            installed_filter_list.id.into(),
                        )),
                    )
                    .unwrap()
                    .unwrap();

                assert_eq!(filters[0].title, "title");
                assert_ne!(filters[0].description, "description");

                Ok(filters)
            })
            .unwrap();

        // must not update description
        let installed_filter_list = flm
            .install_custom_filter_list(
                "https://filters.adtidy.org/extension/safari/filters/101_optimized.txt".to_string(),
                true,
                None,
                Some("description".to_string()),
            )
            .unwrap();

        flm.update_filters(false, 0, false).unwrap();

        source
            .execute_db(|conn: Connection| {
                let filters = FilterRepository::new()
                    .select(
                        &conn,
                        Some(SQLOperator::FieldEqualValue(
                            "filter_id",
                            installed_filter_list.id.into(),
                        )),
                    )
                    .unwrap()
                    .unwrap();

                assert_ne!(filters[0].title, "title");
                assert_eq!(filters[0].description, "description");

                Ok(filters)
            })
            .unwrap();
    }
}
