use crate::filters::indexes::entities::IndexEntity;
use crate::filters::parser::diff_updates::batch_patches_container::BatchPatchesContainer;
use crate::filters::parser::diff_updates::process_diff_path::process_diff_path;
use crate::filters::parser::filter_contents_provider::diff_path_provider::DiffPathProvider;
use crate::filters::parser::metadata::parsers::expires::process_expires;
use crate::filters::parser::metadata::KnownMetadataProperty;
use crate::filters::parser::FilterParser;
use crate::io::fetch_by_schemes::fetch_json_by_scheme;
use crate::io::get_scheme;
use crate::io::http::blocking_client::BlockingClient;
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

    let mut has_at_least_one_index_filter = false;
    let filter_ids = records
        .iter()
        .filter_map(|filter| {
            if !filter.is_custom() {
                has_at_least_one_index_filter = true;
            }

            filter.filter_id
        })
        .collect::<Vec<FilterId>>();

    let (mut diff_updates_map, mut rules_map, mut disabled_rules_map) = db_connection_manager
        .execute_db(|conn: Connection| {
            let diff_updates_map = diff_updates_repository
                .select_map(&conn, &filter_ids)
                .map_err(FLMError::from_database)?;

            let (rules_map, disabled_rules_map) = rule_list_repository
                .select_rules_maps(&conn, &filter_ids)
                .map_err(FLMError::from_database)?;

            Ok((diff_updates_map, rules_map, disabled_rules_map))
        })?;

    let shared_http_client = BlockingClient::new(configuration)?;

    // Parsers with successful filter downloads
    let mut successful_parsers_with_result: Vec<(FilterId, FilterParser)> =
        Vec::with_capacity(filter_entities.len() / 2);

    let last_index_filter_versions = get_latest_filters_versions(
        has_at_least_one_index_filter,
        &shared_http_client,
        configuration.metadata_url.as_str(),
    )?;

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

        if let Some(new_version) = last_index_filter_versions.get(&filter_id) {
            if !filter.version.is_empty() && &filter.version == new_version &&
                // Extra spike, 'cause we may already have metadata, but not the filters
                // This stupid spike is here, 'cause we MIGHT fall in situation,
                // when filter metadata.version is provided, it is up-to-date, BUT empty rules object is saved
                rules_map.get(&filter_id).map(|old_rules| !old_rules.is_empty()).unwrap_or_default()
            {
                continue;
            }
        }

        let build_parser_result = build_parser(
            ignore_filters_expiration,
            filter_id,
            configuration,
            &mut diff_updates_map,
            current_time,
            &mut rules_map,
            &batch_patches_container,
            &filter,
            &shared_http_client,
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

        if !filter.is_custom() {
            parser.should_skip_checksum_validation(false);
        }

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

            filter.last_update_time = match parser
                .get_metadata(KnownMetadataProperty::TimeUpdated)
                .as_str()
            {
                time_slice if !time_slice.is_empty() => DateTime::from_str(time_slice)
                    .unwrap_or_else(|_: ParseError| Utc::now())
                    .timestamp(),

                _ => Utc::now().timestamp(),
            };

            // Should update `parsed info` only for custom filters
            if filter.is_custom() {
                filter.homepage = parser.get_metadata(KnownMetadataProperty::Homepage);
                filter.title = parser.get_metadata(KnownMetadataProperty::Title);
                filter.description = parser.get_metadata(KnownMetadataProperty::Description);
            }

            filter.version = parser.get_metadata(KnownMetadataProperty::Version);
            filter.license = parser.get_metadata(KnownMetadataProperty::License);
            filter.checksum = parser.get_metadata(KnownMetadataProperty::Checksum);

            filter_entities.push(filter);

            let mut new_rule = parser.extract_rule_entity(filter_id);
            new_rule.disabled_text = disabled_rules_map
                .remove(&new_rule.filter_id)
                .unwrap_or_default();

            rule_entities.push(new_rule);
        }

        with_transaction(&mut conn, |transaction: &Transaction| {
            filter_repository.insert(transaction, &filter_entities)?;
            diff_updates_repository.insert(transaction, &diff_path_entities)?;
            rule_list_repository.insert(transaction, &rule_entities)
        })?;

        let new_rules_map = rule_entities
            .into_iter()
            .fold(HashMap::new(), |mut acc, rule| {
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
fn build_parser<'h: 'p, 'p>(
    ignore_filters_expiration: bool,
    filter_id: FilterId,
    configuration: &Configuration,
    diff_updates_map: &mut DiffUpdatesMap,
    current_time: i64,
    rules_map: &mut MapFilterIdOnRulesString,
    batch_patches_container: &Rc<RefCell<BatchPatchesContainer>>,
    filter: &FilterEntity,
    shared_http_client: &'h BlockingClient,
) -> FLMResult<Option<FilterParser<'p>>> {
    let expires_duration = configuration.resolve_right_expires_value(filter.expires) as i64;

    let ready_for_full_update = current_time > filter.last_download_time + expires_duration;

    // We force full filter update through http or filter is ready for full update
    if ignore_filters_expiration || ready_for_full_update {
        return Ok(Some(FilterParser::factory(
            configuration,
            shared_http_client,
        )));
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
                        shared_http_client,
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

/// Gets latest filters versions
fn get_latest_filters_versions(
    has_at_least_one_index_filter: bool,
    shared_http_client: &BlockingClient,
    metadata_url: &str,
) -> FLMResult<HashMap<FilterId, String>> {
    let mut last_index_filter_versions = HashMap::new();
    if has_at_least_one_index_filter {
        let index = fetch_json_by_scheme::<IndexEntity>(
            metadata_url,
            get_scheme(metadata_url).into(),
            shared_http_client,
        )?;

        for entity in index.filters {
            last_index_filter_versions.insert(entity.filterId, entity.version);
        }
    }

    Ok(last_index_filter_versions)
}

#[cfg(test)]
mod tests {
    use super::update_filters_action;
    use crate::storage::entities::filter_entity::FilterEntity;
    use crate::storage::entities::rules_list_entity::RulesListEntity;
    use crate::storage::repositories::filter_repository::FilterRepository;
    use crate::storage::repositories::rules_list_repository::RulesListRepository;
    use crate::storage::repositories::Repository;
    use crate::storage::DbConnectionManager;
    use crate::test_utils::{tests_path, RAIIFile};
    use crate::{Configuration, FilterId};
    use chrono::Utc;
    use rusqlite::Connection;
    use std::{env, thread};
    use url::Url;

    #[allow(clippy::field_reassign_with_default)]
    #[test]
    fn test_update_filters_action() {
        let timestamp = Utc::now().timestamp_micros();

        let source = DbConnectionManager::factory_test().unwrap();

        unsafe { source.lift_up_database().unwrap() }

        let mut _files_guard = vec![];

        // This creates filter in the fixtures folder
        let mut write_rules = |rules: RulesListEntity| -> (Url, RulesListEntity) {
            let mut path = env::current_dir().unwrap();
            path.push("fixtures");
            path.push(format!(
                "{}_update_filters_action_{}_{:?}.txt",
                timestamp,
                &rules.filter_id,
                thread::current().id()
            ));

            let filter_url = Url::from_file_path(&path).unwrap();

            _files_guard.push(RAIIFile::new(&path, rules.text.as_str()));

            (filter_url, rules)
        };

        let first_filter_id = 1;
        let second_filter_id = 2;
        let third_filter_id = -100011;
        let fourth_filter_id = -100012;
        let fifth_filter_id = 5;

        let mut filter1 = FilterEntity::default();
        filter1.filter_id = Some(first_filter_id);
        filter1.group_id = 1;
        filter1.is_enabled = true;
        filter1.title = String::from("Filter1");
        let (download_url1, rules1) = write_rules(RulesListEntity {
            filter_id: first_filter_id,
            text: "Filter1\nNonFilter1".to_string(),
            disabled_text: "NonFilter1".to_string(),
        });
        filter1.download_url = download_url1.to_string();

        let mut filter2 = FilterEntity::default();
        filter2.group_id = 1;
        filter2.filter_id = Some(second_filter_id);
        filter2.title = String::from("Filter2");
        let (download_url2, rules2) = write_rules(RulesListEntity {
            filter_id: second_filter_id,
            text: "Filter2\nNonFilter2".to_string(),
            disabled_text: "NonFilter2".to_string(),
        });
        filter2.download_url = download_url2.to_string();

        let mut filter3 = FilterEntity::default();
        filter3.filter_id = Some(third_filter_id);
        filter3.title = String::from("Filter3");
        filter3.is_enabled = true;
        let (download_url3, rules3) = write_rules(RulesListEntity {
            filter_id: third_filter_id,
            text: "Filter3\nNonFilter3".to_string(),
            disabled_text: "NonFilter3".to_string(),
        });
        filter3.download_url = download_url3.to_string();

        // This filter shouldn't be updated
        let mut filter4 = FilterEntity::default();
        filter4.filter_id = Some(fourth_filter_id);
        filter4.title = String::from("Filter4");
        filter4.last_download_time = Utc::now().timestamp();
        filter4.last_update_time = Utc::now().timestamp();
        filter4.is_enabled = true;
        let (download_url4, rules4) = write_rules(RulesListEntity {
            filter_id: fourth_filter_id,
            text: "Filter4\nNonFilter4".to_string(),
            disabled_text: "NonFilter4".to_string(),
        });
        filter4.download_url = download_url4.to_string();

        // Special case - not yet installed filter
        let mut filter5 = FilterEntity::default();
        filter5.filter_id = Some(fifth_filter_id);
        filter5.group_id = 1;
        filter5.title = String::from("Filter5");
        filter5.is_enabled = true;
        // Another special case: Rules is not installed, but version already came from metadata.
        // Expected behaviour: rules must be installed
        filter5.version = String::from("2.0.1");
        let (download_url5, rules5) = write_rules(RulesListEntity {
            filter_id: fifth_filter_id,
            text: "\n!Version: 2.0.1\nFilter5\nNonFilter5".to_string(),
            // Disabled rules weren't written that way.
            // It will be mistaken to put here non-empty string
            disabled_text: String::new(),
        });
        filter5.download_url = download_url5.to_string();

        let filters_repo = FilterRepository::new();
        let rules_repo = RulesListRepository::new();

        let installed_filters = source
            .execute_db(|mut connection: Connection| {
                let tx = connection.transaction().unwrap();
                filters_repo
                    .insert(&tx, &vec![filter1, filter2, filter3, filter4, filter5])
                    .unwrap();

                rules_repo
                    .insert(
                        &tx,
                        &vec![
                            rules1.clone(),
                            rules2.clone(),
                            rules3.clone(),
                            rules4.clone(),
                        ],
                    )
                    .unwrap();

                tx.commit().unwrap();

                let installed_filters = filters_repo
                    .select_filters_except_bootstrapped(&connection)
                    .unwrap()
                    .unwrap();

                let rules_lists_count = rules_repo
                    .select_rules_except_bootstrapped(&connection)
                    .unwrap()
                    .unwrap()
                    .len();

                assert_eq!(rules_lists_count, 4);

                Ok(installed_filters)
            })
            .unwrap();

        assert_eq!(installed_filters.len(), 5);

        // Write new data into all filters
        let (_, new_rules1) = write_rules(RulesListEntity {
            filter_id: first_filter_id,
            text: "Filter1_new\nNonFilter1_new".to_string(),
            disabled_text: rules1.disabled_text,
        });

        let _ = write_rules(RulesListEntity {
            filter_id: second_filter_id,
            text: "Filter2_new\nNonFilter2_new".to_string(),
            disabled_text: rules2.clone().disabled_text,
        });

        let (_, new_rules3) = write_rules(RulesListEntity {
            filter_id: third_filter_id,
            text: "Filter3_new\nNonFilter3_new".to_string(),
            disabled_text: rules3.disabled_text,
        });

        let _ = write_rules(RulesListEntity {
            filter_id: fourth_filter_id,
            text: "Filter4_new\nNonFilter4_new".to_string(),
            disabled_text: rules4.clone().disabled_text,
        });

        let mut conf = Configuration::default();
        conf.metadata_url = Url::from_file_path(tests_path("fixtures/filters.json"))
            .unwrap()
            .to_string();

        let result =
            update_filters_action(installed_filters, &source, false, false, 0, &conf).unwrap();

        let updated_ids = result
            .updated_list
            .iter()
            .map(|item| item.id)
            .collect::<Vec<FilterId>>();

        // No errors here
        assert!(result.filters_errors.is_empty());

        // Filters 1,3,5
        assert_eq!(updated_ids.len(), 3);
        // First filter is enabled and ready for update by expires
        assert!(updated_ids
            .iter()
            .find(|id| id == &&first_filter_id)
            .is_some());
        // Third filter is enabled and ready for update by expires
        assert!(updated_ids
            .iter()
            .find(|id| id == &&third_filter_id)
            .is_some());
        // Fifth filter is just installed
        assert!(updated_ids
            .iter()
            .find(|id| id == &&fifth_filter_id)
            .is_some());

        source
            .execute_db(|connection: Connection| {
                let rules = rules_repo
                    .select_rules_except_bootstrapped(&connection)
                    .unwrap()
                    .unwrap();

                let first_filter = rules
                    .iter()
                    .find(|rules| rules.filter_id == first_filter_id)
                    .unwrap();

                let second_filter = rules
                    .iter()
                    .find(|rules| rules.filter_id == second_filter_id)
                    .unwrap();

                let third_filter = rules
                    .iter()
                    .find(|rules| rules.filter_id == third_filter_id)
                    .unwrap();

                let fourth_filter = rules
                    .iter()
                    .find(|rules| rules.filter_id == fourth_filter_id)
                    .unwrap();

                let fifth_filter = rules
                    .iter()
                    .find(|rules| rules.filter_id == fifth_filter_id)
                    .unwrap();

                // These will be updated
                assert_eq!(first_filter, &new_rules1);
                assert_eq!(third_filter, &new_rules3);

                // These weren't update
                assert_eq!(second_filter, &rules2);
                assert_eq!(fourth_filter, &rules4);

                // This will be written as new during the update
                assert_eq!(fifth_filter, &rules5);

                Ok(())
            })
            .unwrap()
    }

    #[allow(clippy::field_reassign_with_default)]
    #[test]
    fn test_force_update_filters_action() {
        let timestamp = Utc::now().timestamp_micros();

        let source = DbConnectionManager::factory_test().unwrap();

        unsafe { source.lift_up_database().unwrap() }

        let mut _files_guard = vec![];

        // This creates filter in the fixtures folder
        let mut write_rules = |rules: RulesListEntity| -> (Url, RulesListEntity) {
            let mut path = env::current_dir().unwrap();
            path.push("fixtures");
            path.push(format!(
                "{}_update_filters_action_{}_{:?}.txt",
                timestamp,
                &rules.filter_id,
                thread::current().id()
            ));

            let filter_url = Url::from_file_path(&path).unwrap();

            _files_guard.push(RAIIFile::new(&path, rules.text.as_str()));

            (filter_url, rules)
        };

        let first_filter_id = 1;
        let second_filter_id = 2;
        let third_filter_id = -100011;
        let fourth_filter_id = -100012;
        let fifth_filter_id = 5;

        let mut filter1 = FilterEntity::default();
        filter1.filter_id = Some(first_filter_id);
        filter1.group_id = 1;
        filter1.is_enabled = true;
        filter1.title = String::from("Filter1");
        let (download_url1, rules1) = write_rules(RulesListEntity {
            filter_id: first_filter_id,
            text: "Filter1\nNonFilter1".to_string(),
            disabled_text: "NonFilter1".to_string(),
        });
        filter1.download_url = download_url1.to_string();

        let mut filter2 = FilterEntity::default();
        filter2.group_id = 1;
        filter2.filter_id = Some(second_filter_id);
        filter2.title = String::from("Filter2");
        let (download_url2, rules2) = write_rules(RulesListEntity {
            filter_id: second_filter_id,
            text: "Filter2\nNonFilter2".to_string(),
            disabled_text: "NonFilter2".to_string(),
        });
        filter2.download_url = download_url2.to_string();

        let mut filter3 = FilterEntity::default();
        filter3.filter_id = Some(third_filter_id);
        filter3.title = String::from("Filter3");
        filter3.is_enabled = true;
        let (download_url3, rules3) = write_rules(RulesListEntity {
            filter_id: third_filter_id,
            text: "Filter3\nNonFilter3".to_string(),
            disabled_text: "NonFilter3".to_string(),
        });
        filter3.download_url = download_url3.to_string();

        // This filter shouldn't be updated
        let mut filter4 = FilterEntity::default();
        filter4.filter_id = Some(fourth_filter_id);
        filter4.title = String::from("Filter4");
        filter4.last_download_time = Utc::now().timestamp();
        filter4.last_update_time = Utc::now().timestamp();
        filter4.is_enabled = true;
        let (download_url4, rules4) = write_rules(RulesListEntity {
            filter_id: fourth_filter_id,
            text: "Filter4\nNonFilter4".to_string(),
            disabled_text: "NonFilter4".to_string(),
        });
        filter4.download_url = download_url4.to_string();

        // Special case - not yet installed filter
        let mut filter5 = FilterEntity::default();
        filter5.filter_id = Some(fifth_filter_id);
        filter5.group_id = 1;
        filter5.title = String::from("Filter5");
        filter5.is_enabled = true;
        // Another special case: Rules is not installed, but version already came from metadata.
        // Expected behaviour: rules must be installed
        filter5.version = String::from("2.0.1");
        let (download_url5, rules5) = write_rules(RulesListEntity {
            filter_id: fifth_filter_id,
            text: "\n!Version: 2.0.1\nFilter5\nNonFilter5".to_string(),
            // Disabled rules weren't written that way.
            // It will be mistaken to put here non-empty string
            disabled_text: String::new(),
        });
        filter5.download_url = download_url5.to_string();

        let filters_repo = FilterRepository::new();
        let rules_repo = RulesListRepository::new();

        let installed_filters = source
            .execute_db(|mut connection: Connection| {
                let tx = connection.transaction().unwrap();
                filters_repo
                    .insert(&tx, &vec![filter1, filter2, filter3, filter4, filter5])
                    .unwrap();

                rules_repo
                    .insert(
                        &tx,
                        &vec![
                            rules1.clone(),
                            rules2.clone(),
                            rules3.clone(),
                            rules4.clone(),
                        ],
                    )
                    .unwrap();

                tx.commit().unwrap();

                let installed_filters = filters_repo
                    .select_filters_except_bootstrapped(&connection)
                    .unwrap()
                    .unwrap();

                let rules_lists_count = rules_repo
                    .select_rules_except_bootstrapped(&connection)
                    .unwrap()
                    .unwrap()
                    .len();

                assert_eq!(rules_lists_count, 4);

                Ok(installed_filters)
            })
            .unwrap();

        assert_eq!(installed_filters.len(), 5);

        // Write new data into all filters
        let (_, new_rules1) = write_rules(RulesListEntity {
            filter_id: first_filter_id,
            text: "Filter1_new\nNonFilter1_new".to_string(),
            disabled_text: rules1.clone().disabled_text,
        });

        let (_, new_rules2) = write_rules(RulesListEntity {
            filter_id: second_filter_id,
            text: "Filter2_new\nNonFilter2_new".to_string(),
            disabled_text: rules2.clone().disabled_text,
        });

        let (_, new_rules3) = write_rules(RulesListEntity {
            filter_id: third_filter_id,
            text: "Filter3_new\nNonFilter3_new".to_string(),
            disabled_text: rules3.clone().disabled_text,
        });

        let (_, new_rules4) = write_rules(RulesListEntity {
            filter_id: fourth_filter_id,
            text: "Filter4_new\nNonFilter4_new".to_string(),
            disabled_text: rules4.clone().disabled_text,
        });

        let mut conf = Configuration::default();
        conf.metadata_url = Url::from_file_path(tests_path("fixtures/filters.json"))
            .unwrap()
            .to_string();

        let result =
            update_filters_action(installed_filters, &source, true, true, 0, &conf).unwrap();

        let updated_ids = result
            .updated_list
            .iter()
            .map(|item| item.id)
            .collect::<Vec<FilterId>>();

        // No errors here
        assert!(result.filters_errors.is_empty());

        // All filters were updated
        assert_eq!(updated_ids.len(), 5);

        // First filter is enabled and ready for update by expires
        assert!(updated_ids
            .iter()
            .find(|id| id == &&first_filter_id)
            .is_some());
        // Third filter is enabled and ready for update by expires
        assert!(updated_ids
            .iter()
            .find(|id| id == &&third_filter_id)
            .is_some());
        // Fifth filter is just installed
        assert!(updated_ids
            .iter()
            .find(|id| id == &&fifth_filter_id)
            .is_some());

        source
            .execute_db(|connection: Connection| {
                let rules = rules_repo
                    .select_rules_except_bootstrapped(&connection)
                    .unwrap()
                    .unwrap();

                let first_filter = rules
                    .iter()
                    .find(|rules| rules.filter_id == first_filter_id)
                    .unwrap();

                let second_filter = rules
                    .iter()
                    .find(|rules| rules.filter_id == second_filter_id)
                    .unwrap();

                let third_filter = rules
                    .iter()
                    .find(|rules| rules.filter_id == third_filter_id)
                    .unwrap();

                let fourth_filter = rules
                    .iter()
                    .find(|rules| rules.filter_id == fourth_filter_id)
                    .unwrap();

                let fifth_filter = rules
                    .iter()
                    .find(|rules| rules.filter_id == fifth_filter_id)
                    .unwrap();

                // These will be updated
                assert_eq!(first_filter, &new_rules1);
                assert_eq!(third_filter, &new_rules3);

                // These wasn't update
                assert_eq!(second_filter, &new_rules2);
                assert_eq!(fourth_filter, &new_rules4);

                // This will be written as new during the update
                assert_eq!(fifth_filter, &rules5);

                Ok(())
            })
            .unwrap()
    }
}
