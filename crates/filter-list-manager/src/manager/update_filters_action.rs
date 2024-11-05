use crate::filters::parser::diff_updates::batch_patches_container::BatchPatchesContainer;
use crate::filters::parser::diff_updates::process_diff_path::process_diff_path;
use crate::filters::parser::filter_contents_provider::diff_path_provider::DiffPathProvider;
use crate::filters::parser::metadata::parsers::expires::process_expires;
use crate::filters::parser::metadata::KnownMetadataProperty;
use crate::filters::parser::FilterParser;
use crate::manager::filter_lists_builder::FullFilterListBuilder;
use crate::manager::models::update_result::UpdateFilterError;
use crate::manager::models::UpdateResult;
use crate::storage::entities::diff_update_entity::DiffUpdateEntity;
use crate::storage::entities::filter_entity::FilterEntity;
use crate::storage::entities::rules_list_entity::RulesListEntity;
use crate::storage::repositories::diff_updates_repository::{DiffUpdateRepository, DiffUpdatesMap};
use crate::storage::repositories::filter_repository::FilterRepository;
use crate::storage::repositories::rules_list_repository::{
    MapFilterIdOnRulesString, RulesListRepository,
};
use crate::storage::repositories::Repository;
use crate::storage::sql_generators::operator::SQLOperator;
use crate::storage::with_transaction;
use crate::storage::DbConnectionManager;
use crate::utils::memory::heap;
use crate::{Configuration, FLMError, FLMResult, FilterId, FilterParserError};
use chrono::{DateTime, ParseError, Utc};
use rusqlite::types::Value;
use rusqlite::{Connection, Transaction};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;
use std::time::Instant;

