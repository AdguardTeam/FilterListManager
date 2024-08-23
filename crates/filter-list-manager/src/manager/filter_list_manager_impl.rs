//! Default implementation for [`FilterListManager`]
use super::models::{
    configuration::Configuration, FilterId, FilterListMetadata, FullFilterList, UpdateResult,
};
use crate::filters::indexes::indexes_processor::IndexesProcessor;
use crate::filters::parser::diff_updates::process_diff_path::process_diff_path;
use crate::filters::parser::filter_contents_provider::string_provider::StringProvider;
use crate::manager::models::active_rules_info::ActiveRulesInfo;
use crate::manager::models::configuration::{Locale, LOCALES_DELIMITER};
use crate::manager::models::filter_group::FilterGroup;
use crate::manager::models::filter_list_rules::FilterListRules;
use crate::manager::models::filter_tag::FilterTag;
use crate::manager::update_filters_action::update_filters_action;
use crate::storage::database_path_holder::DatabasePathHolder;
use crate::storage::repositories::db_metadata_repository::DBMetadataRepository;
use crate::storage::repositories::diff_updates_repository::DiffUpdateRepository;
use crate::storage::repositories::filter_group_repository::FilterGroupRepository;
use crate::storage::repositories::filter_tag_repository::FilterTagRepository;
use crate::storage::repositories::localisation::filter_localisations_repository::FilterLocalisationRepository;
use crate::storage::repositories::BulkDeleteRepository;
use crate::storage::spawn_transaction;
use crate::storage::sql_generators::operator::SQLOperator;
use crate::utils::memory::heap;
use crate::{
    filters::parser::metadata::parsers::expires::process_expires,
    filters::parser::metadata::KnownMetadataProperty,
    filters::parser::FilterParser,
    manager::full_filter_list_builder::FullFilterListBuilder,
    manager::FilterListManager,
    storage::{
        connect_using_configuration, entities::filter_entity::FilterEntity,
        repositories::filter_repository::FilterRepository,
        repositories::rules_list_repository::RulesListRepository, repositories::Repository,
        with_transaction,
    },
    FLMError, FLMResult,
};
use chrono::{DateTime, ParseError, Utc};
use rusqlite::types::Value;
use rusqlite::Transaction;
use std::str::FromStr;

/// Default implementation for [`FilterListManager`]
pub struct FilterListManagerImpl {
    configuration: Configuration,
}

impl FilterListManagerImpl {
    fn get_full_filter_lists_internal(
        &self,
        where_clause: Option<SQLOperator>,
    ) -> FLMResult<Vec<FullFilterList>> {
        let conn = connect_using_configuration(&self.configuration)?;

        let result = FilterRepository::new()
            .select(&conn, where_clause)
            .map_err(FLMError::from_database)?;

        if let Some(mut filters) = result {
            FilterLocalisationRepository::new()
                .enrich_filter_lists_with_localisation(
                    &conn,
                    &mut filters,
                    &self.configuration.locale,
                )
                .map_err(FLMError::from_database)?;

            FullFilterListBuilder::new().build(conn, filters)
        } else {
            Ok(vec![])
        }
    }
}

impl FilterListManager for FilterListManagerImpl {
    fn new(mut configuration: Configuration) -> Self {
        configuration.normalized();

        Self { configuration }
    }

