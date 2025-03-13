use crate::storage::repositories::filter_repository::FilterRepository;
use crate::storage::with_transaction;
use crate::storage::DbConnectionManager;
use crate::FLMError;
use crate::FLMResult;
use crate::FilterId;
use rusqlite::Connection;
use rusqlite::Transaction;

/// Service for filter repository
pub(crate) struct FilterService;

impl FilterService {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    pub(crate) fn enable_filter_lists(
        &self,
        connection_manager: &DbConnectionManager,
        ids: Vec<FilterId>,
        is_enabled: bool,
    ) -> FLMResult<usize> {
        connection_manager.execute_db(move |mut conn: Connection| {
            let tx = conn.transaction().map_err(FLMError::from_database)?;

            let result = FilterRepository::new()
                .toggle_filter_lists(&tx, &ids, is_enabled)
                .map_err(FLMError::from_database)?;

            tx.commit().map_err(FLMError::from_database)?;

            Ok(result)
        })
    }

    pub(crate) fn toggle_is_installed(
        &self,
        connection_manager: &DbConnectionManager,
        ids: Vec<FilterId>,
        is_installed: bool,
    ) -> FLMResult<usize> {
        connection_manager.execute_db(move |mut conn: Connection| {
            let tx = conn.transaction().map_err(FLMError::from_database)?;

            let result = FilterRepository::new()
                .toggle_is_installed(&tx, &ids, is_installed)
                .map_err(FLMError::from_database)?;

            tx.commit().map_err(FLMError::from_database)?;

            Ok(result)
        })
    }

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

        connection_manager.execute_db(move |mut conn: Connection| {
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
}
