use crate::storage::connect;
use crate::storage::database_path_holder::DatabasePathHolder;
use crate::storage::db_bootstrap::db_bootstrap;
use crate::storage::db_file_utils::{
    is_db_file_exists, make_directory_for_db_file_if_is_not_exists,
};
use crate::storage::init_schema::{init_schema, init_schema_with_conn};
use crate::storage::migrations::run_migrations;
use crate::storage::repositories::db_schema_repository::DbSchemaRepository;
use crate::storage::repositories::filter_repository::{FilterRepository, FILTER_TABLE_NAME};
use crate::{FLMError, FLMResult};
use rusqlite::{Connection, OpenFlags};
use std::path::PathBuf;
use std::sync::OnceLock;

/// Once guard for lifting database
static LIFT_UP_RESULT: OnceLock<FLMResult<()>> = OnceLock::new();

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

/// Special procedure for automatic lifting database to required level
pub(super) fn lift_up_once(db_path: &PathBuf) -> FLMResult<()> {
    let result = LIFT_UP_RESULT.get_or_init(|| {
        let mut cloned = db_path.to_owned();

        // Pass DatabaseStatus::NoFile here, and below with OpenFlags::SQLITE_OPEN_CREATE
        make_directory_for_db_file_if_is_not_exists(&mut cloned)?;

        let mut conn = Connection::open_with_flags(
            db_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )
        .map_err(FLMError::from_database)?;

        // Check DatabaseStatus::NoSchema here
        let is_filters_table_exists = DbSchemaRepository::new()
            .is_table_exists(&conn, FILTER_TABLE_NAME)
            .map_err(FLMError::from_database)?;

        if is_filters_table_exists == false {
            init_schema_with_conn(&conn)?;
        }

        let at_least_one_record_exist = FilterRepository::new()
            .has_at_least_one_record(&conn)
            .map_err(FLMError::from_database)?;

        if at_least_one_record_exist == false {
            db_bootstrap(&mut conn)?;
        }

        run_migrations(&mut conn)
    });

    if result.is_err() {
        return FLMError::make_err(format!(
            "Cannot autolift db. Error encountered: {}",
            result.as_ref().unwrap_err().to_string()
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::lift_up_database;
    use crate::storage::database_path_holder::DatabasePathHolder;
    use crate::storage::db_bootstrap::db_bootstrap;
    use crate::storage::db_file_utils::is_db_file_exists;
    use crate::storage::init_schema::init_schema;
    use crate::storage::repositories::db_schema_repository::DbSchemaRepository;
    use crate::storage::repositories::filter_repository::{FilterRepository, FILTER_TABLE_NAME};
    use crate::storage::sql_generators::operator::SQLOperator::FieldEqualValue;
    use crate::storage::{connect, force_connect};
    use crate::test_utils::do_with_tests_helper;
    use crate::USER_RULES_FILTER_LIST_ID;
    use std::fs::File;

    #[test]
    fn test_lift_up_from_nonexistent_file() {
        do_with_tests_helper(|mut helper| helper.increment_postfix());

        let database_path_holder = DatabasePathHolder::factory_test().unwrap();
        let path = database_path_holder.get_calculated_path();

        // File must not exist
        assert!(!is_db_file_exists(path));
        lift_up_database(&database_path_holder).unwrap();

        let conn = connect(database_path_holder.get_calculated_path()).unwrap();
        let table_exists = DbSchemaRepository::new()
            .is_table_exists(&conn, FILTER_TABLE_NAME)
            .unwrap();

        assert!(table_exists);

        let filters = FilterRepository::new()
            .select(
                &conn,
                Some(FieldEqualValue(
                    "filter_id",
                    USER_RULES_FILTER_LIST_ID.into(),
                )),
            )
            .unwrap()
            .unwrap();

        let filter_id = filters[0].filter_id.unwrap();

        assert_eq!(filter_id, USER_RULES_FILTER_LIST_ID);
    }

    #[test]
    fn test_lift_up_from_empty_file() {
        do_with_tests_helper(|mut helper| helper.increment_postfix());

        let database_path_holder = DatabasePathHolder::factory_test().unwrap();
        let path = database_path_holder.get_calculated_path();

        {
            let _ = File::create(path);
        }

        // File must exist here
        assert!(is_db_file_exists(path));

        lift_up_database(&database_path_holder).unwrap();

        let conn = connect(
            DatabasePathHolder::factory_test()
                .unwrap()
                .get_calculated_path(),
        )
        .unwrap();
        let table_exists = DbSchemaRepository::new()
            .is_table_exists(&conn, FILTER_TABLE_NAME)
            .unwrap();

        assert!(table_exists);

        let filters = FilterRepository::new()
            .select(
                &conn,
                Some(FieldEqualValue(
                    "filter_id",
                    USER_RULES_FILTER_LIST_ID.into(),
                )),
            )
            .unwrap()
            .unwrap();

        let filter_id = filters[0].filter_id.unwrap();

        assert_eq!(filter_id, USER_RULES_FILTER_LIST_ID);
    }

    #[test]
    fn test_lift_up_from_only_schema() {
        do_with_tests_helper(|mut helper| helper.increment_postfix());

        let database_path_holder = DatabasePathHolder::factory_test().unwrap();
        let connection_source = database_path_holder.get_calculated_path();

        {
            // SAFETY: Do not need automatic lifting database here
            let mut conn = unsafe { force_connect(&connection_source) }.unwrap();
            init_schema(&connection_source).unwrap();
            db_bootstrap(&mut conn).unwrap();
        }

        lift_up_database(&database_path_holder).unwrap();

        let conn = connect(&connection_source).unwrap();

        let filters = FilterRepository::new()
            .select(
                &conn,
                Some(FieldEqualValue(
                    "filter_id",
                    USER_RULES_FILTER_LIST_ID.into(),
                )),
            )
            .unwrap()
            .unwrap();

        let filter_id = filters[0].filter_id.unwrap();

        assert_eq!(filter_id, USER_RULES_FILTER_LIST_ID);
    }
}
