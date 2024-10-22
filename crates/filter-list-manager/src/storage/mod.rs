//! Root module of data storage operations
use crate::storage::database_configuration_holder::DatabaseConfigurationHolder;

use crate::{Configuration, DbJournalMode, FLMError, FLMResult};
use rusqlite::{Connection, OpenFlags, Transaction};

pub mod constants;
pub(crate) mod database_configuration_holder;
pub(crate) mod database_status;
pub(crate) mod db_bootstrap;
pub(crate) mod db_file_utils;
pub(crate) mod entities;
pub mod error;
mod migrations;
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
/// [`FLMError`] if you couldn't open db
pub fn connect_with_create(
    connection_source: &DatabaseConfigurationHolder,
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
pub(crate) fn connect(connection_source: &DatabaseConfigurationHolder) -> FLMResult<Connection> {
    connect_internal(connection_source, OpenFlags::SQLITE_OPEN_READ_WRITE)
}

/// Main connection function
#[inline]
fn connect_internal(
    connection_source: &DatabaseConfigurationHolder,
    open_flags: OpenFlags,
) -> FLMResult<Connection> {
    let conn = Connection::open_with_flags(connection_source.get_calculated_path(), open_flags)
        .map_err(FLMError::from_database)?;

    let mode = connection_source.get_journal_mode();
    if mode != DbJournalMode::DEFAULT {
        conn.pragma_update(None, "journal_mode", mode.as_str())
            .map_err(FLMError::from_database)?;
    }

    println!("JOURNAL MODE: {}", mode.as_str());

    conn.pragma_update(None, "busy_timeout", 175000)
        .map_err(FLMError::from_database)?;

    Ok(conn)
}

/// Connects to database, using the [`Configuration`] object
///
/// # Failure
///
/// returns [`FLMError`] if an error encountered
pub(crate) fn connect_using_configuration(configuration: &Configuration) -> FLMResult<Connection> {
    let value = DatabaseConfigurationHolder::from_configuration(configuration)?;
    connect(&value)
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
    let transaction = conn.transaction()?;
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
    let transaction = conn.transaction()?;
    let out = f(&transaction)?;

    Ok((transaction, out))
}

#[cfg(test)]
mod tests {
    use crate::storage::connect;
    use crate::storage::database_configuration_holder::DatabaseConfigurationHolder;
    use crate::storage::error::DatabaseError;
    use crate::test_utils::do_with_tests_helper;
    use crate::FLMError;

    #[test]
    fn test_db_file_does_not_exists() {
        do_with_tests_helper(|mut helper| helper.increment_postfix());

        let database_path_holder = DatabaseConfigurationHolder::factory_test().unwrap();

        let err = connect(&database_path_holder).err().unwrap();

        assert_eq!(err, FLMError::Database(DatabaseError::CannotOpen));
    }
}
