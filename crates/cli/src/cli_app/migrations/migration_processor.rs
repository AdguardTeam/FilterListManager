use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;
use std::string::String;

use super::file_matcher::FileMatcher;
use super::migration_queries::{MigrationQueries, MigrationQueriesImpl};

/// CLI migration processor
pub(crate) struct MigrationProcessor {
    migration_queries: Box<dyn MigrationQueries>,
}

impl MigrationProcessor {
    /// Init with DB connection
    pub(crate) fn new(conn: Connection) -> Self {
        MigrationProcessor {
            migration_queries: Box::new(MigrationQueriesImpl::new(conn)),
        }
    }

    /// Process migration
    pub(crate) fn run(&self, migration_folder: &PathBuf) {
        let schema_version = self
            .migration_queries
            .get_schema_version()
            .expect("Cannot get schema version");

        let (collected_sql, next_schema_version) =
            self.collect_sql_from_files(schema_version, migration_folder);

        let t_result = self
            .migration_queries
            .execute_migration_queries(&collected_sql);

        match t_result {
            Ok(_) => {
                self.migration_queries
                    .update_schema_version(next_schema_version)
                    .expect("Cannot update schema version. Next migration may be failed");
            }
            Err(err) => {
                self.migration_queries
                    .rollback_migration()
                    .expect(format!("Cannot rollback migration transaction: {}", err).as_str());
            }
        }
    }

    fn collect_sql_from_files(
        &self,
        schema_version: i32,
        migration_folder: &PathBuf,
    ) -> (String, i32) {
        let mut read_dir: Vec<PathBuf> = match fs::read_dir(migration_folder) {
            Ok(read_dir) => read_dir.map(|entry| entry.unwrap().path()).collect(),
            Err(why) => panic!("Cannot read dir {}: {}", migration_folder.display(), why),
        };
        read_dir.sort();

        let file_matcher = FileMatcher::create_matcher()
            .expect("Cannot create regexp matcher. This is internal problem");

        let mut next_schema_version: i32 = 0;
        let mut queries_string: String = String::new();
        for entry_path in read_dir {
            let file_name = entry_path.file_name().unwrap().to_str().unwrap();
            let file_index = file_matcher.extract_migration_index(file_name).unwrap();

            // Skip previous migrations
            if file_index <= schema_version {
                continue;
            }

            let contents = fs::read_to_string(entry_path).expect("Unable to read file");

            queries_string += "\n";
            queries_string += contents.as_str();

            next_schema_version = file_index;
        }

        (queries_string, next_schema_version)
    }
}
