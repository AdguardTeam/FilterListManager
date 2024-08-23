//! Root module of data storage operations
use crate::storage::database_path_holder::DatabasePathHolder;
use crate::{Configuration, FLMError, FLMResult};
use rusqlite::{Connection, OpenFlags, Transaction};
use std::path::PathBuf;

pub mod constants;
pub(crate) mod database_path_holder;
pub(crate) mod database_status;
pub(crate) mod db_bootstrap;
pub(crate) mod db_file_utils;
pub(crate) mod entities;
pub mod error;
pub(crate) mod init_schema;
pub(crate) mod repositories;
pub(crate) mod sql_generators;
mod utils;

/// Database filename for [`crate::FilterListType::STANDARD`]
pub const STANDARD_FILTERS_DATABASE_FILENAME: &str = "agflm_standard.db";
/// Database filename for [`crate::FilterListType::DNS`]
pub const DNS_FILTERS_DATABASE_FILENAME: &str = "agflm_dns.db";

#[doc(hidden)]
/// Open a new connection to a SQLite storage. If db file does not exist, it will be created.
///
/// # Failure
///
/// `Err` if you couldn't open db
pub fn force_connect(db_path: &PathBuf) -> FLMResult<Connection> {
    connect_internal(
        db_path,
        OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
    )
}

/// Creates connection to existing db from path
///
/// # Failure
///
/// returns [`AGError`] if an error encountered
pub(crate) fn connect(db_path: &PathBuf) -> FLMResult<Connection> {
    connect_internal(db_path, OpenFlags::SQLITE_OPEN_READ_WRITE)
}

#[inline]
fn connect_internal(path_buf: &PathBuf, open_flags: OpenFlags) -> FLMResult<Connection> {
    Connection::open_with_flags(path_buf, open_flags).map_err(FLMError::from_database)
}

/// Connects to database, using the [`Configuration`] object
///
/// # Failure
///
/// returns [`FLMError`] if an error encountered
pub(crate) fn connect_using_configuration(configuration: &Configuration) -> FLMResult<Connection> {
    let database_path_holder = DatabasePathHolder::from_configuration(configuration)?;
    connect(database_path_holder.get_calculated_path())
}

/// Make transaction for passed connection
///
/// # Failure
///
/// returns [`rusqlite::Error`] if an error encountered
pub(crate) fn build_transaction(conn: &mut Connection) -> rusqlite::Result<Transaction> {
    conn.transaction()
}

#[doc(hidden)]
/// Make a block, wrapped with transaction
///
/// # Failure
///
/// returns [`rusqlite::Error`] if an error encountered
pub(crate) fn with_transaction<F, U>(conn: &mut Connection, f: F) -> rusqlite::Result<U>
where
    F: FnOnce(&Transaction) -> rusqlite::Result<U>,
{
    let transaction = build_transaction(conn)?;
    let out = f(&transaction)?;
    transaction.commit()?;

    Ok(out)
}

#[doc(hidden)]
/// Make a transactional block, but don't commit transaction. Return it instead
///
/// Returns tuple([`Transaction`], `U`)
///
/// # Failure
///
/// Returns [`rusqlite::Error`] if an error encountered
pub(crate) fn spawn_transaction<F, U>(
    conn: &mut Connection,
    f: F,
) -> rusqlite::Result<(Transaction, U)>
where
    F: FnOnce(&Transaction) -> rusqlite::Result<U>,
{
    let transaction = build_transaction(conn)?;
    let out = f(&transaction)?;

    Ok((transaction, out))
}

#[cfg(test)]
mod tests {
    use crate::storage::connect;
    use crate::storage::database_path_holder::DatabasePathHolder;
    use crate::storage::error::DatabaseError;
    use crate::test_utils::do_with_tests_helper;
    use crate::FLMError;

    #[test]
    fn test_db_file_does_not_exists() {
        do_with_tests_helper(|mut helper| helper.increment_postfix());

        let database_path_holder = DatabasePathHolder::factory_test().unwrap();
        let connection_source = database_path_holder.get_calculated_path();

        let err = connect(&connection_source).err().unwrap();

        assert_eq!(err, FLMError::Database(DatabaseError::CannotOpen));
    }
}
