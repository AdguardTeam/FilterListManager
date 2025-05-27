use std::str::FromStr;

use chrono::DateTime;
use chrono::ParseError;
use chrono::Utc;
use rusqlite::Connection;
use rusqlite::Transaction;

use crate::filters::parser::diff_updates::process_diff_path::process_diff_path;
use crate::filters::parser::filter_contents_provider::string_provider::StringProvider;
use crate::filters::parser::metadata::parsers::expires::process_expires;
use crate::filters::parser::metadata::KnownMetadataProperty;
use crate::filters::parser::FilterParser;
use crate::io::http::blocking_client::BlockingClient;
use crate::manager::filter_lists_builder::FullFilterListBuilder;
use crate::storage::entities::filter::filter_entity::FilterEntity;
use crate::storage::entities::rules_list::rules_list_entity::RulesListEntity;
use crate::storage::repositories::diff_updates_repository::DiffUpdateRepository;
use crate::storage::repositories::filter_repository::FilterRepository;
use crate::storage::repositories::rules_list_repository::RulesListRepository;
use crate::storage::repositories::BulkDeleteRepository;
use crate::storage::repositories::Repository;
use crate::storage::spawn_transaction;
use crate::storage::sql_generators::operator::SQLOperator;
use crate::storage::with_transaction;
use crate::storage::DbConnectionManager;
use crate::utils::memory::heap;
use crate::Configuration;
use crate::FLMError;
use crate::FLMResult;
use crate::FilterId;
use crate::FullFilterList;
use crate::StoredFilterMetadata;

/// Manager for filter logic
pub(crate) struct FilterManager;

impl FilterManager {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    /// Installs custom filter from string
    pub(crate) fn install_custom_filter_from_string(
        &self,
        connection_manager: &DbConnectionManager,
        configuration: &Configuration,
        download_url: String,
        last_download_time: i64,
        is_enabled: bool,
        is_trusted: bool,
        filter_body: String,
        title: Option<String>,
        description: Option<String>,
    ) -> FLMResult<FullFilterList> {
        let client = BlockingClient::new(configuration)?;
        let provider = StringProvider::new(filter_body, &client);

        let mut parser = FilterParser::with_custom_provider(heap(provider), configuration);

        let normalized_url = parser
            .parse_from_url(&download_url)
            .map_err(FLMError::from_parser_error)?;

        let (diff_path, mut entity, rule_entity): (String, FilterEntity, RulesListEntity) = self
            .prepare_custom_filter_list_to_install(
                &mut parser,
                normalized_url,
                is_trusted,
                title,
                description,
            )?;

        entity.last_download_time = last_download_time;
        entity.is_enabled = is_enabled;

        let (inserted_entity, rule_entity): (FilterEntity, RulesListEntity) = self
            .install_custom_filter_list(
                connection_manager,
                diff_path,
                entity,
                rule_entity,
                parser.is_directives_encountered(),
            )?;

        let full_filter_list: FullFilterList =
            self.make_full_filter_list(download_url, inserted_entity, rule_entity)?;

        Ok(full_filter_list)
    }

    /// Installs custom filter list from url
    pub(crate) fn install_custom_filter_list_from_url(
        &self,
        connection_manager: &DbConnectionManager,
        configuration: &Configuration,
        download_url: String,
        is_trusted: bool,
        title: Option<String>,
        description: Option<String>,
    ) -> FLMResult<FullFilterList> {
        let client = BlockingClient::new(configuration)?;
        let mut parser = FilterParser::factory(configuration, &client);

        let normalized_url = if download_url.is_empty() {
            String::new()
        } else {
            parser
                .parse_from_url(&download_url)
                .map_err(FLMError::from_parser_error)?
        };

        let (diff_path, entity, rule_entity): (String, FilterEntity, RulesListEntity) = self
            .prepare_custom_filter_list_to_install(
                &mut parser,
                normalized_url,
                is_trusted,
                title,
                description,
            )?;

        let (inserted_entity, rule_entity): (FilterEntity, RulesListEntity) = self
            .install_custom_filter_list(
                connection_manager,
                diff_path,
                entity,
                rule_entity,
                parser.is_directives_encountered(),
            )?;

        let full_filter_list: FullFilterList =
            self.make_full_filter_list(download_url, inserted_entity, rule_entity)?;

        Ok(full_filter_list)
    }
}

impl FilterManager {
    /// Deletes custom filter lists
    pub(crate) fn delete_custom_filter_lists(
        &self,
        connection_manager: &DbConnectionManager,
        ids: Vec<FilterId>,
    ) -> FLMResult<usize> {
        let rows_updated: usize = connection_manager.execute_db(move |mut conn: Connection| {
            let filter_repository = FilterRepository::new();
            let rules_repository = RulesListRepository::new();

            let custom_filters = filter_repository
                .filter_custom_filters(&conn, &ids)
                .map_err(FLMError::from_database)?;

            with_transaction(&mut conn, move |tx: &Transaction| {
                let rows_deleted = filter_repository.bulk_delete(tx, &custom_filters)?;
                rules_repository.bulk_delete(tx, &custom_filters)?;

                Ok(rows_deleted)
            })
        })?;

        Ok(rows_updated)
    }

