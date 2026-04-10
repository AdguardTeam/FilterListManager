use crate::storage::repositories::db_metadata_repository::DBMetadataRepository;
use crate::{FLMError, FLMResult};
use include_dir::{include_dir, Dir, File};
use regex::Regex;
use rusqlite::Transaction;
use std::cell::Cell;

/// Regex for matching migration files
const FILE_MATCHING_REGEX: &str = r"(\d+)-migration.sql";

/// Embed migrations
const MIGRATIONS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/resources/sql/migrations");

/// Consistently applies migrations that have not yet been applied to the current database
pub(super) fn run_migrations(tx: &mut Transaction) -> FLMResult<()> {
    migrations_internal(&MIGRATIONS_DIR, tx)
}

/// Runner
#[inline]
fn migrations_internal(dir: &Dir, tx: &mut Transaction) -> FLMResult<()> {
    let current_schema_version = DBMetadataRepository::read_for_migration(tx)
        .map_err(FLMError::from_database)?
        .unwrap_or_default();

    let next_schema_version = Cell::new(current_schema_version);

    for_each_migration_file(dir, |file_version, file| {
        if file_version > current_schema_version {
            if let Some(contents) = file.contents_utf8() {
                tx.execute_batch(contents)
                    .map_err(FLMError::from_database)?;
            }

            next_schema_version.set(file_version);
        }

        Ok(())
    })?;

    // No new migrations
    if current_schema_version == next_schema_version.get() {
        return Ok(());
    }

    DBMetadataRepository::save_for_migration(tx, next_schema_version.get())
        .map_err(FLMError::from_database)?;

    Ok(())
}

