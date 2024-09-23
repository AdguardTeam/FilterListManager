use crate::storage::connect;
use crate::storage::database_path_holder::DatabasePathHolder;
use crate::storage::db_bootstrap::db_bootstrap;
use crate::storage::db_file_utils::is_db_file_exists;
use crate::storage::init_schema::init_schema;
use crate::storage::migrations::run_migrations;
use crate::storage::repositories::db_schema_repository::DbSchemaRepository;
use crate::storage::repositories::filter_repository::{FilterRepository, FILTER_TABLE_NAME};
use crate::{FLMError, FLMResult};

/// Current database status
pub(crate) enum DatabaseStatus {
    /// DB file does not exist
    NoFile,
    /// File exists, but has no schema or file corrupted
    NoSchema,
    /// File has schema, but is not filled with data
    OnlySchema,
    /// Db is fully operational
    Filled,
}

/// Determines current [`DatabaseStatus`]
pub(crate) fn get_database_status(
    database_path_holder: &DatabasePathHolder,
) -> FLMResult<DatabaseStatus> {
    if is_db_file_exists(database_path_holder.get_calculated_path()) == false {
        return Ok(DatabaseStatus::NoFile);
    }

    let conn = connect(database_path_holder.get_calculated_path())?;

    let is_filters_table_exists = DbSchemaRepository::new()
        .is_table_exists(&conn, FILTER_TABLE_NAME)
        .map_err(FLMError::from_database)?;

    if is_filters_table_exists == false {
        return Ok(DatabaseStatus::NoSchema);
    }

    let at_least_one_record_exist = FilterRepository::new()
        .has_at_least_one_record(&conn)
        .map_err(FLMError::from_database)?;

    if at_least_one_record_exist == false {
        return Ok(DatabaseStatus::OnlySchema);
    }

    return Ok(DatabaseStatus::Filled);
}

/// "Lifting" [`DatabaseStatus`] for filling database with index
pub(crate) fn lift_up_database(database_path_holder: &DatabasePathHolder) -> FLMResult<()> {
    match get_database_status(&database_path_holder)? {
        DatabaseStatus::NoFile | DatabaseStatus::NoSchema => {
            let mut conn = init_schema(database_path_holder.get_calculated_path())?;
            db_bootstrap(&mut conn)
        }
        DatabaseStatus::OnlySchema => {
            let mut conn = connect(&database_path_holder.get_calculated_path())?;
            db_bootstrap(&mut conn)
        }
        DatabaseStatus::Filled => Ok(()),
    }?;

    let mut conn = connect(database_path_holder.get_calculated_path())?;
    run_migrations(&mut conn)
}
