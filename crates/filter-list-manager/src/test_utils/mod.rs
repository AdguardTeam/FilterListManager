pub(crate) mod indexes_fixtures;
pub(crate) mod tests_db;

use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard, Once};

use tests_db::TestsDb;

use crate::filters::indexes::indexes_processor::IndexesProcessor;
use crate::storage::connect;
use crate::storage::database_path_holder::DatabasePathHolder;
use crate::storage::database_status::lift_up_database;
use crate::storage::entities::filter_entity::FilterEntity;
use crate::storage::repositories::filter_repository::FilterRepository;
use libc::atexit;
use rusqlite::Connection;

// TODO: For multithreading tests, we need a copy of each helper,
// because we can increment postfixes, and databases names may collides in different tests
static TESTS_DB_HELPER: Lazy<Mutex<TestsDb>> = Lazy::new(|| Mutex::new(TestsDb::new()));

/// Implemented teardown for DB files, hastily
static TEARDOWN_HACK: Once = Once::new();

/// Sets "at application exit" hook for cleanup created db files
pub fn set_at_exit_hook() {
    unsafe {
        atexit(tear_down);
    }
}

#[no_mangle]
pub extern "C" fn tear_down() {
    // I believe `unwrap_or_else` will prevent cascade tests failing
    let helper = TESTS_DB_HELPER
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    helper.tear_down()
}

/// Lifts database, then fills it with metadata fixtures
pub(crate) fn spawn_test_db_with_metadata() -> (IndexesProcessor, Connection, Vec<FilterEntity>) {
    let connection_source = DatabasePathHolder::factory_test().unwrap();
    let (index, index_i18n) = indexes_fixtures::build_filters_indices_fixtures().unwrap();

    let mut indexes_processor =
        IndexesProcessor::factory_test(connection_source.clone(), index, index_i18n);

    // TODO: `connect` won't lift database here, cause it lifts only once, for one thread and one prefix
    lift_up_database(&connection_source).unwrap();

    indexes_processor.fill_empty_db().unwrap();

    let conn = connect(connection_source.get_calculated_path()).unwrap();

    let inserted_filters = FilterRepository::new()
        .select_filters_except_bootstrapped(&conn)
        .unwrap()
        .unwrap();

    (indexes_processor, conn, inserted_filters)
}

/// Helper for test database naming
pub(crate) fn do_with_tests_helper<'a, F, R>(f: F) -> R
where
    F: FnOnce(MutexGuard<TestsDb>) -> R,
{
    TEARDOWN_HACK.call_once(set_at_exit_hook);
    let helper = TESTS_DB_HELPER
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    f(helper)
}
