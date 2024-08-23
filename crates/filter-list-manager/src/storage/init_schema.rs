use crate::storage::force_connect;
use crate::{FLMError, FLMResult};
use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;

/// Database schema
const SCHEMA_STR: &str = include_str!("../../resources/sql/schema.sql");

/// Inits schema in database
pub(crate) fn init_schema(db_path: &PathBuf) -> FLMResult<Connection> {
    // Should check full path before
    let mut directory = db_path.clone();
    // Pops filename
    directory.pop();

    if !directory.exists() {
        // Tries to create recursive path
        fs::create_dir_all(directory).map_err(FLMError::from_io)?;
    }

    let conn = force_connect(db_path)?;

    conn.execute_batch(SCHEMA_STR)
        .map_err(FLMError::from_database)?;

    Ok(conn)
}

#[cfg(test)]
mod tests {
    use crate::storage::init_schema::init_schema;
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

        init_schema(&path).unwrap();
    }
}
