use super::DbConnectionManager;
use crate::{FLMError, FLMResult};
use rusqlite::{Connection, OpenFlags};

#[doc(hidden)]
/// Open a new connection to a SQLite storage. If db file does not exist, it will be created.
///
/// # Failure
///
/// [`FLMError`] if you couldn't open db
pub(super) fn connect_with_create(
    connection_source: &DbConnectionManager,
) -> FLMResult<Connection> {
    connect_internal(
        connection_source,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    )
}

/// Creates connection to existing db from path in R/W mode.
///
/// # Failure
///
/// returns [`FLMError`] if an error encountered
pub(super) fn connect(connection_source: &DbConnectionManager) -> FLMResult<Connection> {
    connect_internal(connection_source, OpenFlags::SQLITE_OPEN_READ_WRITE)
}

/// Main connection function
#[inline]
fn connect_internal(
    connection_source: &DbConnectionManager,
    open_flags: OpenFlags,
) -> FLMResult<Connection> {
    Connection::open_with_flags(connection_source.get_calculated_path(), open_flags)
        .map_err(FLMError::from_database)
}

#[cfg(test)]
mod tests {
    use super::connect;
    use crate::storage::error::DatabaseError;
    use crate::storage::DbConnectionManager;
    use crate::test_utils::do_with_tests_helper;
    use crate::FLMError;

    #[test]
    fn test_db_file_does_not_exists() {
        do_with_tests_helper(|mut helper| helper.increment_postfix());

        let database_path_holder = DbConnectionManager::factory_test().unwrap();

        let err = connect(&database_path_holder).err().unwrap();

        assert_eq!(err, FLMError::Database(DatabaseError::CannotOpen));
    }
}
