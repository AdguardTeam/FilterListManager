use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

use rusqlite::Connection;
use rusqlite::Error;

use crate::filters::parser::collectors::streaming_filter_collector::StreamingFilterCollector;
use crate::manager::models::configuration::Configuration;
use crate::storage::blob::filter_stream::FilterStream;
use crate::storage::blob::{write_to_stream, BLOB_CHUNK_SIZE};
use crate::storage::repositories::filter_includes_repository::FilterIncludesRepository;
use crate::storage::repositories::filter_repository::FilterRepository;
use crate::storage::repositories::rules_list_repository::RulesListRepository;
use crate::storage::DbConnectionManager;
use crate::utils::integrity;
use crate::utils::parsing::LF_BYTES_SLICE;
use crate::FLMError;
use crate::FLMResult;
use crate::FilterId;

/// Manager for streaming rules from storage
pub(crate) struct StreamingRulesManager;

impl StreamingRulesManager {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    /// Saves rules to file blob
    pub(crate) fn save_rules_to_file_blob<P: AsRef<Path>>(
        &self,
        connection_manager: &DbConnectionManager,
        configuration: &Configuration,
        filter_id: FilterId,
        file_path: P,
    ) -> FLMResult<()> {
        let file_already_exists = fs::metadata(&file_path).is_ok();

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file_path)
            .map_err(FLMError::from_io)?;

        let mut handler = BufWriter::with_capacity(BLOB_CHUNK_SIZE, file);