    fn install_custom_filter_list(
        &self,
        download_url: String,
        is_trusted: bool,
        title: Option<String>,
        description: Option<String>,
    ) -> FLMResult<FullFilterList> {
        let mut parser = FilterParser::factory(&self.configuration);

        let normalized_url = if download_url.is_empty() {
            String::new()
        } else {
            parser
                .parse_from_url(&download_url)
                .map_err(FLMError::from_parser_error)?
        };

        let expires = match parser.get_metadata(KnownMetadataProperty::Expires) {
            value if value.is_empty() => 0i32,
            value => process_expires(value.as_str()),
        };

        let time_updated: i64 = match parser
            .get_metadata(KnownMetadataProperty::TimeUpdated)
            .as_str()
        {
            time_slice if time_slice.len() > 0 => DateTime::from_str(time_slice)
                .unwrap_or_else(|_: ParseError| Utc::now())
                .timestamp(),
            _ => Utc::now().timestamp(),
        };

        let new_title = match title {
            Some(title) => title,
            None => parser.get_metadata(KnownMetadataProperty::Title),
        };
        let new_description = match description {
            Some(description) => description,
            None => parser.get_metadata(KnownMetadataProperty::Description),
        };

        let mut entity = FilterEntity::default();
        entity.title = new_title;
        entity.description = new_description;
        entity.last_update_time = time_updated;
        entity.download_url = normalized_url;
        entity.is_enabled = true;
        entity.version = parser.get_metadata(KnownMetadataProperty::Version);
        entity.is_trusted = is_trusted;
        entity.expires = expires;
        entity.homepage = parser.get_metadata(KnownMetadataProperty::Homepage);
        entity.checksum = parser.get_metadata(KnownMetadataProperty::Checksum);
        entity.license = parser.get_metadata(KnownMetadataProperty::License);

        let mut connection = connect_using_configuration(&self.configuration)?;
        let (transaction, inserted_entity) =
            spawn_transaction(&mut connection, |transaction: &Transaction| {
                FilterRepository::new().only_insert_row(transaction, entity)
            })
            .map_err(FLMError::from_database)?;

        let filter_id = match inserted_entity.filter_id {
            None => {
                return FLMError::make_err("Cannot resolve filter_id, after saving custom filter")
            }
            Some(filter_id) => filter_id,
        };

        let diff_path = parser.get_metadata(KnownMetadataProperty::DiffPath);
        if !diff_path.is_empty() {
            if let Some(entity) =
                process_diff_path(filter_id, diff_path).map_err(FLMError::from_display)?
            {
                DiffUpdateRepository::new()
                    .insert(&transaction, vec![entity])
                    .map_err(FLMError::from_database)?;
            }
        }

        let rule_entity = parser.extract_rule_entity(filter_id);
        RulesListRepository::new()
            .insert(
                &transaction,
                // @TODO: Repository must return saved entity
                vec![rule_entity.clone()],
            )
            .map_err(FLMError::from_database)?;

        let filter_list: Option<FullFilterList> = FullFilterList::from_filter_entity(
            inserted_entity,
            vec![],
            vec![],
            Some(rule_entity.into()),
        );

        if let Some(filter) = filter_list {
            transaction.commit().map_err(FLMError::from_database)?;
            Ok(filter)
        } else {
            FLMError::make_err(format!(
                "Cannot cast inserted entity to FilterList. Url: {}",
                download_url
            ))
        }
    }

    fn fetch_filter_list_metadata(&self, url: String) -> FLMResult<FilterListMetadata> {
        let mut parser = FilterParser::factory(&self.configuration);
        let download_url = parser
            .parse_from_url(&url)
            .map_err(FLMError::from_parser_error)?;

        Ok(FilterListMetadata {
            title: parser.get_metadata(KnownMetadataProperty::Title),
            description: parser.get_metadata(KnownMetadataProperty::Description),
            time_updated: parser.get_metadata(KnownMetadataProperty::TimeUpdated),
            version: parser.get_metadata(KnownMetadataProperty::Version),
            homepage: parser.get_metadata(KnownMetadataProperty::Homepage),
            license: parser.get_metadata(KnownMetadataProperty::License),
            checksum: parser.get_metadata(KnownMetadataProperty::Checksum),
            url: download_url,
            rules_count: parser.get_rules_count(),
        })
    }

    fn enable_filter_lists(&self, ids: Vec<FilterId>, is_enabled: bool) -> FLMResult<usize> {
        let conn = connect_using_configuration(&self.configuration)?;

        FilterRepository::new()
            .toggle_filter_lists(&conn, ids, is_enabled)
            .map_err(FLMError::from_database)
    }

    fn install_filter_lists(&self, ids: Vec<FilterId>, is_installed: bool) -> FLMResult<usize> {
        let conn = connect_using_configuration(&self.configuration)?;

        FilterRepository::new()
            .toggle_is_installed(&conn, ids, is_installed)
            .map_err(FLMError::from_database)
    }

    fn delete_custom_filter_lists(&self, ids: Vec<FilterId>) -> FLMResult<usize> {
        let mut conn = connect_using_configuration(&self.configuration)?;

        let filter_repository = FilterRepository::new();
        let rules_repository = RulesListRepository::new();

        let custom_filters = filter_repository
            .filter_custom_filters(&conn, &ids)
            .map_err(FLMError::from_database)?;

        with_transaction(&mut conn, |transaction: &Transaction| {
            let rows_deleted = filter_repository.bulk_delete(transaction, &custom_filters)?;
            rules_repository.bulk_delete(transaction, &custom_filters)?;

            Ok(rows_deleted)
        })
        .map_err(FLMError::from_database)
    }

