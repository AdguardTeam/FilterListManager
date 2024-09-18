use crate::{FLMError, FLMResult};
use std::fs;
use std::path::PathBuf;

/// Check that db file exists
///
/// * `db_path` - Path to database file
pub(crate) fn is_db_file_exists(db_path: &PathBuf) -> bool {
    fs::metadata(db_path).is_ok()
}

/// Makes directory tree by path
///
/// * `db_path` - Path to database file
pub(crate) fn make_directory_for_db_file_if_is_not_exists(db_path: &mut PathBuf) -> FLMResult<()> {
    db_path.pop();

    fs::create_dir_all(db_path).map_err(FLMError::from_io)
}
