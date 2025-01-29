use crate::storage::repositories::db_schema_repository::DbSchemaRepository;
use crate::storage::repositories::filter_repository::{FilterRepository, FILTER_TABLE_NAME};
use crate::{FLMError, FLMResult};
use rusqlite::Transaction;
use std::fs;
use std::path::PathBuf;

/// Current database status
pub(crate) enum DatabaseStatus {
    /// DB file does not exist
    NoFile,
    /// File exists, but has no schema or file corrupted
    NoSchema,
    /// File has schema, but is not filled with data
    OnlySchema,
    /// Db is fully operational
    Filled,
}

#[allow(clippy::bool_comparison)]
/// Determines current [`DatabaseStatus`]
pub(crate) fn get_database_status(
    tx: &Transaction,
    db_file_path: &PathBuf,
) -> FLMResult<DatabaseStatus> {
    if is_db_file_exists(db_file_path) == false {
        return Ok(DatabaseStatus::NoFile);
    }

    let is_filters_table_exists = DbSchemaRepository::new()
        .is_table_exists(tx, FILTER_TABLE_NAME)
        .map_err(FLMError::from_database)?;

    if is_filters_table_exists == false {
        return Ok(DatabaseStatus::NoSchema);
    }

    let at_least_one_record_exist = FilterRepository::new()
        .has_at_least_one_record(tx)
        .map_err(FLMError::from_database)?;

    if at_least_one_record_exist == false {
        return Ok(DatabaseStatus::OnlySchema);
    }

    Ok(DatabaseStatus::Filled)
}

/// Check that db file exists
///
/// * `db_path` - Path to database file
#[inline]
pub(crate) fn is_db_file_exists(db_path: &PathBuf) -> bool {
    fs::metadata(db_path).is_ok()
}

/// Recursively creates DB folder if it does not exist
#[inline]
pub(crate) fn create_db_folder_if_it_does_not_exist(mut db_path: PathBuf) -> FLMResult<()> {
    // Pops filename
    db_path.pop();

    if !db_path.exists() {
        // Tries to create recursive path
        fs::create_dir_all(db_path).map_err(FLMError::from_io)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::storage::database_status::create_db_folder_if_it_does_not_exist;
    use crate::storage::DbConnectionManager;
    use crate::Configuration;
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::{env, fs};

    #[test]
    fn test_recursive_folder_creation() {
        let mut path = env::current_dir().unwrap();
        path.push("fixtures");
        path.push("nonexistent_directory");

        if path.exists() {
            let files = fs::read_dir(path.clone()).unwrap();

            for file in files {
                fs::remove_file(file.unwrap().path()).unwrap();
            }

            fs::remove_dir(path.clone()).unwrap();
        }

        path.push(format!(
            "test_recursive_folder_creation-{}.db",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ));

        let conf = Configuration::default();
        let holder = DbConnectionManager::from_configuration(&conf).unwrap();

        create_db_folder_if_it_does_not_exist(holder.get_calculated_path().to_owned()).unwrap();
    }
}