    fn get_all_tags(&self) -> FLMResult<Vec<FilterTag>> {
        let conn = connect_using_configuration(&self.configuration)?;

        FilterTagRepository::new()
            .select_with_block(&conn, FilterTag::from)
            .map_err(FLMError::from_database)
    }

    fn get_all_groups(&self) -> FLMResult<Vec<FilterGroup>> {
        let conn = connect_using_configuration(&self.configuration)?;

        FilterGroupRepository::new()
            .select_localised_with_block(&self.configuration.locale, &conn, FilterGroup::from)
            .map_err(FLMError::from_database)
    }

    fn get_full_filter_lists(&self) -> FLMResult<Vec<FullFilterList>> {
        self.get_full_filter_lists_internal(None)
    }

    fn get_full_filter_list_by_id(&self, filter_id: FilterId) -> FLMResult<Option<FullFilterList>> {
        let mut vec = self.get_full_filter_lists_internal(Some(SQLOperator::FieldEqualValue(
            "filter_id",
            filter_id.into(),
        )))?;

        Ok(if vec.is_empty() {
            None
        } else {
            Some(vec.swap_remove(0))
        })
    }

    fn save_custom_filter_rules(&self, rules: FilterListRules) -> FLMResult<()> {
        let mut conn = connect_using_configuration(&self.configuration)?;
        let count = FilterRepository::new()
            .count(
                &conn,
                Some(FilterRepository::custom_filter_with_id(rules.filter_id)),
            )
            .map_err(FLMError::from_database)?;

        if count == 0 {
            return Err(FLMError::EntityNotFound(rules.filter_id as i64));
        }

        RulesListRepository::new()
            .insert_row(&mut conn, rules.into())
            .map_err(FLMError::from_database)
    }

    fn save_disabled_rules(
        &self,
        filter_id: FilterId,
        disabled_rules: Vec<String>,
    ) -> FLMResult<()> {
        let rules_list_repository = RulesListRepository::new();

        let mut conn = connect_using_configuration(&self.configuration)?;
        let rules_lists = rules_list_repository
            .select(
                &conn,
                Some(SQLOperator::FieldEqualValue("filter_id", filter_id.into())),
            )
            .map_err(FLMError::from_database)?;

        let mut rules_list_entity = match rules_lists {
            None => return Err(FLMError::EntityNotFound(filter_id as i64)),
            Some(mut vec) => vec.swap_remove(0),
        };

        rules_list_entity.disabled_text = disabled_rules.join("\n");

        rules_list_repository
            .insert_row(&mut conn, rules_list_entity)
            .map_err(FLMError::from_database)
    }

    fn update_filters(
        &self,
        ignore_filters_expiration: bool,
        loose_timeout: i32,
        ignore_filters_status: bool,
    ) -> FLMResult<Option<UpdateResult>> {
        let conn = connect_using_configuration(&self.configuration)?;
        let filter_repository = FilterRepository::new();

        let records = match filter_repository
            .select(&conn, None)
            .map_err(FLMError::from_database)?
        {
            None => return Ok(None),
            Some(records) => records,
        };

        let update_result = update_filters_action(
            records,
            conn,
            ignore_filters_expiration,
            ignore_filters_status,
            loose_timeout,
            &self.configuration,
        )?;

        Ok(Some(update_result))
    }

    fn force_update_filters_by_ids(
        &self,
        ids: Vec<FilterId>,
        loose_timeout: i32,
    ) -> FLMResult<Option<UpdateResult>> {
        let conn = connect_using_configuration(&self.configuration)?;
        let filter_repository = FilterRepository::new();

        let values = ids.into_iter().map(|id| id.into()).collect::<Vec<Value>>();

        let result =
            filter_repository.select(&conn, Some(SQLOperator::FieldIn("filter_id", values)));

        let records = match result.map_err(FLMError::from_database)? {
            None => return Ok(None),
            Some(records) => records,
        };

        let update_result = update_filters_action(
            records,
            conn,
            true,
            true,
            loose_timeout,
            &self.configuration,
        )?;

        Ok(Some(update_result))
    }

