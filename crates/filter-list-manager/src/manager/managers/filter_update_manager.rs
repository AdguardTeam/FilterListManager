use rusqlite::types::Value;
use rusqlite::Connection;

use crate::filters::indexes::indexes_processor::IndexesProcessor;
use crate::manager::models::PullMetadataResult;
use crate::manager::update_filters_action::update_filters_action;
use crate::storage::repositories::filter_repository::FilterRepository;
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
        let values = ids.into_iter().map(|id| id.into()).collect::<Vec<Value>>();

        let result = connection_manager.execute_db(|conn: Connection| {
            FilterRepository::new()
                .select(&conn, Some(SQLOperator::FieldIn("filter_id", values)))
                .map_err(FLMError::from_database)
        })?;

        let Some(records) = result else {
            return Ok(None);
        };

        let update_result = update_filters_action(
            records,
            connection_manager,
            true,
            true,
            loose_timeout,
            configuration,
        )?;

        Ok(Some(update_result))
    }

    /// Pulls metadata
    pub(crate) fn pull_metadata(
        &self,
        connection_manager: &DbConnectionManager,
        configuration: &Configuration,
    ) -> FLMResult<PullMetadataResult> {
        let mut processor = IndexesProcessor::factory(connection_manager, configuration)?;

        processor.sync_metadata(
            configuration.metadata_url.as_str(),
            configuration.metadata_locales_url.as_str(),
        )
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
        let result = connection_manager.execute_db(|conn: Connection| {
            FilterRepository::new()
                .select(&conn, None)
                .map_err(FLMError::from_database)
        })?;

        let Some(records) = result else {
            return Ok(None);
        };

        let update_result = update_filters_action(
            records,
            connection_manager,
            ignore_filters_expiration,
            ignore_filters_status,
            loose_timeout,
            configuration,
        )?;

        Ok(Some(update_result))
    }

    /// Updates filters with custom setup
    pub(crate) fn update_filters_by_ids(
        &self,
        connection_manager: &DbConnectionManager,
        configuration: &Configuration,
        ids: Vec<FilterId>,
        force_update: bool,
        loose_timeout: i32,
        ignore_filters_status: bool,
    ) -> FLMResult<Option<UpdateResult>> {
        if ids.is_empty() {
            return Ok(Some(UpdateResult::default()));
        }

        let values = ids.into_iter().map(|id| id.into()).collect::<Vec<Value>>();

        let result = connection_manager.execute_db(|conn: Connection| {
            FilterRepository::new()
                .select(&conn, Some(SQLOperator::FieldIn("filter_id", values)))
                .map_err(FLMError::from_database)
        })?;

        let Some(records) = result else {
            return Ok(Some(UpdateResult::default()));
        };

        let update_result = update_filters_action(
            records,
            connection_manager,
            force_update,
            ignore_filters_status,
            loose_timeout,
            configuration,
        )?;

        Ok(Some(update_result))
    }
}

#[cfg(test)]
mod tests {
    use super::FilterUpdateManager;
    use crate::storage::repositories::filter_repository::FilterRepository;
    use crate::storage::{with_transaction, DbConnectionManager};
    use crate::test_utils::tests_path;
    use crate::{Configuration, FilterId};
    use rusqlite::Connection;
    use url::Url;

    #[test]
    fn test_pull_metadata() {
        let conn = DbConnectionManager::factory_test().unwrap();
        let folder = tests_path("fixtures/pull_metadata_existent_db_test");
        let manager = FilterUpdateManager::new();

        // Fill database with new 13 filters (ids: 1-14, exclude 12)
        let mut configuration1 = Configuration::default();
        let mut file1 = folder.clone();
        file1.push("filters1.json");
        let mut i18n_file1 = folder.clone();
        i18n_file1.push("filters_i18n.json");

        unsafe {
            conn.lift_up_database().unwrap();
        }

        configuration1.metadata_url = Url::from_file_path(file1).unwrap().to_string();
        configuration1.metadata_locales_url = Url::from_file_path(i18n_file1).unwrap().to_string();

        let result1 = manager.pull_metadata(&conn, &configuration1).unwrap();

        assert_eq!(result1.moved_filters.len(), 0);
        assert_eq!(result1.removed_filters.len(), 0);
        assert_eq!(result1.added_filters.len(), 13);

        // Next update list:
        // Filters:
        // 1, 7, 10, 13 - were deprecated
        // 14 - suddenly disappeared
        // 250 - new, but already deprecated (should not be included)
        // 255, 257 - new

        // Will enable 1 and 14, to check filters movement behaviour
        let moved_filters: [FilterId; 2] = [1, 14];
        conn.execute_db(|mut conn: Connection| {
            with_transaction(&mut conn, |tx| {
                FilterRepository::new().toggle_filter_lists(&tx, &moved_filters, true)
            })
        })
        .unwrap();

        // Run next update
        let mut configuration2 = Configuration::default();
        let mut i18n_file2 = folder.clone();
        i18n_file2.push("filters_i18n.json");
        let mut file2 = folder.clone();
        file2.push("filters2.json");
        configuration2.metadata_url = Url::from_file_path(file2).unwrap().to_string();
        configuration2.metadata_locales_url = Url::from_file_path(i18n_file2).unwrap().to_string();

        let result2 = manager.pull_metadata(&conn, &configuration2).unwrap();

        // 255, 257 were added
        assert_eq!(result2.added_filters.len(), 2);
        // 1, 14 were moved
        assert_eq!(result2.moved_filters.len(), 2);
        result2.moved_filters.iter().for_each(|moved| {
            assert!(moved.new_id.is_negative());

            let found = moved_filters
                .iter()
                .find(|element| &moved.previous_id == *element);

            assert!(found.is_some())
        });
        // 7, 10, 13 should be removed
        assert_eq!(result2.removed_filters.len(), 3);

        conn.execute_db(|conn: Connection| {
            let list = FilterRepository::new()
                .select_filters_except_bootstrapped(&conn)
                .unwrap()
                .unwrap();

            // Other filters should be in db
            assert_eq!(list.len(), 12);

            // New custom filters (Were 1,14)
            assert_eq!(
                list.iter()
                    .filter(|entity| entity.filter_id.as_ref().unwrap().is_negative())
                    .count(),
                2
            );

            // New filters
            assert!(list
                .iter()
                .find(|entity| entity.filter_id.as_ref().unwrap() == &255)
                .is_some());
            assert!(list
                .iter()
                .find(|entity| entity.filter_id.as_ref().unwrap() == &257)
                .is_some());

            Ok(())
        })
        .unwrap();
    }
}
