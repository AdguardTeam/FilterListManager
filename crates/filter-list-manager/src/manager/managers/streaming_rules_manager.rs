use std::collections::HashSet;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

use rusqlite::Connection;
use rusqlite::Error;

use crate::storage::blob::write_to_stream;
use crate::storage::repositories::rules_list_repository::RulesListRepository;
use crate::storage::DbConnectionManager;
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
        filter_id: FilterId,
        file_path: P,
    ) -> FLMResult<()> {
        let file_already_exists = fs::metadata(&file_path).is_ok();

        let mut handler = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&file_path)
            .map_err(FLMError::from_io)?;

        connection_manager
            .execute_db(|conn: Connection| {
                let rules_repository = RulesListRepository::new();
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
}