    fn change_locale(&mut self, suggested_locale: Locale) -> FLMResult<bool> {
        // Get saved locales
        let conn = connect_using_configuration(&self.configuration)?;
        let saved_locales = FilterLocalisationRepository::new()
            .select_available_locales(&conn)
            .map_err(FLMError::from_database)?;

        // Process suggested locale
        let new_locale = Configuration::normalize_locale_string(&suggested_locale);
        let mut fallback_locale: Option<&str> = None;

        if let Some(position) = new_locale.find(LOCALES_DELIMITER) {
            fallback_locale = Some(&new_locale[0..position])
        }

        let mut is_found_fallback_locale = false;
        for locale in saved_locales {
            if locale == new_locale {
                self.configuration.locale = new_locale;

                return Ok(true);
            }

            if let Some(value) = fallback_locale {
                if value == new_locale {
                    is_found_fallback_locale = true;
                    break;
                }
            }
        }

        // We didn't find exact locale, but we may use fallback
        if is_found_fallback_locale {
            self.configuration.locale = new_locale;

            return Ok(true);
        }

        Ok(false)
    }

    fn pull_metadata(&self) -> FLMResult<()> {
        let mut processor = IndexesProcessor::factory(
            DatabasePathHolder::from_configuration(&self.configuration)?,
            self.configuration.request_timeout_ms,
        );

        processor.sync_metadata(
            &self.configuration.metadata_url,
            &self.configuration.metadata_locales_url,
        )
    }

    fn update_custom_filter_metadata(
        &self,
        filter_id: FilterId,
        title: String,
        is_trusted: bool,
    ) -> FLMResult<bool> {
        let mut conn = connect_using_configuration(&self.configuration)?;

        if title.trim().is_empty() {
            return Err(FLMError::FieldIsEmpty("title"));
        }

        let filter_repository = FilterRepository::new();

        let result = filter_repository
            .count(
                &conn,
                Some(FilterRepository::custom_filter_with_id(filter_id.into())),
            )
            .map_err(FLMError::from_database)?;

        if result > 0 {
            return with_transaction(&mut conn, move |transaction: &Transaction| {
                filter_repository.update_custom_filter_metadata(
                    &transaction,
                    filter_id,
                    title,
                    is_trusted,
                )
            })
            .map_err(FLMError::from_database);
        }

        Err(FLMError::EntityNotFound(filter_id as i64))
    }

    fn get_database_path(&self) -> FLMResult<String> {
        let database_path_holder = DatabasePathHolder::from_configuration(&self.configuration)?;

        Ok(database_path_holder
            .get_calculated_path()
            .to_string_lossy()
            .to_string())
    }

    fn get_database_version(&self) -> FLMResult<Option<i32>> {
        let conn = connect_using_configuration(&self.configuration)?;
        let entity = DBMetadataRepository::read(&conn).map_err(FLMError::from_database)?;

        Ok(entity.map(|e| e.version))
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
        let provider = StringProvider::new(filter_body);

        let mut parser = FilterParser::with_custom_provider(heap(provider), &self.configuration);

        let normalized_url = parser
            .parse_from_url(&download_url)
            .map_err(FLMError::from_parser_error)?;

        let expires = match parser.get_metadata(KnownMetadataProperty::Expires) {
            value if value.is_empty() => 0i32,
            value => process_expires(value.as_str()),
        };

        let time_updated: i64 = match parser
            .get_metadata(KnownMetadataProperty::TimeUpdated)
            .as_str()
        {
            time_slice if time_slice.len() > 0 => DateTime::from_str(time_slice)
                .unwrap_or_else(|_: ParseError| Utc::now())
                .timestamp(),
            _ => Utc::now().timestamp(),
        };

        let new_title = match custom_title {
            Some(title) => title,
            None => parser.get_metadata(KnownMetadataProperty::Title),
        };
        let new_description = match custom_description {
            Some(description) => description,
            None => parser.get_metadata(KnownMetadataProperty::Description),
        };

        let mut entity = FilterEntity::default();
        entity.title = new_title;
        entity.description = new_description;
        entity.last_update_time = time_updated;
        entity.last_download_time = last_download_time;
        entity.download_url = normalized_url;
        entity.is_enabled = is_enabled;
        entity.version = parser.get_metadata(KnownMetadataProperty::Version);
        entity.is_trusted = is_trusted;
        entity.expires = expires;
        entity.homepage = parser.get_metadata(KnownMetadataProperty::Homepage);
        entity.checksum = parser.get_metadata(KnownMetadataProperty::Checksum);
        entity.license = parser.get_metadata(KnownMetadataProperty::License);

        let mut connection = connect_using_configuration(&self.configuration)?;
        let (transaction, inserted_entity) =
            spawn_transaction(&mut connection, |transaction: &Transaction| {
                FilterRepository::new().only_insert_row(transaction, entity)
            })
            .map_err(FLMError::from_database)?;

        let filter_id = match inserted_entity.filter_id {
            None => {
                return FLMError::make_err("Cannot resolve filter_id, after saving custom filter")
            }
            Some(filter_id) => filter_id,
        };

        let diff_path = parser.get_metadata(KnownMetadataProperty::DiffPath);
        if !diff_path.is_empty() {
            if let Some(entity) =
                process_diff_path(filter_id, diff_path).map_err(FLMError::from_display)?
            {
                DiffUpdateRepository::new()
                    .insert(&transaction, vec![entity])
                    .map_err(FLMError::from_database)?;
            }
        }

        let rule_entity = parser.extract_rule_entity(filter_id);
        RulesListRepository::new()
            .insert(&transaction, vec![rule_entity.clone()])
            .map_err(FLMError::from_database)?;

        let filter_list: Option<FullFilterList> = FullFilterList::from_filter_entity(
            inserted_entity,
            vec![],
            vec![],
            Some(rule_entity.into()),
        );

        if let Some(filter) = filter_list {
            transaction.commit().map_err(FLMError::from_database)?;
            Ok(filter)
        } else {
            FLMError::make_err(format!(
                "Cannot cast inserted entity to FilterList. Url: {}",
                download_url
            ))
        }
    }

