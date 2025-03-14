//! Default implementation for [`FilterListManager`]

use super::models::{
    configuration::Configuration, FilterId, FilterListMetadata, FilterListMetadataWithBody,
    FullFilterList, UpdateResult,
};
use crate::filters::indexes::indexes_processor::IndexesProcessor;
use crate::filters::parser::diff_updates::process_diff_path::process_diff_path;
use crate::filters::parser::filter_contents_provider::string_provider::StringProvider;
use crate::io::http::blocking_client::BlockingClient;
use crate::manager::models::active_rules_info::ActiveRulesInfo;
use crate::manager::models::configuration::request_proxy_mode::RequestProxyMode;
use crate::manager::models::configuration::{Locale, LOCALES_DELIMITER};
use crate::manager::models::disabled_rules_raw::DisabledRulesRaw;
use crate::manager::models::filter_group::FilterGroup;
use crate::manager::models::filter_list_rules::FilterListRules;
use crate::manager::models::filter_list_rules_raw::FilterListRulesRaw;
use crate::manager::models::filter_tag::FilterTag;
use crate::manager::update_filters_action::update_filters_action;
use crate::storage::blob::write_to_stream;
use crate::storage::repositories::db_metadata_repository::DBMetadataRepository;
use crate::storage::repositories::diff_updates_repository::DiffUpdateRepository;
use crate::storage::repositories::filter_group_repository::FilterGroupRepository;
use crate::storage::repositories::filter_tag_repository::FilterTagRepository;
use crate::storage::repositories::localisation::filter_localisations_repository::FilterLocalisationRepository;
use crate::storage::repositories::BulkDeleteRepository;
use crate::storage::spawn_transaction;
use crate::storage::sql_generators::operator::SQLOperator;
use crate::storage::DbConnectionManager;
use crate::utils::memory::heap;
use crate::utils::parsing::LF_BYTES_SLICE;
use crate::{
    filters::parser::metadata::parsers::expires::process_expires,
    filters::parser::metadata::KnownMetadataProperty,
    filters::parser::FilterParser,
    manager::filter_lists_builder::FullFilterListBuilder,
    manager::FilterListManager,
    storage::{
        entities::filter_entity::FilterEntity, repositories::filter_repository::FilterRepository,
        repositories::rules_list_repository::RulesListRepository, repositories::Repository,
        with_transaction,
    },
    FLMError, FLMResult, StoredFilterMetadata,
};
use chrono::{DateTime, ParseError, Utc};
use rusqlite::types::Value;
use rusqlite::{Connection, Error, Transaction};
use std::collections::HashSet;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use std::str::FromStr;

/// Default implementation for [`FilterListManager`]
pub struct FilterListManagerImpl {
    configuration: Configuration,
    pub(crate) connection_manager: DbConnectionManager,
}

impl FilterListManagerImpl {
    fn get_full_filter_lists_internal(
        &self,
        where_clause: Option<SQLOperator>,
    ) -> FLMResult<Vec<FullFilterList>> {
        self.connection_manager.execute_db(move |conn: Connection| {
            FilterRepository::new()
                .select(&conn, where_clause)
                .map_err(FLMError::from_database)?
                .map(|filters| {
                    FullFilterListBuilder::new(&self.configuration.locale)
                        .build_full_filter_lists(conn, filters)
                })
                .unwrap_or(Ok(vec![]))
        })
    }

    fn get_stored_filter_metadata_list_internal(
        &self,
        where_clause: Option<SQLOperator>,
    ) -> FLMResult<Vec<StoredFilterMetadata>> {
        self.connection_manager.execute_db(move |conn: Connection| {
            FilterRepository::new()
                .select(&conn, where_clause)
                .map_err(FLMError::from_database)?
                .map(|filters| {
                    FullFilterListBuilder::new(&self.configuration.locale)
                        .build_stored_filter_metadata_lists(conn, filters)
                })
                .unwrap_or(Ok(vec![]))
        })
    }
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