        connection_manager
            .execute_db(|conn: Connection| {
                let rules_repository = RulesListRepository::new();
                let includes_repository = FilterIncludesRepository::new();

                // 1. Get lightweight metadata without loading rules_text/disabled_rules_text
                let metadata = rules_repository
                    .get_metadata(&conn, filter_id)
                    .map_err(|why| match why {
                        Error::QueryReturnedNoRows => FLMError::EntityNotFound(filter_id as i64),
                        err => FLMError::from_database(err),
                    })?;

                // 2. Streaming integrity verification of rules_text via blob
                if let Some(ref dk) = integrity::derive_key_if_needed(configuration) {
                    if !rules_repository
                        .verify_blob_integrity_streaming(&conn, dk, &metadata)
                        .map_err(FLMError::from_database)?
                    {
                        return Err(FLMError::FilterIntegrityCheckFailed(filter_id));
                    }
                }

                // 3. Get blob handle of rules and disabled rules
                let (disabled_rules, blob) = rules_repository
                    .get_blob_handle_and_disabled_rules(&conn, filter_id)
                    .map_err(|why| match why {
                        Error::QueryReturnedNoRows => FLMError::EntityNotFound(filter_id as i64),
                        err => FLMError::from_database(err),
                    })?;

                let disabled_rules_set = disabled_rules
                    .split(|i| i == &LF_BYTES_SLICE)
                    .map(|value| value.to_vec())
                    .collect::<HashSet<Vec<u8>>>();

                if metadata.has_directives {
                    // 4a. Path WITH directives: load include metadata, verify, stream with directives

                    // Load include metadata (without body)
                    let include_metas = includes_repository
                        .get_include_metadata_for_filter(&conn, filter_id)
                        .map_err(FLMError::from_database)?;

                    // Verify integrity of each include via blob streaming
                    if let Some(ref dk) = integrity::derive_key_if_needed(configuration) {
                        for inc_meta in &include_metas {
                            if !includes_repository
                                .verify_include_blob_integrity_streaming(&conn, dk, inc_meta)
                                .map_err(FLMError::from_database)?
                            {
                                return Err(FLMError::FilterIntegrityCheckFailed(
                                    inc_meta.filter_id,
                                ));
                            }
                        }
                    }

                    // Build url -> row_id map
                    let includes_url_to_row_id: HashMap<String, i64> = include_metas
                        .into_iter()
                        .map(|m| (m.absolute_url, m.row_id))
                        .collect();

                    // Get download_url for resolving relative include paths
                    let download_url = FilterRepository::new()
                        .select_download_urls(&conn, [filter_id].iter(), 1)
                        .map_err(FLMError::from_database)?
                        .remove(&filter_id)
                        .unwrap_or_default();

                    let mut filter_stream = FilterStream::new(
                        blob,
                        &mut handler,
                        &disabled_rules_set,
                        &includes_url_to_row_id,
                        &conn,
                    );

                    StreamingFilterCollector::new(configuration)
                        .collect(&mut filter_stream, &download_url)?;
                } else {
                    // 4b. Path WITHOUT directives: simple blob streaming
                    write_to_stream(&mut handler, blob, &disabled_rules_set)?;
                }

                Ok(())
            })
            .and_then(|_| handler.flush().map_err(FLMError::from_io))
            .inspect_err(|_| {
                drop(handler);
                if !file_already_exists {
                    fs::remove_file(&file_path).unwrap_or(());
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::StreamingRulesManager;
    use crate::manager::managers::rules_list_manager::RulesListManager;
    use crate::manager::FilterListManager;
    use crate::storage::entities::filter::filter_entity::FilterEntity;
    use crate::storage::entities::filter::filter_include_entity::FilterIncludeEntity;
    use crate::storage::entities::rules_list::rules_list_entity::RulesListEntity;
    use crate::storage::repositories::filter_includes_repository::FilterIncludesRepository;
    use crate::storage::repositories::filter_repository::FilterRepository;
    use crate::storage::repositories::rules_list_repository::RulesListRepository;
    use crate::storage::repositories::Repository;
    use crate::storage::with_transaction;
    use crate::test_utils::tests_fixtures::get_tests_fixtures_path;
    use crate::{
        Configuration, FilterId, FilterListManagerImpl, FilterListRules, USER_RULES_FILTER_LIST_ID,
    };
    use chrono::Utc;
    use rusqlite::Connection;
    use std::fs;
    use std::fs::File;

    #[test]
    fn test_save_rules_to_file_blob() {
        let mut path = get_tests_fixtures_path();
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

        RulesListManager::new()
            .save_custom_filter_rules(&flm.connection_manager, flm.get_configuration(), rules)
            .unwrap();

        StreamingRulesManager::new()
            .save_rules_to_file_blob(
                &flm.connection_manager,
                flm.get_configuration(),
                USER_RULES_FILTER_LIST_ID,
                &path,
            )
            .unwrap();

        let test_string = fs::read_to_string(&path).unwrap();
        fs::remove_file(&path).unwrap();

        assert_eq!(test_string.as_str(), "first\nthird\nfifth");
    }

    #[test]
    fn test_save_rules_to_file_blob_with_includes() {
        let mut path = get_tests_fixtures_path();
        path.push(format!(
            "test_filter_rules_includes_{}.txt",
            Utc::now().timestamp_micros()
        ));

        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let flm = FilterListManagerImpl::new(conf).unwrap();

        let custom_filter_id: FilterId = -10001;
        let download_url = "https://example.com/filters/main.txt";
        let include_url = "https://example.com/filters/included.txt";

        let include_body = "included_rule_1\nincluded_rule_2";

        let rules_text = format!("rule_before\n!#include {}\nrule_after", "included.txt");

        flm.connection_manager
            .execute_db(|mut conn: Connection| {
                let mut filter = FilterEntity::default();
                filter.filter_id = Some(custom_filter_id);
                filter.download_url = download_url.to_string();
                filter.is_enabled = true;
                filter.is_installed = true;

                let _ = with_transaction(&mut conn, |tx| {
                    FilterRepository::new().insert(tx, &[filter])?;

                    let mut rules_entity =
                        RulesListEntity::make(custom_filter_id, rules_text.clone(), 3);
                    rules_entity.set_has_directives(true);
                    RulesListRepository::new().insert(tx, &[rules_entity])?;

                    let include_entity = FilterIncludeEntity::make(
                        custom_filter_id,
                        include_url.to_string(),
                        2,
                        include_body.to_string(),
                    );
                    FilterIncludesRepository::new()
                        .replace_entities_for_filters(tx, &[include_entity])
                });

                Ok(())
            })
            .unwrap();

        StreamingRulesManager::new()
            .save_rules_to_file_blob(
                &flm.connection_manager,
                flm.get_configuration(),
                custom_filter_id,
                &path,
            )
            .unwrap();

        let test_string = fs::read_to_string(&path).unwrap();
        fs::remove_file(&path).unwrap();

        assert_eq!(
            test_string.as_str(),
            "rule_before\nincluded_rule_1\nincluded_rule_2\nrule_after"
        );
    }

    #[test]
    fn test_save_rules_to_file_blob_with_includes_trailing_newline() {
        let mut path = get_tests_fixtures_path();
        path.push(format!(
            "test_filter_rules_includes_trailing_nl_{}.txt",
            Utc::now().timestamp_micros()
        ));

        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let flm = FilterListManagerImpl::new(conf).unwrap();

        let custom_filter_id: FilterId = -10003;
        let download_url = "https://example.com/filters/main3.txt";
        let include_urls = [
            "https://example.com/filters/included3_1.txt",
            "https://example.com/filters/included3_2.txt",
            "https://example.com/filters/included3_3.txt",
        ];

        let include_bodies = [
            "included_rule_1\nincluded_rule_2\n",
            "included_rule_3\nincluded_rule_4\n",
            "included_rule_5\nincluded_rule_6\n",
        ];

        flm.connection_manager
            .execute_db(|mut conn: Connection| {
                let mut filter = FilterEntity::default();
                filter.filter_id = Some(custom_filter_id);
                filter.download_url = download_url.to_string();
                filter.is_enabled = true;
                filter.is_installed = true;

                let _ = with_transaction(&mut conn, |tx| {
                    FilterRepository::new().insert(tx, &[filter])?;

                    let rules_text = "rule_before\n!#include included3_1.txt\n!#include included3_2.txt\n!#include included3_3.txt\nrule_after".to_string();
                    let mut rules_entity =
                        RulesListEntity::make(custom_filter_id, rules_text, 5);
                    rules_entity.set_has_directives(true);
                    RulesListRepository::new().insert(tx, &[rules_entity])?;

                    let include_entities = include_urls
                        .iter()
                        .zip(include_bodies.iter())
                        .map(|(include_url, include_body)| {
                            FilterIncludeEntity::make(
                                custom_filter_id,
                                (*include_url).to_string(),
                                2,
                                (*include_body).to_string(),
                            )
                        })
                        .collect::<Vec<FilterIncludeEntity>>();

                    FilterIncludesRepository::new()
                        .replace_entities_for_filters(tx, &include_entities)
                });

                Ok(())
            })
            .unwrap();

        StreamingRulesManager::new()
            .save_rules_to_file_blob(
                &flm.connection_manager,
                flm.get_configuration(),
                custom_filter_id,
                &path,
            )
            .unwrap();

        let test_string = fs::read_to_string(&path).unwrap();
        fs::remove_file(&path).unwrap();

        assert_eq!(
            test_string.as_str(),
            "rule_before\nincluded_rule_1\nincluded_rule_2\nincluded_rule_3\nincluded_rule_4\nincluded_rule_5\nincluded_rule_6\nrule_after"
        );
    }

    #[test]
    fn test_save_rules_to_file_blob_with_includes_and_disabled_rules() {
        let mut path = get_tests_fixtures_path();
        path.push(format!(
            "test_filter_rules_includes_disabled_{}.txt",
            Utc::now().timestamp_micros()
        ));

        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();
        let flm = FilterListManagerImpl::new(conf).unwrap();

        let custom_filter_id: FilterId = -10002;
        let download_url = "https://example.com/filters/main2.txt";
        let include_url = "https://example.com/filters/included2.txt";

        let include_body = "inc_rule_1\ninc_rule_2\ninc_rule_3";

        let rules_text = format!("rule_a\n!#include {}\nrule_b\nrule_c", "included2.txt");

        flm.connection_manager
            .execute_db(|mut conn: Connection| {
                let mut filter = FilterEntity::default();
                filter.filter_id = Some(custom_filter_id);
                filter.download_url = download_url.to_string();
                filter.is_enabled = true;
                filter.is_installed = true;

                let _ = with_transaction(&mut conn, |tx| {
                    FilterRepository::new().insert(tx, &[filter])?;

                    let mut rules_entity =
                        RulesListEntity::make(custom_filter_id, rules_text.clone(), 4);
                    rules_entity.set_has_directives(true);
                    rules_entity.disabled_text = "rule_a\ninc_rule_2".to_string();
                    RulesListRepository::new().insert(tx, &[rules_entity])?;

                    let include_entity = FilterIncludeEntity::make(
                        custom_filter_id,
                        include_url.to_string(),
                        3,
                        include_body.to_string(),
                    );
                    FilterIncludesRepository::new()
                        .replace_entities_for_filters(tx, &[include_entity])
                });

                Ok(())
            })
            .unwrap();

        StreamingRulesManager::new()
            .save_rules_to_file_blob(
                &flm.connection_manager,
                flm.get_configuration(),
                custom_filter_id,
                &path,
            )
            .unwrap();

        let test_string = fs::read_to_string(&path).unwrap();
        fs::remove_file(&path).unwrap();

        assert_eq!(
            test_string.as_str(),
            "inc_rule_1\ninc_rule_3\nrule_b\nrule_c"
        );
    }
}