    /// Enables or disabled filter lists
    pub(crate) fn enable_filter_lists(
        &self,
        connection_manager: &DbConnectionManager,
        ids: Vec<FilterId>,
        is_enabled: bool,
    ) -> FLMResult<usize> {
        let rows_updated: usize = connection_manager.execute_db(move |mut conn: Connection| {
            let tx = conn.transaction().map_err(FLMError::from_database)?;

            let rows_updated = FilterRepository::new()
                .toggle_filter_lists(&tx, &ids, is_enabled)
                .map_err(FLMError::from_database)?;

            tx.commit().map_err(FLMError::from_database)?;

            Ok(rows_updated)
        })?;

        Ok(rows_updated)
    }

    /// Gets full filter list by id
    pub(crate) fn get_full_filter_list_by_id(
        &self,
        connection_manager: &DbConnectionManager,
        configuration: &Configuration,
        where_clause: Option<SQLOperator>,
    ) -> FLMResult<Option<FullFilterList>> {
        let mut vec: Vec<FullFilterList> =
            self.get_full_filter_lists_inner(connection_manager, configuration, where_clause)?;

        Ok(if vec.is_empty() {
            None
        } else {
            Some(vec.swap_remove(0))
        })
    }

    /// Gets stored filter metadata by id
    pub(crate) fn get_stored_filter_metadata_by_id(
        &self,
        connection_manager: &DbConnectionManager,
        configuration: &Configuration,
        where_clause: Option<SQLOperator>,
    ) -> FLMResult<Option<StoredFilterMetadata>> {
        let mut vec = self.get_stored_filter_metadata_list_inner(
            connection_manager,
            configuration,
            where_clause,
        )?;

        Ok(if vec.is_empty() {
            None
        } else {
            Some(vec.swap_remove(0))
        })
    }

    /// Gets stored filter metadata
    pub(crate) fn get_stored_filters_metadata(
        &self,
        connection_manager: &DbConnectionManager,
        configuration: &Configuration,
        where_clause: Option<SQLOperator>,
    ) -> FLMResult<Vec<StoredFilterMetadata>> {
        let stored_filters_metadata: Vec<StoredFilterMetadata> = self
            .get_stored_filter_metadata_list_inner(
                connection_manager,
                configuration,
                where_clause,
            )?;

        Ok(stored_filters_metadata)
    }

    /// Toggles is_installed flag for filter lists
    pub(crate) fn install_filter_lists(
        &self,
        connection_manager: &DbConnectionManager,
        ids: Vec<FilterId>,
        is_installed: bool,
    ) -> FLMResult<usize> {
        let rows_updated: usize = connection_manager.execute_db(move |mut conn: Connection| {
            let tx = conn.transaction().map_err(FLMError::from_database)?;

            let rows_updated = FilterRepository::new()
                .toggle_is_installed(&tx, &ids, is_installed)
                .map_err(FLMError::from_database)?;

            tx.commit().map_err(FLMError::from_database)?;

            Ok(rows_updated)
        })?;

        Ok(rows_updated)
    }

    /// Updates custom filter metadata
    pub(crate) fn update_custom_filter_metadata(
        &self,
        connection_manager: &DbConnectionManager,
        filter_id: FilterId,
        title: String,
        is_trusted: bool,
    ) -> FLMResult<bool> {
        if title.trim().is_empty() {
            return Err(FLMError::FieldIsEmpty("title"));
        }

        let is_updated: bool = connection_manager.execute_db(move |mut conn: Connection| {
            let filter_repository = FilterRepository::new();

            let count = filter_repository
                .count(
                    &conn,
                    Some(FilterRepository::custom_filter_with_id(filter_id)),
                )
                .map_err(FLMError::from_database)?;

            if count > 0 {
                let is_title_set_by_user = !title.is_empty();

                let is_updated: bool =
                    with_transaction(&mut conn, move |transaction: &Transaction| {
                        filter_repository.update_user_metadata_for_custom_filter(
                            transaction,
                            filter_id,
                            title.as_str(),
                            is_trusted,
                            is_title_set_by_user,
                        )
                    })?;

                Ok(is_updated)
            } else {
                Err(FLMError::EntityNotFound(filter_id as i64))
            }
        })?;

        Ok(is_updated)
    }
}

#[cfg(test)]
impl FilterManager {
    /// Gets full filter lists
    pub(crate) fn get_full_filter_lists(
        &self,
        connection_manager: &DbConnectionManager,
        configuration: &Configuration,
        where_clause: Option<SQLOperator>,
    ) -> FLMResult<Vec<FullFilterList>> {
        let full_filter_lists: Vec<FullFilterList> =
            self.get_full_filter_lists_inner(connection_manager, configuration, where_clause)?;

        Ok(full_filter_lists)
    }
}

