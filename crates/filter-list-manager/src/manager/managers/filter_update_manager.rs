use rusqlite::types::Value;
use rusqlite::Connection;

use crate::filters::indexes::indexes_processor::IndexesProcessor;
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
            &connection_manager,
            ignore_filters_expiration,
            ignore_filters_status,
            loose_timeout,
            &configuration,
        )?;

        Ok(Some(update_result))
    }
}