    #[allow(clippy::field_reassign_with_default)]
    fn install_custom_filter_list(
        &self,
        download_url: String,
        is_trusted: bool,
        title: Option<String>,
        description: Option<String>,
    ) -> FLMResult<FullFilterList> {
        let client = BlockingClient::new(&self.configuration)?;
        let mut parser = FilterParser::factory(&self.configuration, &client);

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
            time_slice if !time_slice.is_empty() => DateTime::from_str(time_slice)
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
        entity.last_download_time = Utc::now().timestamp();
        entity.download_url = normalized_url;
        entity.is_enabled = true;
        entity.version = parser.get_metadata(KnownMetadataProperty::Version);
        entity.is_trusted = is_trusted;
        entity.expires = expires;
        entity.homepage = parser.get_metadata(KnownMetadataProperty::Homepage);
        entity.checksum = parser.get_metadata(KnownMetadataProperty::Checksum);
        entity.license = parser.get_metadata(KnownMetadataProperty::License);

        self.connection_manager
            .execute_db(move |mut connection: Connection| {
                let (transaction, inserted_entity) =
                    spawn_transaction(&mut connection, |transaction: &Transaction| {
                        FilterRepository::new().only_insert_row(transaction, entity)
                    })
                    .map_err(FLMError::from_database)?;

                let filter_id = match inserted_entity.filter_id {
                    None => {
                        return FLMError::make_err(
                            "Cannot resolve filter_id, after saving custom filter",
                        )
                    }
                    Some(filter_id) => filter_id,
                };

                let diff_path = parser.get_metadata(KnownMetadataProperty::DiffPath);
                if !diff_path.is_empty() {
                    if let Some(entity) =
                        process_diff_path(filter_id, diff_path).map_err(FLMError::from_display)?
                    {
                        DiffUpdateRepository::new()
                            .insert(&transaction, &[entity])
                            .map_err(FLMError::from_database)?;
                    }
                }

                let rule_entity = parser.extract_rule_entity(filter_id);
                RulesListRepository::new()
                    .insert(&transaction, &[rule_entity.clone()])
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
            })
    }

