use rusqlite::Connection;

use crate::storage::entities::db_metadata_entity::DBMetadataEntity;
use crate::storage::repositories::db_metadata_repository::DBMetadataRepository;
use crate::storage::DbConnectionManager;
use crate::FLMError;
use crate::FLMResult;

/// Manager for general database logic
pub(crate) struct DbManager;

impl DbManager {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    /// Gets database path
    pub(crate) fn get_database_path(
        &self,
        connection_manager: &DbConnectionManager,
    ) -> FLMResult<String> {
        let path = connection_manager.get_calculated_path();

        if path.is_absolute() {
            Ok(path.to_string_lossy().to_string())
        } else {
            path.canonicalize()
                .map_err(FLMError::from_io)
                .map(|path| path.to_string_lossy().to_string())
        }
    }

    /// Gets database version
    pub(crate) fn get_database_version(
        &self,
        connection_manager: &DbConnectionManager,
    ) -> FLMResult<Option<i32>> {
        let entity: Option<DBMetadataEntity> =
            connection_manager.execute_db(|conn: Connection| {
                DBMetadataRepository::read(&conn).map_err(FLMError::from_database)
            })?;

        Ok(entity.map(|e| e.version))
    }
}