    fn get_active_rules(&self) -> FLMResult<Vec<ActiveRulesInfo>> {
        let conn = connect_using_configuration(&self.configuration)?;

        let result = FilterRepository::new()
            .select(
                &conn,
                Some(SQLOperator::FieldEqualValue("is_enabled", true.into())),
            )
            .map_err(FLMError::from_database)?;

        if let Some(list) = result {
            let filter_ids = list
                .iter()
                .filter(|entity| entity.filter_id.is_some())
                .map(|entity| entity.filter_id.unwrap().into())
                .collect::<Vec<Value>>();

            let mut rules = RulesListRepository::new()
                .select_mapped(&conn, Some(SQLOperator::FieldIn("filter_id", filter_ids)))
                .map_err(FLMError::from_database)?;

            Ok(list
                .into_iter()
                .flat_map(|filter_entity: FilterEntity| {
                    if let Some(filter_id) = filter_entity.filter_id {
                        if let Some(rule_entity) = rules.remove(&filter_id) {
                            if rule_entity.filter_id == filter_id {
                                let disabled_lines =
                                    rule_entity.disabled_text.lines().collect::<Vec<&str>>();

                                // Make a difference of rule_entity.text from rule_entity.disabled_text
                                let filtered_rules = rule_entity
                                    .text
                                    .lines()
                                    .flat_map(|line| {
                                        disabled_lines
                                            .iter()
                                            .find(|line_from_disabled| line_from_disabled != &&line)
                                    })
                                    .map(|line| line.to_string())
                                    .collect::<Vec<String>>();

                                return Some(ActiveRulesInfo {
                                    filter_id,
                                    group_id: filter_entity.group_id,
                                    is_trusted: filter_entity.is_trusted,
                                    rules: filtered_rules,
                                });
                            }
                        }
                    }

                    None
                })
                .collect())
        } else {
            Ok(vec![])
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::repositories::rules_list_repository::RulesListRepository;
    use crate::storage::sql_generators::operator::SQLOperator;
    use crate::test_utils::{do_with_tests_helper, spawn_test_db_with_metadata};
    use crate::{Configuration, FilterListManager, FilterListManagerImpl};
    use chrono::{Duration, Utc};
    use std::fs;
    use std::ops::Sub;

    #[test]
    fn test_insert_custom_filter() {
        do_with_tests_helper(|mut helper| {
            helper.increment_postfix();
        });

        let _ = spawn_test_db_with_metadata();

        let flm = FilterListManagerImpl::new(Configuration::default());

        let path = fs::canonicalize("./tests/fixtures/1.txt").unwrap();

        let mut first_filter_url = String::from("file:///");
        first_filter_url += path.to_str().unwrap();

        let title = String::from("first title");
        let description =
            String::from("Filter that enables ad blocking on websites in Russian language.");

        let full_filter_list = flm
            .install_custom_filter_list(first_filter_url, true, Some(title.clone()), None)
            .unwrap();

        assert!(full_filter_list.is_custom);
        assert!(full_filter_list.is_trusted);

        assert_eq!(full_filter_list.title, title);
        assert_eq!(full_filter_list.description, description);

        assert!(full_filter_list.is_enabled);
    }

    #[test]
    fn delete_filter_lists() {
        do_with_tests_helper(|mut helper| {
            helper.increment_postfix();
        });

        let (_, _, inserted_filters) = spawn_test_db_with_metadata();

        let flm = FilterListManagerImpl::new(Configuration::default());
        let deleted = flm
            .delete_custom_filter_lists(vec![inserted_filters.first().unwrap().filter_id.unwrap()])
            .unwrap();

        // Do not delete index filters
        assert_eq!(deleted, 0);

        let path = fs::canonicalize("./tests/fixtures/1.txt").unwrap();

        let mut first_filter_url = String::from("file:///");
        first_filter_url += path.to_str().unwrap();

        let title = String::from("first title");

        let full_filter_list = flm
            .install_custom_filter_list(first_filter_url, true, Some(title.clone()), None)
            .unwrap();

        let custom_was_deleted = flm
            .delete_custom_filter_lists(vec![full_filter_list.id])
            .unwrap();

        assert_eq!(custom_was_deleted, 1)
    }

    #[test]
    fn test_install_local_custom_filter() {
        do_with_tests_helper(|mut helper| {
            helper.increment_postfix();
        });

        let _ = spawn_test_db_with_metadata();
        let flm = FilterListManagerImpl::new(Configuration::default());

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
        do_with_tests_helper(|mut helper| {
            helper.increment_postfix();
        });

        let (_, conn, _) = spawn_test_db_with_metadata();
        let flm = FilterListManagerImpl::new(Configuration::default());

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
        let rules_entity = binding.first().unwrap();

        assert_eq!(rules_entity.disabled_text, disabled_rules_string);
    }

    #[test]
    fn test_install_custom_filter_from_string() {
        do_with_tests_helper(|mut helper| {
            helper.increment_postfix();
        });

        let _ = spawn_test_db_with_metadata();
        let flm = FilterListManagerImpl::new(Configuration::default());

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
        do_with_tests_helper(|mut helper| {
            helper.increment_postfix();
        });

        let _ = spawn_test_db_with_metadata();
        let flm = FilterListManagerImpl::new(Configuration::default());

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
        do_with_tests_helper(|mut helper| {
            helper.increment_postfix();
        });

        let _ = spawn_test_db_with_metadata();

        {
            let mut conf = Configuration::default();
            conf.locale = String::from("el");

            let flm = FilterListManagerImpl::new(conf);
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

            let flm = FilterListManagerImpl::new(conf);
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
        do_with_tests_helper(|mut helper| {
            helper.increment_postfix();
        });

        let _ = spawn_test_db_with_metadata();

        let flm = FilterListManagerImpl::new(Configuration::default());
        let filter = flm.get_full_filter_list_by_id(257).unwrap().unwrap();

        assert_eq!(
            filter.subscription_url.as_str(),
            "https://raw.githubusercontent.com/uBlockOrigin/uAssets/master/filters/badware.txt"
        );
        assert_eq!(
            filter.download_url.as_str(),
            "https://filters.adtidy.org/extension/safari/filters/257_optimized.txt"
        );
        assert!(filter.subscription_url.len() > 0);
        assert!(filter.download_url.len() > 0);
    }

    #[test]
    fn test_get_active_rules() {
        let filter = include_str!("../../tests/fixtures/small_pseudo_custom_filter_rules_test.txt");
        do_with_tests_helper(|mut helper| {
            helper.increment_postfix();
        });

        let _ = spawn_test_db_with_metadata();

        let flm = FilterListManagerImpl::new(Configuration::default());

        let new_filter = flm
            .install_custom_filter_from_string(
                String::from("https://i-dont-ca.re"),
                1970,
                true,
                true,
                String::from(filter),
                None,
                None,
            )
            .unwrap();

        // Last line rule has any copies in file.
        // They all must be excluded from get_active_rules output
        let new_filter_rules = new_filter.rules.unwrap();
        let disabled_rule = new_filter_rules.rules.last().unwrap().to_owned();

        flm.save_disabled_rules(new_filter.id, vec![disabled_rule])
            .unwrap();

        // There must be only two records. UserRules and new_filter
        let active_rules = flm.get_active_rules().unwrap();

        assert_eq!(active_rules.len(), 2);

        let actual_filter = active_rules.last().unwrap();

        assert_eq!(actual_filter.filter_id, new_filter.id);
        assert!(actual_filter.is_trusted);
        assert_eq!(actual_filter.group_id, new_filter.group_id);

        assert_eq!(actual_filter.rules.len(), 32);
        assert_eq!(new_filter_rules.rules.len(), 38);
    }
}
