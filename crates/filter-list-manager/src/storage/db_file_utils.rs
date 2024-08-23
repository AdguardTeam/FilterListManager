use std::fs;
use std::path::PathBuf;

/// Check that db file exists
///
/// * `db_path` - How to check filename, with path or db_type
pub(crate) fn is_db_file_exists(db_path: &PathBuf) -> bool {
    fs::metadata(db_path).is_ok()
}
