use adguard_flm::storage::force_connect;
use std::path::PathBuf;

mod file_matcher;
mod migration_processor;
mod migration_queries;

use migration_processor::MigrationProcessor;

pub(super) fn entry(migration_folder: &PathBuf, db_path: &PathBuf) {
    let conn = force_connect(db_path).unwrap();
    MigrationProcessor::new(conn).run(migration_folder);

    println!("Migration success!");
}
