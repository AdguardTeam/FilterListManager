pub(crate) mod indexes_fixtures;
pub(crate) mod tests_db;

use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, Once};
use tests_db::TestsDb;

use crate::filters::indexes::indexes_processor::IndexesProcessor;
use crate::io::http::blocking_client::BlockingClient;
use crate::storage::entities::filter::filter_entity::FilterEntity;
use crate::storage::repositories::filter_repository::FilterRepository;
use crate::storage::DbConnectionManager;
use crate::{Configuration, FLMError};
use libc::atexit;
use rusqlite::Connection;

/// Temporary databases filenames manipulator
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
pub(crate) fn spawn_test_db_with_metadata(
    connection_source: &DbConnectionManager,
) -> (IndexesProcessor, Vec<FilterEntity>) {
    let (index, index_i18n) = indexes_fixtures::build_filters_indices_fixtures().unwrap();

    let mut indexes_processor =
        IndexesProcessor::factory_test(connection_source, index, index_i18n);

    unsafe { connection_source.lift_up_database().unwrap() }

    let inserted_filters = connection_source
        .execute_db(|mut connection: Connection| {
            indexes_processor.fill_empty_db(&mut connection).unwrap();

            FilterRepository::new()
                .select_filters_except_bootstrapped(&connection)
                .map_err(FLMError::from_database)
        })
        .unwrap()
        .unwrap();

    (indexes_processor, inserted_filters)
}

/// Helper for test database naming
pub(crate) fn do_with_database_names_manipulator<'a, F, R>(f: F) -> R
where
    F: FnOnce(MutexGuard<TestsDb>) -> R,
{
    TEARDOWN_HACK.call_once(set_at_exit_hook);
    let helper = TESTS_DB_HELPER
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    f(helper)
}

/// I hope this file will be "almost" always removed after the tests
pub(crate) struct RAIIFile(PathBuf);

impl RAIIFile {
    pub(crate) fn new(path: &PathBuf, contents: &str) -> Self {
        fs::write(&path, contents).unwrap();
        Self(path.to_path_buf())
    }
}

impl Drop for RAIIFile {
    fn drop(&mut self) {
        if fs::metadata(&self.0).is_ok() {
            fs::remove_file(&self.0).unwrap()
        }
    }
}

/// Builds [`PathBuf`] from string path, relative to tests folder
pub(crate) fn tests_path(relative_path: &'static str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push(relative_path);

    path
}

lazy_static! {
    /// Default blocking http client for testing purposes
    pub(crate) static ref SHARED_TEST_BLOCKING_HTTP_CLIENT: BlockingClient = BlockingClient::new(&Configuration::default()).unwrap();
}