impl FilterManager {
    /// Gets full filter lists by filter_list_builder
    fn get_full_filter_lists_inner(
        &self,
        connection_manager: &DbConnectionManager,
        configuration: &Configuration,
        where_clause: Option<SQLOperator>,
    ) -> FLMResult<Vec<FullFilterList>> {
        let full_filter_list: Vec<FullFilterList> =
            connection_manager.execute_db(move |conn: Connection| {
                FilterRepository::new()
                    .select(&conn, where_clause)
                    .map_err(FLMError::from_database)?
                    .map(|filters| {
                        FullFilterListBuilder::new(&configuration.locale)
                            .build_full_filter_lists(conn, filters)
                    })
                    .unwrap_or(Ok(vec![]))
            })?;

        Ok(full_filter_list)
    }

    /// Gets stored filter metadata by filter_list_builder
    fn get_stored_filter_metadata_list_inner(
        &self,
        connection_manager: &DbConnectionManager,
        configuration: &Configuration,
        where_clause: Option<SQLOperator>,
    ) -> FLMResult<Vec<StoredFilterMetadata>> {
        let stored_filter_metadata: Vec<StoredFilterMetadata> =
            connection_manager.execute_db(move |conn: Connection| {
                FilterRepository::new()
                    .select(&conn, where_clause)
                    .map_err(FLMError::from_database)?
                    .map(|filters| {
                        FullFilterListBuilder::new(&configuration.locale)
                            .build_stored_filter_metadata_lists(conn, filters)
                    })
                    .unwrap_or(Ok(vec![]))
            })?;

        Ok(stored_filter_metadata)
    }

    /// Prepares custom filter list for installation
    fn prepare_custom_filter_list_to_install(
        &self,
        parser: &mut FilterParser,
        normalized_url: String,
        is_trusted: bool,
        title: Option<String>,
        description: Option<String>,
    ) -> FLMResult<(String, FilterEntity, RulesListEntity)> {
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

        let (processed_title, is_user_title) = match title {
            Some(title_candidate) if title_candidate.len() > 0 => (title_candidate, true),
            _ => (parser.get_metadata(KnownMetadataProperty::Title), false),
        };

        let (processed_description, is_user_description) = match description {
            Some(description_candidate) if description_candidate.len() > 0 => {
                (description_candidate, true)
            }
            _ => (
                parser.get_metadata(KnownMetadataProperty::Description),
                false,
            ),
        };

        let mut entity = FilterEntity::default();
        entity.title = processed_title;
        entity.description = processed_description;
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
        entity.set_is_user_title(is_user_title);
        entity.set_is_user_description(is_user_description);

        let diff_path = parser.get_metadata(KnownMetadataProperty::DiffPath);

        let rule_entity = parser.extract_rule_entity(0);

        Ok((diff_path, entity, rule_entity))
    }

    /// Installs custom filter lists
    fn install_custom_filter_list(
        &self,
        connection_manager: &DbConnectionManager,
        diff_path: String,
        entity: FilterEntity,
        mut rule_entity: RulesListEntity,
        is_directives_encountered: bool,
    ) -> FLMResult<(FilterEntity, RulesListEntity)> {
        let (inserted_entity, rule_entity): (FilterEntity, RulesListEntity) = connection_manager
            .execute_db(move |mut conn: Connection| {
                let (tx, inserted_entity) = spawn_transaction(&mut conn, |tx: &Transaction| {
                    FilterRepository::new().only_insert_row(tx, entity)
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

                if !diff_path.is_empty() && !is_directives_encountered {
                    if let Some(entity) =
                        process_diff_path(filter_id, diff_path).map_err(FLMError::from_display)?
                    {
                        DiffUpdateRepository::new()
                            .insert(&tx, &[entity])
                            .map_err(FLMError::from_database)?;
                    }
                }

                rule_entity.filter_id = filter_id;
                RulesListRepository::new()
                    .insert(&tx, &[rule_entity.clone()])
                    .map_err(FLMError::from_database)?;

                tx.commit().map_err(FLMError::from_database)?;

                Ok((inserted_entity, rule_entity))
            })?;

        Ok((inserted_entity, rule_entity))
    }

    /// Completes custom filter list installation
    fn make_full_filter_list(
        &self,
        download_url: String,
        inserted_entity: FilterEntity,
        rule_entity: RulesListEntity,
    ) -> FLMResult<FullFilterList> {
        let filter_list: Option<FullFilterList> = FullFilterList::from_filter_entity(
            inserted_entity,
            vec![],
            vec![],
            Some(rule_entity.into()),
        );

        if let Some(filter) = filter_list {
            Ok(filter)
        } else {
            FLMError::make_err(format!(
                "Cannot cast inserted entity to FilterList. Url: {}",
                download_url
            ))
        }
    }
}
