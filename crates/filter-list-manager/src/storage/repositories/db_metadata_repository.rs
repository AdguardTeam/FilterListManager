use crate::storage::entities::db_metadata_entity::DBMetadataEntity;
use crate::storage::entities::hydrate::Hydrate;
use rusqlite::{named_params, Connection, OptionalExtension, Transaction};

/// Keeps tests aligned with current metadata write contract in `save()`.
/// Used only in tests that build ad-hoc DB schemas manually.
#[cfg(test)]
pub(crate) const CREATE_METADATA_SHAPE: &str = r"
CREATE TABLE [metadata] (
    [rowid] INTEGER PRIMARY KEY,
    [schema_version] INTEGER NOT NULL,
    [custom_filter_increment] INTEGER NOT NULL,
    [filter_count_signature] TEXT
);";

/// Basic SQL-query with all fields
const BASIC_SELECT_SQL: &str = r"
    SELECT
        schema_version,
        custom_filter_increment,
        filter_count_signature
    FROM
        [metadata]
    WHERE
        rowid = 1
";

/// Repository for `metadata` table. We store here misc app settings.
pub(crate) struct DBMetadataRepository;

impl DBMetadataRepository {
    pub(crate) fn read(connection: &Connection) -> rusqlite::Result<Option<DBMetadataEntity>> {
        let mut statement = connection.prepare(BASIC_SELECT_SQL)?;

        let row = statement
            .query_row((), DBMetadataEntity::hydrate)
            .optional()?;

        Ok(row)
    }

    /// Migration-safe read: only reads metadata schema version.
    pub(in crate::storage) fn read_for_migration(
        tx: &Transaction,
    ) -> rusqlite::Result<Option<i32>> {
        tx.query_row(
            r"
            SELECT
                schema_version
            FROM
                [metadata]
            WHERE
                rowid = 1
        ",
            [],
            |row| row.get(0),
        )
        .optional()
    }

    /// Migration-safe save: updates only `schema_version` and never rewrites
    /// the full metadata row.
    ///
    /// This avoids accidental data loss from `INSERT OR REPLACE` during
    /// migrations. If `rowid = 1` is missing, we insert a new row and take all
    /// non-version defaults from `DBMetadataEntity::default()` so defaults are
    /// controlled in one place.
    ///
    /// Intentionally fail-fast for corrupted/tampered metadata shape:
    /// we do not add runtime fallbacks for removed required columns.
    pub(in crate::storage) fn save_for_migration(
        transaction: &Transaction,
        schema_version: i32,
    ) -> rusqlite::Result<()> {
        let mut update_statement = transaction.prepare(
            r"
            UPDATE
                [metadata]
            SET
                [schema_version] = :schema_version
            WHERE
                [rowid] = 1
        ",
        )?;

        let affected = update_statement.execute(named_params! {
            ":schema_version": schema_version,
        })?;

        if affected > 0 {
            return Ok(());
        }

        let mut metadata = DBMetadataEntity::default();
        metadata.version = schema_version;

        let mut insert_statement = transaction.prepare(
            r"
            INSERT INTO
                [metadata]
            (
                [rowid],
                [schema_version],
                [custom_filter_increment],
                [filter_count_signature]
            ) VALUES (
                1,
                :schema_version,
                :custom_filter_increment,
                :filter_count_signature
            )
        ",
        )?;

        insert_statement.execute(named_params! {
            ":schema_version": metadata.version,
            ":custom_filter_increment": metadata.custom_filters_autoincrement_value,
            ":filter_count_signature": metadata.filter_count_signature,
        })?;

        Ok(())
    }

    pub(crate) fn save(
        transaction: &Transaction,
        entity: &DBMetadataEntity,
    ) -> rusqlite::Result<()> {
        let mut statement = transaction.prepare(
            r"
            INSERT OR REPLACE INTO
                [metadata]
            (
                rowid,
                schema_version,
                custom_filter_increment,
                filter_count_signature
            ) VALUES (
                1,
                :schema_version,
                :custom_filter_increment,
                :filter_count_signature
            )
        ",
        )?;

        statement.execute(named_params! {
            ":schema_version": entity.version,
            ":custom_filter_increment": entity.custom_filters_autoincrement_value,
            ":filter_count_signature": entity.filter_count_signature,
        })?;

        Ok(())
    }
}