    fn fetch_filter_list_metadata(&self, url: String) -> FLMResult<FilterListMetadata> {
        let client = BlockingClient::new(&self.configuration)?;
        let mut parser = FilterParser::factory(&self.configuration, &client);

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

    fn fetch_filter_list_metadata_with_body(
        &self,
        url: String,
    ) -> FLMResult<FilterListMetadataWithBody> {
        let client = BlockingClient::new(&self.configuration)?;
        let mut parser = FilterParser::factory(&self.configuration, &client);

        let download_url = parser
            .parse_from_url(&url)
            .map_err(FLMError::from_parser_error)?;

        Ok(FilterListMetadataWithBody {
            metadata: FilterListMetadata {
                title: parser.get_metadata(KnownMetadataProperty::Title),
                description: parser.get_metadata(KnownMetadataProperty::Description),
                time_updated: parser.get_metadata(KnownMetadataProperty::TimeUpdated),
                version: parser.get_metadata(KnownMetadataProperty::Version),
                homepage: parser.get_metadata(KnownMetadataProperty::Homepage),
                license: parser.get_metadata(KnownMetadataProperty::License),
                checksum: parser.get_metadata(KnownMetadataProperty::Checksum),
                url: download_url,
                rules_count: parser.get_rules_count(),
            },
            filter_body: parser.extract_rule_entity(0).text,
        })
    }

    fn enable_filter_lists(&self, ids: Vec<FilterId>, is_enabled: bool) -> FLMResult<usize> {
        self.connection_manager
            .execute_db(move |mut conn: Connection| {
                let tx = conn.transaction().map_err(FLMError::from_database)?;

                let result = FilterRepository::new()
                    .toggle_filter_lists(&tx, &ids, is_enabled)
                    .map_err(FLMError::from_database)?;

                tx.commit().map_err(FLMError::from_database)?;

                Ok(result)
            })
    }

    fn install_filter_lists(&self, ids: Vec<FilterId>, is_installed: bool) -> FLMResult<usize> {
        self.connection_manager
            .execute_db(move |mut conn: Connection| {
                let tx = conn.transaction().map_err(FLMError::from_database)?;

                let result = FilterRepository::new()
                    .toggle_is_installed(&tx, &ids, is_installed)
                    .map_err(FLMError::from_database)?;

                tx.commit().map_err(FLMError::from_database)?;

                Ok(result)
            })
    }

    fn delete_custom_filter_lists(&self, ids: Vec<FilterId>) -> FLMResult<usize> {
        self.connection_manager
            .execute_db(move |mut conn: Connection| {
                let filter_repository = FilterRepository::new();
                let rules_repository = RulesListRepository::new();

                let custom_filters = filter_repository
                    .filter_custom_filters(&conn, &ids)
                    .map_err(FLMError::from_database)?;

                with_transaction(&mut conn, move |transaction: &Transaction| {
                    let rows_deleted =
                        filter_repository.bulk_delete(transaction, &custom_filters)?;
                    rules_repository.bulk_delete(transaction, &custom_filters)?;

                    Ok(rows_deleted)
                })
            })
    }

    fn get_all_tags(&self) -> FLMResult<Vec<FilterTag>> {
        self.connection_manager.execute_db(|conn: Connection| {
            FilterTagRepository::new()
                .select_with_block(&conn, FilterTag::from)
                .map_err(FLMError::from_database)
        })
    }

    fn get_all_groups(&self) -> FLMResult<Vec<FilterGroup>> {
        self.connection_manager.execute_db(|conn: Connection| {
            FilterGroupRepository::new()
                .select_localised_with_block(&self.configuration.locale, &conn, FilterGroup::from)
                .map_err(FLMError::from_database)
        })
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

    fn get_stored_filters_metadata(&self) -> FLMResult<Vec<StoredFilterMetadata>> {
        self.get_stored_filter_metadata_list_internal(None)
    }

    fn get_stored_filter_metadata_by_id(
        &self,
        filter_id: FilterId,
    ) -> FLMResult<Option<StoredFilterMetadata>> {
        let mut vec = self.get_stored_filter_metadata_list_internal(Some(
            SQLOperator::FieldEqualValue("filter_id", filter_id.into()),
        ))?;

        Ok(if vec.is_empty() {
            None
        } else {
            Some(vec.swap_remove(0))
        })
    }

    fn save_custom_filter_rules(&self, rules: FilterListRules) -> FLMResult<()> {
        self.connection_manager
            .execute_db(move |mut conn: Connection| {
                let filter_repository = FilterRepository::new();

                let result = filter_repository
                    .select(
                        &conn,
                        Some(FilterRepository::custom_filter_with_id(rules.filter_id)),
                    )
                    .map_err(FLMError::from_database)?;

                match result {
                    Some(mut filters) if !filters.is_empty() => {
                        with_transaction(&mut conn, |transaction: &Transaction| {
                            // SAFETY: index "0" always present in this branch until condition
                            // `!filters.is_empty()` is met.
                            let filter = unsafe { filters.get_unchecked_mut(0) };

                            filter.last_update_time = Utc::now().timestamp();

                            filter_repository.insert(transaction, &filters)?;

                            RulesListRepository::new().insert(transaction, &[rules.into()])
                        })
                    }

                    _ => Err(FLMError::EntityNotFound(rules.filter_id as i64)),
                }
            })
    }

    fn save_disabled_rules(
        &self,
        filter_id: FilterId,
        disabled_rules: Vec<String>,
    ) -> FLMResult<()> {
        self.connection_manager
            .execute_db(move |mut conn: Connection| {
                let rules_list_repository = RulesListRepository::new();

                let rules_lists_count = rules_list_repository
                    .count(
                        &conn,
                        Some(SQLOperator::FieldEqualValue("filter_id", filter_id.into())),
                    )
                    .map_err(FLMError::from_database)?;

                if rules_lists_count == 0 {
                    return Err(FLMError::EntityNotFound(filter_id as i64));
                }

                with_transaction(&mut conn, |transaction: &Transaction| {
                    rules_list_repository
                        .set_disabled_rules(transaction, filter_id, disabled_rules.join("\n"))
                        .map(|_| ())
                })
            })
    }

    fn update_filters(
        &self,
        ignore_filters_expiration: bool,
        loose_timeout: i32,
        ignore_filters_status: bool,
    ) -> FLMResult<Option<UpdateResult>> {
        let result = self.connection_manager.execute_db(|conn: Connection| {
            FilterRepository::new()
                .select(&conn, None)
                .map_err(FLMError::from_database)
        })?;

        let Some(records) = result else {
            return Ok(None);
        };

        let update_result = update_filters_action(
            records,
            &self.connection_manager,
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
        let values = ids.into_iter().map(|id| id.into()).collect::<Vec<Value>>();

        let result = self.connection_manager.execute_db(|conn: Connection| {
            FilterRepository::new()
                .select(&conn, Some(SQLOperator::FieldIn("filter_id", values)))
                .map_err(FLMError::from_database)
        })?;

        let Some(records) = result else {
            return Ok(None);
        };

        let update_result = update_filters_action(
            records,
            &self.connection_manager,
            true,
            true,
            loose_timeout,
            &self.configuration,
        )?;

        Ok(Some(update_result))
    }

    fn change_locale(&mut self, suggested_locale: Locale) -> FLMResult<bool> {
        // Get saved locales
        let saved_locales = self.connection_manager.execute_db(|conn: Connection| {
            FilterLocalisationRepository::new()
                .select_available_locales(&conn)
                .map_err(FLMError::from_database)
        })?;

        // Process suggested locale
        let normalized_locale = Configuration::normalize_locale_string(&suggested_locale);
        let mut fallback_locale: Option<&str> = None;

        if let Some(position) = normalized_locale.find(LOCALES_DELIMITER) {
            fallback_locale = Some(&normalized_locale[0..position])
        }

        let mut is_found_fallback_locale = false;
        for locale in saved_locales {
            if locale == normalized_locale {
                self.configuration.locale = locale;

                return Ok(true);
            }

            if let Some(value) = fallback_locale {
                if locale == value {
                    is_found_fallback_locale = true;
                }
            }
        }

        // We didn't find exact locale, but we may use fallback
        if is_found_fallback_locale {
            if let Some(value) = fallback_locale {
                self.configuration.locale = value.to_string();

                return Ok(true);
            }
        }

        Ok(false)
    }

    fn pull_metadata(&self) -> FLMResult<()> {
        let mut processor =
            IndexesProcessor::factory(&self.connection_manager, &self.configuration)?;

        processor.sync_metadata(
            self.configuration.metadata_url.as_str(),
            self.configuration.metadata_locales_url.as_str(),
        )
    }

    fn update_custom_filter_metadata(
        &self,
        filter_id: FilterId,
        title: String,
        is_trusted: bool,
    ) -> FLMResult<bool> {
        if title.trim().is_empty() {
            return Err(FLMError::FieldIsEmpty("title"));
        }

        self.connection_manager
            .execute_db(move |mut conn: Connection| {
                let filter_repository = FilterRepository::new();

                let count = filter_repository
                    .count(
                        &conn,
                        Some(FilterRepository::custom_filter_with_id(filter_id)),
                    )
                    .map_err(FLMError::from_database)?;

                if count > 0 {
                    with_transaction(&mut conn, move |transaction: &Transaction| {
                        filter_repository.update_custom_filter_metadata(
                            transaction,
                            filter_id,
                            title.as_str(),
                            is_trusted,
                        )
                    })
                } else {
                    Err(FLMError::EntityNotFound(filter_id as i64))
                }
            })
    }

    fn get_database_path(&self) -> FLMResult<String> {
        let path = self.connection_manager.get_calculated_path();

        if path.is_absolute() {
            Ok(path.to_string_lossy().to_string())
        } else {
            path.canonicalize()
                .map_err(FLMError::from_io)
                .map(|path| path.to_string_lossy().to_string())
        }
    }

    fn lift_up_database(&self) -> FLMResult<()> {
        // SAFETY: Safe, as long as the call to this function does not get inside the `execute_db` closure one way or another
        // @see DbConnectionManager
        unsafe { self.connection_manager.lift_up_database() }
    }

    fn get_database_version(&self) -> FLMResult<Option<i32>> {
        let entity = self.connection_manager.execute_db(|conn: Connection| {
            DBMetadataRepository::read(&conn).map_err(FLMError::from_database)
        })?;

        Ok(entity.map(|e| e.version))
    }

    #[allow(clippy::field_reassign_with_default)]
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
        let client = BlockingClient::new(&self.configuration)?;
        let provider = StringProvider::new(filter_body, &client);

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
            time_slice if !time_slice.is_empty() => DateTime::from_str(time_slice)
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

        self.connection_manager
            .execute_db(move |mut connection: Connection| {
                let (transaction, inserted_entity) =
                    spawn_transaction(&mut connection, |transaction: &Transaction| {
                        FilterRepository::new().only_insert_row(transaction, entity)
                    })
                    .map_err(FLMError::from_database)?;

                let filter_id = match inserted_entity.filter_id {
                    None => {
                        return FLMError::make_err(
                            "Cannot resolve filter_id, after saving custom filter",
                        )
                    }
                    Some(filter_id) => filter_id,
                };

                let diff_path = parser.get_metadata(KnownMetadataProperty::DiffPath);
                if !diff_path.is_empty() {
                    if let Some(entity) =
                        process_diff_path(filter_id, diff_path).map_err(FLMError::from_display)?
                    {
                        DiffUpdateRepository::new()
                            .insert(&transaction, &[entity])
                            .map_err(FLMError::from_database)?;
                    }
                }

                let rule_entity = parser.extract_rule_entity(filter_id);
                RulesListRepository::new()
                    .insert(&transaction, &[rule_entity.clone()])
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
            })
    }

    fn get_active_rules(&self) -> FLMResult<Vec<ActiveRulesInfo>> {
        let (list, mut rules) = self.connection_manager.execute_db(|conn: Connection| {
            let enabled_filters = FilterRepository::new()
                .select(
                    &conn,
                    Some(SQLOperator::FieldEqualValue("is_enabled", true.into())),
                )
                .map_err(FLMError::from_database)?
                .unwrap_or_default();

            let filter_ids = enabled_filters
                .iter()
                .filter_map(|entity| entity.filter_id)
                .map(Into::into)
                .collect::<Vec<Value>>();

            let map = RulesListRepository::new()
                .select_mapped(&conn, Some(SQLOperator::FieldIn("filter_id", filter_ids)))
                .map_err(FLMError::from_database)?;

            Ok((enabled_filters, map))
        })?;

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
                                .filter(|line| {
                                    !disabled_lines
                                        .iter()
                                        .any(|line_from_disabled| line_from_disabled == line)
                                })
                                .map(ToString::to_string)
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
    }

    fn get_filter_rules_as_strings(
        &self,
        ids: Vec<FilterId>,
    ) -> FLMResult<Vec<FilterListRulesRaw>> {
        let result = self.connection_manager.execute_db(|conn: Connection| {
            RulesListRepository::new()
                .select(
                    &conn,
                    Some(SQLOperator::FieldIn(
                        "filter_id",
                        ids.into_iter().map(Into::into).collect(),
                    )),
                )
                .map_err(FLMError::from_database)
        })?;

        Ok(result
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect())
    }

    fn save_rules_to_file_blob<P: AsRef<Path>>(
        &self,
        filter_id: FilterId,
        file_path: P,
    ) -> FLMResult<()> {
        let file_already_exists = fs::metadata(&file_path).is_ok();

        let mut handler = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&file_path)
            .map_err(FLMError::from_io)?;

        self.connection_manager
            .execute_db(|connection: Connection| {
                let rules_repository = RulesListRepository::new();
                let (disabled_rules, blob) = rules_repository
                    .get_blob_handle_and_disabled_rules(&connection, filter_id)
                    .map_err(|why| match why {
                        Error::QueryReturnedNoRows => FLMError::EntityNotFound(filter_id as i64),
                        err => FLMError::from_database(err),
                    })?;

                let disabled_rules_set = disabled_rules
                    .split(|i| i == &LF_BYTES_SLICE)
                    .map(|value| value.to_vec())
                    .collect::<HashSet<Vec<u8>>>();

                write_to_stream(&mut handler, blob, disabled_rules_set)?;

                Ok(())
            })
            .map_err(|why| {
                drop(handler);
                if !file_already_exists {
                    fs::remove_file(&file_path).unwrap_or(());
                }

                why
            })
    }

    fn get_disabled_rules(&self, ids: Vec<FilterId>) -> FLMResult<Vec<DisabledRulesRaw>> {
        self.connection_manager
            .execute_db(|connection: Connection| {
                RulesListRepository::new()
                    .get_disabled_rules_by_ids(&connection, &ids)
                    .map_err(FLMError::from_database)
            })
    }

    fn set_proxy_mode(&mut self, mode: RequestProxyMode) {
        self.configuration.request_proxy_mode = mode;
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::entities::rules_list_entity::RulesListEntity;
    use crate::storage::repositories::filter_repository::FilterRepository;
    use crate::storage::repositories::rules_list_repository::RulesListRepository;
    use crate::storage::repositories::Repository;
    use crate::storage::sql_generators::operator::SQLOperator;
    use crate::storage::with_transaction;
    use crate::storage::DbConnectionManager;
    use crate::test_utils::spawn_test_db_with_metadata;
    use crate::{
        Configuration, FilterId, FilterListManager, FilterListManagerImpl, FilterListRules,
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
            "https://filters.adtidy.org/extension/safari/filters/257_optimized.txt"
        );
        assert!(filter.subscription_url.len() > 0);
        assert!(filter.download_url.len() > 0);
    }

    #[test]
    fn test_get_active_rules_with_disabled_rules() {
        let filter = include_str!("../../tests/fixtures/small_pseudo_custom_filter_rules_test.txt");

        let source = DbConnectionManager::factory_test().unwrap();
        let _ = spawn_test_db_with_metadata(&source);

        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let flm = FilterListManagerImpl::new(conf).unwrap();

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

    #[test]
    fn test_get_active_rules() {
        let source = DbConnectionManager::factory_test().unwrap();
        let _ = spawn_test_db_with_metadata(&source);

        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let flm = FilterListManagerImpl::new(conf).unwrap();
        let list_ids = flm
            .get_full_filter_lists_internal(None)
            .unwrap()
            .into_iter()
            .map(|f| f.id)
            .collect::<Vec<FilterId>>();

        flm.enable_filter_lists(list_ids, true).unwrap();

        let iter = flm
            .get_active_rules()
            .unwrap()
            .into_iter()
            // Take filters with rules
            .filter(|info| info.filter_id != crate::USER_RULES_FILTER_LIST_ID)
            .take(4);

        for filter in iter {
            assert!(filter.rules.len() > 0);
        }
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

        let lists = flm.get_full_filter_lists_internal(None).unwrap();

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
                        .map(|id| RulesListEntity {
                            filter_id: id,
                            text: "example.com\nexample.org".to_string(),
                            disabled_text: "example.com".to_string(),
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
                let rules1 = RulesListEntity {
                    filter_id: last_filter_id,
                    text: "Text\nDisabled Text\n123".to_string(),
                    disabled_text: "Disabled Text\n123".to_string(),
                };

                let rules2 = RulesListEntity {
                    filter_id: first_filter_id,
                    text: "Text2\nDisabled Text2".to_string(),
                    disabled_text: "Disabled Text2".to_string(),
                };

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
        let source = DbConnectionManager::factory_test().unwrap();
        spawn_test_db_with_metadata(&source);

        let mut conf = Configuration::default();
        conf.metadata_url = "https://filters.adtidy.org/extension/safari/filters.json".to_string();
        conf.metadata_locales_url =
            "https://filters.adtidy.org/windows/filters_i18n.json".to_string();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();

        let mut flm = FilterListManagerImpl::new(conf).unwrap();
        flm.pull_metadata().unwrap();

        let mut res = flm.change_locale("ru".to_string()).unwrap();
        assert!(res);

        res = flm.change_locale("ru-RU".to_string()).unwrap();
        assert!(res);

        res = flm.change_locale("ru_RU".to_string()).unwrap();
        assert!(res);

        res = flm.change_locale("ruRU".to_string()).unwrap();
        assert!(!res);
    }
}
