use rusqlite::types::Value;
use rusqlite::Connection;
use std::collections::HashSet;

use crate::filters::indexes::indexes_processor::IndexesProcessor;
use crate::manager::update_filters_action::update_filters_action;
use crate::storage::repositories::filter_repository::FilterRepository;
use crate::storage::repositories::rules_list_repository::RulesListRepository;
use crate::storage::sql_generators::operator::SQLOperator;
use crate::storage::DbConnectionManager;
use crate::Configuration;
use crate::FLMError;
use crate::FLMResult;
use crate::FilterId;
use crate::UpdateResult;

pub(crate) struct FilterUpdateManager;

impl FilterUpdateManager {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    /// Forced updates filters by ids
    pub(crate) fn force_update_filters_by_ids(
        &self,
        connection_manager: &DbConnectionManager,
        configuration: &Configuration,
        ids: Vec<FilterId>,
        loose_timeout: i32,
    ) -> FLMResult<Option<UpdateResult>> {
        let values = ids
            .clone()
            .into_iter()
            .map(Into::into)
            .collect::<Vec<Value>>();

        let (filters_with_directives, result) =
            connection_manager.execute_db(|conn: Connection| {
                let filters_with_directives = self.gets_filters_with_directives(&conn, &ids)?;
                let result = FilterRepository::new()
                    .select(&conn, Some(SQLOperator::FieldIn("filter_id", values)))
                    .map_err(FLMError::from_database)?;

                Ok((filters_with_directives, result))
            })?;

        let Some(records) = result else {
            return Ok(None);
        };

        let update_result = update_filters_action(
            records,
            filters_with_directives,
            &connection_manager,
            true,
            true,
            loose_timeout,
            &configuration,
        )?;

        Ok(Some(update_result))
    }

    /// Pulls metadata
    pub(crate) fn pull_metadata(
        &self,
        connection_manager: &DbConnectionManager,
        configuration: &Configuration,
    ) -> FLMResult<()> {
        let mut processor = IndexesProcessor::factory(connection_manager, configuration)?;

        let _ = processor.sync_metadata(
            configuration.metadata_url.as_str(),
            configuration.metadata_locales_url.as_str(),
        );

        Ok(())
    }

    /// Updates filters
    pub(crate) fn update_filters(
        &self,
        connection_manager: &DbConnectionManager,
        configuration: &Configuration,
        ignore_filters_expiration: bool,
        loose_timeout: i32,
        ignore_filters_status: bool,
    ) -> FLMResult<Option<UpdateResult>> {
        let (filters_with_directives, result) =
            connection_manager.execute_db(|conn: Connection| {
                let filters_with_directives = self.gets_filters_with_directives(&conn, &[])?;

                let result = FilterRepository::new()
                    .select(&conn, None)
                    .map_err(FLMError::from_database)?;

                Ok((filters_with_directives, result))
            })?;

        let Some(records) = result else {
            return Ok(None);
        };

        let update_result = update_filters_action(
            records,
            filters_with_directives,
            &connection_manager,
            ignore_filters_expiration,
            ignore_filters_status,
            loose_timeout,
            &configuration,
        )?;

        Ok(Some(update_result))
    }

    /// Gets filters ids with compilation directives in rules
    fn gets_filters_with_directives(
        &self,
        conn: &Connection,
        with_filter_ids: &[FilterId],
    ) -> FLMResult<HashSet<FilterId>> {
        RulesListRepository::new()
            .select_filter_ids_by_rules_with_directives(&conn, with_filter_ids)
            .map_err(FLMError::from_database)
    }
}

#[cfg(test)]
mod tests {
    use super::FilterUpdateManager;
    use crate::storage::entities::rules_list::rules_list_entity::RulesListEntity;
    use crate::storage::repositories::rules_list_repository::RulesListRepository;
    use crate::storage::repositories::Repository;
    use crate::storage::{with_transaction, DbConnectionManager};
    use rusqlite::{Connection, Transaction};