/// Tries to update passed filters
pub(super) fn update_filters_action(
    records: Vec<FilterEntity>,
    db_connection_manager: &DbConnectionManager,
    ignore_filters_expiration: bool,
    ignore_filters_status: bool,
    loose_timeout: i32,
    configuration: &Configuration,
) -> FLMResult<UpdateResult> {
    let filter_repository = FilterRepository::new();
    let rule_list_repository = RulesListRepository::new();
    let diff_updates_repository = DiffUpdateRepository::new();

    let current_time = Utc::now().timestamp();
    let mut filter_entities: Vec<FilterEntity> = Vec::with_capacity(records.len());
    let mut rule_entities: Vec<RulesListEntity> = Vec::with_capacity(records.len());

    let start_time = Instant::now();
    let is_use_timeout = loose_timeout > 0;

    let mut update_result = UpdateResult {
        updated_list: vec![],
        remaining_filters_count: 0,
        filters_errors: vec![],
    };

    // region Diff updates
    let filter_ids = records
        .iter()
        .filter(|filter| filter.filter_id.is_some())
        .map(|filter| filter.filter_id.unwrap())
        .collect::<Vec<FilterId>>();

    let (mut diff_updates_map, mut old_rules_map) =
        db_connection_manager.execute_db(|conn: Connection| {
            let diff_updates_map = diff_updates_repository
                .select_map(&conn, &filter_ids)
                .map_err(FLMError::from_database)?;

            let filter_ids_for_diff_updates =
                diff_updates_map.keys().cloned().collect::<Vec<FilterId>>();

            let rules_map = rule_list_repository
                .select_rules_string_map(&conn, &filter_ids_for_diff_updates)
                .map_err(FLMError::from_database)?;

            Ok((diff_updates_map, rules_map))
        })?;
    // endregion

    // Parsers with successful filter downloads
    let mut successful_parsers_with_result: Vec<(FilterId, FilterParser)> =
        Vec::with_capacity(filter_entities.len() / 2);

    // Put here processed_filters
    let mut diff_path_entities: Vec<DiffUpdateEntity> = vec![];
    let rows_count = records.len();
    let batch_patches_container = BatchPatchesContainer::factory();
    for (index, filter) in records.into_iter().enumerate() {
        if !ignore_filters_status && !filter.is_enabled {
            continue;
        }

        if filter.download_url.is_empty() {
            continue;
        }

        let filter_id = match filter.filter_id {
            Some(filter_id) => filter_id,
            None => {
                update_result.filters_errors.push(UpdateFilterError {
                    filter_id: 0,
                    message: format!(
                        "Cannot get filter contents from database for filter with url: {}",
                        filter.download_url
                    ),
                });

                continue;
            }
        };

        let build_parser_result = build_parser(
            ignore_filters_expiration,
            filter_id,
            configuration,
            &mut diff_updates_map,
            current_time,
            &mut old_rules_map,
            &batch_patches_container,
            &filter,
        );

        let mut parser = match build_parser_result {
            Ok(Some(parser)) => parser,
            Ok(None) => {
                // Not ready for update
                continue;
            }
            Err(err) => {
                // An error occurred
                update_result.filters_errors.push(UpdateFilterError {
                    filter_id,
                    message: err.to_string(),
                });

                continue;
            }
        };

        if let Err(err) = parser.parse_from_url(&filter.download_url) {
            // NoContent means update just unavailable yet
            if err.error != FilterParserError::NoContent {
                update_result.filters_errors.push(UpdateFilterError {
                    filter_id,
                    message: err.to_string(),
                });
            }

            continue;
        }

        successful_parsers_with_result.push((filter_id, parser));

        if is_use_timeout && start_time.elapsed().as_secs() > loose_timeout as u64 {
            // Set count of unprocessed filters
            update_result.remaining_filters_count = (rows_count - index + 1) as i32;
            break;
        }
    }

    let successful_filter_ids = successful_parsers_with_result
        .iter()
        .map(|entity| entity.0.into())
        .collect::<Vec<Value>>();

    update_result.updated_list = db_connection_manager.execute_db(|mut conn: Connection| {
        // Retrieve filters once again, cuz properties may have changed
        let mut new_filters_map = filter_repository
            .select_mapped(
                &conn,
                Some(SQLOperator::FieldIn("filter_id", successful_filter_ids)),
            )
            .map_err(FLMError::from_database)?;

        // Gets from db second time, because filters may have changes
        for (filter_id, mut parser) in successful_parsers_with_result {
            let Some(mut filter) = new_filters_map.remove(&filter_id) else {
                update_result.filters_errors.push(UpdateFilterError {
                    filter_id,
                    message: format!(
                        "Filter with id: {} is gone from database while updating",
                        filter_id
                    ),
                });

                continue;
            };

            let expires = match parser.get_metadata(KnownMetadataProperty::Expires) {
                value if value.is_empty() => 0i32,
                value => process_expires(value.as_str()),
            };

            let diff_path = parser.get_metadata(KnownMetadataProperty::DiffPath);
            if !diff_path.is_empty() {
                match process_diff_path(filter_id, diff_path) {
                    Ok(Some(entity)) => diff_path_entities.push(entity),
                    Err(why) => update_result.filters_errors.push(UpdateFilterError {
                        filter_id,
                        message: why.to_string(),
                    }),
                    _ => {}
                }
            }

            filter.expires = expires;
            filter.last_download_time = current_time;

            // Do not change (description and title) for custom filters,
            // 'cause user may set this info manually
            // So we don't need change these values for registry filters

            filter.last_update_time = match parser
                .get_metadata(KnownMetadataProperty::TimeUpdated)
                .as_str()
            {
                time_slice if time_slice.len() > 0 => DateTime::from_str(time_slice)
                    .unwrap_or_else(|_: ParseError| Utc::now())
                    .timestamp(),

                _ => Utc::now().timestamp(),
            };

            // Should update `parsed info` only for custom filters
            if filter.is_custom() {
                filter.homepage = parser.get_metadata(KnownMetadataProperty::Homepage);
            }

            // TODO: Spike, until we implement streaming parsing and/or index downloading before update;
            let new_version = parser.get_metadata(KnownMetadataProperty::Version);
            if !filter.version.is_empty() && filter.version == new_version {
                continue;
            }

            filter.version = new_version;
            filter.license = parser.get_metadata(KnownMetadataProperty::License);
            filter.checksum = parser.get_metadata(KnownMetadataProperty::Checksum);

            filter_entities.push(filter);
            rule_entities.push(parser.extract_rule_entity(filter_id));
        }

        with_transaction(&mut conn, |transaction: &Transaction| {
            filter_repository.insert(transaction, &filter_entities)?;
            diff_updates_repository.insert(transaction, &diff_path_entities)?;
            rule_list_repository.insert(transaction, &rule_entities)
        })?;

        let new_rules_map = rule_entities
            .into_iter()
            .fold(HashMap::new(), |mut acc, mut rule| {
                rule.disabled_text = old_rules_map.remove(&rule.filter_id).unwrap_or_default();
                acc.insert(rule.filter_id, rule);
                acc
            });

        let mut builder = FullFilterListBuilder::new(&configuration.locale);
        builder.set_rules_map(new_rules_map);

        builder.build_full_filter_lists(conn, filter_entities)
    })?;

    Ok(update_result)
}

/// Parsers factory logic
#[inline]
fn build_parser(
    ignore_filters_expiration: bool,
    filter_id: FilterId,
    configuration: &Configuration,
    diff_updates_map: &mut DiffUpdatesMap,
    current_time: i64,
    rules_map: &mut MapFilterIdOnRulesString,
    batch_patches_container: &Rc<RefCell<BatchPatchesContainer>>,
    filter: &FilterEntity,
) -> FLMResult<Option<FilterParser>> {
    let expires_duration = configuration.resolve_right_expires_value(filter.expires) as i64;

    let ready_for_full_update = current_time > filter.last_download_time + expires_duration;

    // We force full filter update through http or filter is ready for full update
    if ignore_filters_expiration || ready_for_full_update {
        return Ok(Some(FilterParser::factory(configuration)));
    }

    if let Some(diff_update_info) = diff_updates_map.remove(&filter_id) {
        // if we have diff updates, we should check next_update time
        if current_time > diff_update_info.next_check_time {
            return match rules_map.remove(&filter_id) {
                Some(rules) => {
                    // All set. We can apply diff update
                    let provider = DiffPathProvider::new(
                        diff_update_info.next_path,
                        rules,
                        Rc::clone(batch_patches_container),
                    );

                    Ok(Some(FilterParser::with_custom_provider(
                        heap(provider),
                        configuration,
                    )))
                }
                None => FLMError::make_err(
                    "Strange behaviour. Cannot get filter contents from database",
                ),
            };
        }
    }

    Ok(None)
}
