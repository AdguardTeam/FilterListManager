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
    let conn = Connection::open_with_flags(connection_source.get_calculated_path(), open_flags)
        .map_err(FLMError::from_database)?;

    enable_wal_mode(&conn);

    Ok(conn)
}

/// Attempts to switch the database to WAL (Write-Ahead Logging) journal mode.
///
/// WAL significantly reduces disk write volume compared to the default
/// `DELETE` journal mode. When it can be applied, it is a persistent,
/// database-level setting stored in the DB header, so connections that open
/// the database afterwards typically use WAL without re-applying the pragma
/// (subject to the same filesystem support).
///
/// This is **best-effort**: some filesystems (e.g. certain network/shared
/// mounts) and in-memory databases do not support WAL. In that case SQLite
/// rejects the pragma and the database keeps its existing journal mode. Failing
/// to enable WAL is not fatal — a working connection in `DELETE` mode is
/// preferable to no connection at all. The result is intentionally ignored;
/// the effective mode can be checked on demand via `PRAGMA journal_mode`.
///
/// Note: once WAL is active, SQLite creates two sidecar files alongside the
/// database — `<dbname>-wal` (the write-ahead log) and `<dbname>-shm` (shared
/// memory). These are required for normal operation and must not be removed
/// while the database is in use.
fn enable_wal_mode(conn: &Connection) {
    let _ = conn.pragma_update(None, "journal_mode", "WAL");
}

#[cfg(test)]
mod tests {
    use super::{connect, connect_with_create, enable_wal_mode};
    use crate::storage::error::DatabaseError;
    use crate::storage::DbConnectionManager;
    use crate::FLMError;
    use rusqlite::Connection;

    #[test]
    fn test_db_file_does_not_exists() {
        let database_path_holder = DbConnectionManager::factory_test().unwrap();

        let err = connect(&database_path_holder).err().unwrap();

        assert_eq!(err, FLMError::Database(DatabaseError::CannotOpen));
    }

    /// On a normal local filesystem SQLite supports WAL, so opening a fresh
    /// database through the production code path must switch it to WAL.
    #[test]
    fn test_connect_with_create_sets_wal_mode() {
        let dcm = DbConnectionManager::factory_test().unwrap();

        // Create the DB and open a connection.
        let conn = connect_with_create(&dcm).unwrap();

        // Verify that the journal_mode was actually set to WAL.
        let mode: String = conn
            .pragma_query_value(None, "journal_mode", |row| row.get(0))
            .unwrap();
        assert!(
            mode.eq_ignore_ascii_case("wal"),
            "Expected journal_mode=wal, got {}",
            mode
        );
    }

    /// WAL is a persistent DB-header setting: a second connection to an
    /// existing WAL database must report WAL without re-applying the pragma.
    #[test]
    fn test_connect_preserves_wal_mode() {
        let dcm = DbConnectionManager::factory_test().unwrap();

        // First connection: creates the DB and sets WAL.
        let conn1 = connect_with_create(&dcm).unwrap();
        let mode: String = conn1
            .pragma_query_value(None, "journal_mode", |row| row.get(0))
            .unwrap();
        assert!(mode.eq_ignore_ascii_case("wal"));
        drop(conn1);

        // Second connection: should inherit WAL from the DB header.
        let conn2 = connect(&dcm).unwrap();
        let mode: String = conn2
            .pragma_query_value(None, "journal_mode", |row| row.get(0))
            .unwrap();
        assert!(mode.eq_ignore_ascii_case("wal"));
    }

    /// Exercises the best-effort contract through the production
    /// `enable_wal_mode` helper: in-memory databases do not support WAL, so the
    /// pragma is accepted but the mode stays "memory". The connection must
    /// remain fully usable, and the database must not report WAL.
    #[test]
    fn test_connection_remains_usable_when_wal_cannot_be_applied() {
        let conn = Connection::open_in_memory().unwrap();
        enable_wal_mode(&conn);

        // WAL was requested but is unsupported for in-memory databases.
        let mode: String = conn
            .pragma_query_value(None, "journal_mode", |row| row.get(0))
            .unwrap();
        assert!(!mode.eq_ignore_ascii_case("wal"));

        // The connection must remain fully usable.
        conn.execute_batch("CREATE TABLE t (x INTEGER PRIMARY KEY)")
            .unwrap();
        conn.execute("INSERT INTO t VALUES (1)", []).unwrap();
        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM t", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }
}
