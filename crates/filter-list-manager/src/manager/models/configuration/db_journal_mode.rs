//! SQLite journal modes description

/// SQLite journal modes selector.
/// Will be used in PRAGMA statements for connections.
///
/// [https://www.sqlite.org/pragma.html#pragma_journal_mode](SQLite documentation)
#[derive(Copy, Clone)]
pub enum DbJournalMode {
    WAL,
    DELETE,
    MEMORY,
    TRUNCATE,
    PERSIST,
    OFF,
}

impl DbJournalMode {
    /// Returns `str` for the PRAGMA statement
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            DbJournalMode::WAL => "WAL",
            DbJournalMode::DELETE => "DELETE",
            DbJournalMode::MEMORY => "MEMORY",
            DbJournalMode::TRUNCATE => "TRUNCATE",
            DbJournalMode::PERSIST => "PERSIST",
            DbJournalMode::OFF => "OFF",
        }
    }
}