    #[test]
    fn test_get_rules_with_directives() {
        let source = DbConnectionManager::factory_test().unwrap();
        unsafe { source.lift_up_database().unwrap() };

        // Create 6 filters with different rules
        let filter_ids = (1..=6).map(Into::into).collect::<Vec<_>>();

        // Prepare data: 4 filters with directives and 2 without
        let rules_list_entities = vec![
            RulesListEntity {
                filter_id: filter_ids[0],
                text: "example.com\n!#include example_include.txt".to_string(),
                disabled_text: "".to_string(),
                rules_count: 1,
            },
            RulesListEntity {
                filter_id: filter_ids[1],
                text: "example.org\n!#if browser_generic".to_string(),
                disabled_text: "".to_string(),
                rules_count: 1,
            },
            RulesListEntity {
                filter_id: filter_ids[2],
                text: "example.net\n!#else".to_string(),
                disabled_text: "".to_string(),
                rules_count: 1,
            },
            RulesListEntity {
                filter_id: filter_ids[3],
                text: "example.io\n!#endif".to_string(),
                disabled_text: "".to_string(),
                rules_count: 1,
            },
            RulesListEntity {
                filter_id: filter_ids[4],
                text: "example.dev\nnormal.rule".to_string(),
                disabled_text: "".to_string(),
                rules_count: 1,
            },
            RulesListEntity {
                filter_id: filter_ids[5],
                text: "example.app\nanother.normal.rule".to_string(),
                disabled_text: "".to_string(),
                rules_count: 1,
            },
        ];

        let manager = FilterUpdateManager::new();

        source
            .execute_db(|mut conn: Connection| {
                // Save rules to the database
                with_transaction(&mut conn, |tx: &Transaction| {
                    RulesListRepository::new().insert(&tx, &rules_list_entities)
                })
            })
            .unwrap();

        let filters_with_directives = source
            .execute_db(|conn: Connection| {
                // Test 1: Check that the method returns all 4 filters with directives when passing an empty array
                manager.gets_filters_with_directives(&conn, &[])
            })
            .unwrap();

        assert_eq!(
            filters_with_directives.len(),
            4,
            "Should find 4 filters with directives"
        );
        assert!(
            filters_with_directives.contains(&filter_ids[0]),
            "Should contain filter_id with !#include"
        );
        assert!(
            filters_with_directives.contains(&filter_ids[1]),
            "Should contain filter_id with !#if"
        );
        assert!(
            filters_with_directives.contains(&filter_ids[2]),
            "Should contain filter_id with !#else"
        );
        assert!(
            filters_with_directives.contains(&filter_ids[3]),
            "Should contain filter_id with !#endif"
        );
        assert!(
            !filters_with_directives.contains(&filter_ids[4]),
            "Should not contain filter_id without directives"
        );
        assert!(
            !filters_with_directives.contains(&filter_ids[5]),
            "Should not contain filter_id without directives"
        );

        // Test 2: Check filtering with specified IDs
        // Select 2 filters with directives and 2 without
        let selected_ids = vec![
            filter_ids[0], // with !#include directive
            filter_ids[2], // with !#else directive
            filter_ids[4], // without directives
            filter_ids[5], // without directives
        ];

        let filtered_with_directives = source
            .execute_db(|conn: Connection| {
                manager.gets_filters_with_directives(&conn, &selected_ids)
            })
            .unwrap();

        assert_eq!(
            filtered_with_directives.len(),
            2,
            "Should find 2 filters with directives from the selected ones"
        );
        assert!(
            filtered_with_directives.contains(&filter_ids[0]),
            "Should contain filter_id with !#include"
        );
        assert!(
            filtered_with_directives.contains(&filter_ids[2]),
            "Should contain filter_id with !#else"
        );
        assert!(
            !filtered_with_directives.contains(&filter_ids[4]),
            "Should not contain filter_id without directives"
        );
        assert!(
            !filtered_with_directives.contains(&filter_ids[5]),
            "Should not contain filter_id without directives"
        );
    }
}
