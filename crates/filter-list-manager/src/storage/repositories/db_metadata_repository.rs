use crate::storage::entities::db_metadata_entity::DBMetadataEntity;
use rusqlite::{named_params, Connection, OptionalExtension, Row, Transaction};

/// Repository for `metadata` table. We store here misc app settings.
pub(crate) struct DBMetadataRepository;

impl DBMetadataRepository {
    pub(crate) fn read(connection: &Connection) -> rusqlite::Result<Option<DBMetadataEntity>> {
        let mut statement = connection.prepare(
            r"
            SELECT
                schema_version,
                custom_filter_increment
            FROM
                [metadata]
            WHERE
                rowid = 1
        ",
        )?;

        let row = statement
            .query_row((), DBMetadataRepository::hydrate)
            .optional()?;

        Ok(row)
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
                custom_filter_increment
            )
            VALUES
            (
                1,
                :schema_version,
                :custom_filter_increment
            )
        ",
        )?;

        statement.execute(named_params! {
            ":schema_version": entity.version,
            ":custom_filter_increment": entity.custom_filters_autoincrement_value
        })?;

        Ok(())
    }

    pub(crate) fn hydrate(row: &Row) -> rusqlite::Result<DBMetadataEntity> {
        Ok(DBMetadataEntity {
            version: row.get(0)?,
            custom_filters_autoincrement_value: row.get(1)?,
        })
    }
}
