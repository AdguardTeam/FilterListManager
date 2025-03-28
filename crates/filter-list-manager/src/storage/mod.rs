//! Root module of data storage operations

use crate::{FLMError, FLMResult};
use rusqlite::{Connection, Transaction};

pub(crate) mod blob;
pub mod constants;
pub(crate) mod database_status;
pub(crate) mod db_bootstrap;
mod db_connection_manager;
pub(crate) mod entities;
pub mod error;
mod migrations;
pub(crate) mod repositories;
pub(crate) mod sql_generators;
mod utils;

pub use db_connection_manager::DbConnectionManager;

#[doc(hidden)]
/// Make a block, wrapped with transaction
///
/// # Failure
///
/// returns [`rusqlite::Error`] if an error encountered
pub(crate) fn with_transaction<F, U>(conn: &mut Connection, f: F) -> FLMResult<U>
where
    F: FnOnce(&Transaction) -> rusqlite::Result<U>,
{
    let transaction = conn.transaction().map_err(FLMError::from_database)?;
    let out = f(&transaction).map_err(FLMError::from_database)?;
    transaction.commit().map_err(FLMError::from_database)?;

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