/// Creates and runs an iterator over migration files
fn for_each_migration_file<Block>(dir: &Dir, block: Block) -> FLMResult<()>
where
    Block: Fn(i32, &File) -> FLMResult<()>,
{
    let mut dir_iterator: Vec<(&str, &File)> = dir
        .files()
        .filter_map(|file| {
            if let Some(last_component) = file.path().components().last() {
                if let Some(filename) = last_component.as_os_str().to_str() {
                    return Some((filename, file));
                }
            }

            None
        })
        .collect();

    let regex = Regex::new(FILE_MATCHING_REGEX).map_err(FLMError::from_display)?;

    dir_iterator.sort_by_key(|a| a.0);

    for (filename, file) in dir_iterator {
        if let Some(captures) = regex.captures(filename) {
            if captures.len() > 0 {
                let index = captures[1].parse::<i32>().map_err(FLMError::from_display)?;

                block(index, file)?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::storage::migrations::{
        for_each_migration_file, migrations_internal, run_migrations,
    };
    use crate::storage::repositories::db_metadata_repository::CREATE_METADATA_SHAPE;
    use include_dir::{Dir, File};
    use std::cell::RefCell;

    #[test]
    fn test_for_each_migration_file() {
        let entries = [
            include_dir::DirEntry::File(File::new(".gitignore", b"")),
            include_dir::DirEntry::File(File::new("002-migration.sql", b"2")),
            include_dir::DirEntry::Dir(Dir::new("003-migration.sql", &[])),
            include_dir::DirEntry::File(File::new("001-migration.sql", b"1")),
        ];

        let dir = Dir::new("", &entries);

        let actual_paths: RefCell<Vec<String>> = RefCell::new(vec![]);
        let actual_contents: RefCell<Vec<String>> = RefCell::new(vec![]);

        for_each_migration_file(&dir, |_, file| {
            actual_paths
                .borrow_mut()
                .push(String::from(file.path().as_os_str().to_str().unwrap()));
            actual_contents
                .borrow_mut()
                .push(file.contents_utf8().unwrap().to_string());

            Ok(())
        })
        .unwrap();

        assert_eq!(
            actual_paths.into_inner(),
            &["001-migration.sql", "002-migration.sql"]
        );
        assert_eq!(actual_contents.into_inner(), &["1", "2"]);
    }

    #[test]
    fn test_migration() {
        let mut conn = rusqlite::Connection::open_in_memory().unwrap();

        let initial_db = format!(
            r###"
            {}

            CREATE TABLE [filter] (
               [filter_id] INTEGER PRIMARY KEY,
               [text] TEXT
            );

            INSERT INTO [filter] ([filter_id], [text])
            VALUES
                (1, "First filter"),
                (2, "Second filter");
        "###,
            CREATE_METADATA_SHAPE
        );

        conn.execute_batch(&initial_db).unwrap();

        let migration_1: &str = r###"
            -- These fails if migrations runs twice
            ALTER TABLE [filter] ADD COLUMN [is_enabled] BOOLEAN NOT NULL DEFAULT 0;
            ALTER TABLE [filter] ADD COLUMN [is_installed] BOOLEAN NOT NULL DEFAULT 0;
        "###;

        let migration_2: &str = r###"
            INSERT INTO [filter] ([filter_id], [text], [is_enabled], [is_installed])
            VALUES
                (3, "Third filter", 1, 0),
                (4, "Fourth filter", 0, 1);
        "###;

        {
            let entries = [
                include_dir::DirEntry::File(File::new("001-migration.sql", migration_1.as_bytes())),
                include_dir::DirEntry::File(File::new("002-migration.sql", migration_2.as_bytes())),
            ];

            let dir = Dir::new("", &entries);

            let mut tx = conn.transaction().unwrap();
            migrations_internal(&dir, &mut tx).unwrap();
            tx.commit().unwrap();

            // Check new fields
            conn.query_row(
                r"
                SELECT
                    sql
                FROM
                    sqlite_schema
                WHERE
                    type='table' AND name='filter'
            ",
                (),
                |row| {
                    let sql: String = row.get(0).unwrap();

                    assert!(sql.contains("is_enabled"));
                    assert!(sql.contains("is_installed"));

                    Ok(())
                },
            )
            .unwrap();

            // Check insert
            conn.query_row(
                r"
                SELECT
                    COUNT(filter_id)
                FROM
                    [filter]
            ",
                (),
                |row| {
                    let count: i32 = row.get(0).unwrap();
                    assert_eq!(count, 4);

                    Ok(())
                },
            )
            .unwrap()
        }

        let migration_3: &str = r###"
            ALTER TABLE [filter] DROP COLUMN [is_enabled];
            ALTER TABLE [filter] ADD COLUMN [is_trusted] BOOLEAN NOT NULL DEFAULT 0;
        "###;

        let migration_4: &str = r###"
            ALTER TABLE [filter] DROP COLUMN [is_trusted];
            INSERT INTO [filter] ([filter_id], [text], [is_installed])
            VALUES
                (5, "Fifth filter", 0);
        "###;

        {
            let entries = [
                include_dir::DirEntry::File(File::new("001-migration.sql", migration_1.as_bytes())),
                include_dir::DirEntry::File(File::new("002-migration.sql", migration_2.as_bytes())),
                include_dir::DirEntry::File(File::new("003-migration.sql", migration_3.as_bytes())),
                include_dir::DirEntry::File(File::new("004-migration.sql", migration_4.as_bytes())),
            ];

            let dir = Dir::new("", &entries);

            let mut tx = conn.transaction().unwrap();
            migrations_internal(&dir, &mut tx).unwrap();
            tx.commit().unwrap();

            // Check fields removal
            conn.query_row(
                r"
                SELECT
                    sql
                FROM
                    sqlite_schema
                WHERE
                    type='table' AND name='filter'
            ",
                (),
                |row| {
                    let sql: String = row.get(0).unwrap();

                    assert!(!sql.contains("is_enabled"));
                    assert!(sql.contains("is_installed"));
                    assert!(!sql.contains("is_trusted"));

                    Ok(())
                },
            )
            .unwrap();

            // Check insert
            conn.query_row(
                r"
                SELECT
                    COUNT(filter_id)
                FROM
                    [filter]
            ",
                (),
                |row| {
                    let count: i32 = row.get(0).unwrap();
                    assert_eq!(count, 5);

                    Ok(())
                },
            )
            .unwrap()
        }
    }

    #[test]
    fn test_validate_migrations_syntax() {
        let mut conn = rusqlite::Connection::open_in_memory().unwrap();
        let mut tx = conn.transaction().unwrap();
        let initial_db = include_str!("../../resources/sql/schema.sql");

        tx.execute_batch(initial_db).unwrap();

        run_migrations(&mut tx).unwrap();
    }

    #[test]
    fn test_migration_when_filter_count_signature_column_is_missing() {
        let mut conn = rusqlite::Connection::open_in_memory().unwrap();

        let initial_db = format!(
            r#"
            {}

            INSERT INTO [metadata] (
                [rowid],
                [schema_version],
                [custom_filter_increment],
                [filter_count_signature]
            ) VALUES (
                1,
                7,
                -10000,
                'signed-count-before-tamper'
            );

            ALTER TABLE [metadata] DROP COLUMN [filter_count_signature];

            CREATE TABLE [filter] (
                [filter_id] INTEGER PRIMARY KEY,
                [text] TEXT
            );
        "#,
            CREATE_METADATA_SHAPE
        );

        conn.execute_batch(&initial_db).unwrap();

        let entries = [include_dir::DirEntry::File(File::new(
            "008-migration.sql",
            b"ALTER TABLE [filter] ADD COLUMN [dummy] INTEGER NOT NULL DEFAULT 0;",
        ))];
        let dir = Dir::new("", &entries);

        let mut tx = conn.transaction().unwrap();
        migrations_internal(&dir, &mut tx).unwrap();
        tx.commit().unwrap();

        conn.query_row(
            r"
                SELECT
                    [schema_version],
                    [custom_filter_increment]
                FROM
                    [metadata]
                WHERE
                    [rowid] = 1
            ",
            (),
            |row| {
                let schema_version: i32 = row.get(0).unwrap();
                let custom_filter_increment: i32 = row.get(1).unwrap();

                assert_eq!(schema_version, 8);
                assert_eq!(custom_filter_increment, -10000);

                Ok(())
            },
        )
        .unwrap();
    }

    #[test]
    fn test_migration_preserves_filter_count_signature_when_column_exists() {
        let mut conn = rusqlite::Connection::open_in_memory().unwrap();

        let initial_db = format!(
            r#"
            {}

            INSERT INTO [metadata] (
                [rowid],
                [schema_version],
                [custom_filter_increment],
                [filter_count_signature]
            ) VALUES (
                1,
                7,
                -10000,
                'signed-count-before-migration'
            );

            CREATE TABLE [filter] (
                [filter_id] INTEGER PRIMARY KEY,
                [text] TEXT
            );
        "#,
            CREATE_METADATA_SHAPE
        );

        conn.execute_batch(&initial_db).unwrap();

        let entries = [include_dir::DirEntry::File(File::new(
            "008-migration.sql",
            b"ALTER TABLE [filter] ADD COLUMN [dummy] INTEGER NOT NULL DEFAULT 0;",
        ))];
        let dir = Dir::new("", &entries);

        let mut tx = conn.transaction().unwrap();
        migrations_internal(&dir, &mut tx).unwrap();
        tx.commit().unwrap();

        conn.query_row(
            r"
                SELECT
                    [schema_version],
                    [filter_count_signature]
                FROM
                    [metadata]
                WHERE
                    [rowid] = 1
            ",
            (),
            |row| {
                let schema_version: i32 = row.get(0).unwrap();
                let filter_count_signature: Option<String> = row.get(1).unwrap();

                assert_eq!(schema_version, 8);
                assert_eq!(
                    filter_count_signature,
                    Some(String::from("signed-count-before-migration"))
                );

                Ok(())
            },
        )
        .unwrap();
    }
}
