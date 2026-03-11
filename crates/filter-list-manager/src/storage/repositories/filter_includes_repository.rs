use crate::storage::blob::{BlobHandleImpl, BLOB_CHUNK_SIZE};
use crate::storage::entities::filter::filter_include_entity::FilterIncludeEntity;
use crate::storage::entities::filter::filter_include_metadata_entity::FilterIncludeMetadataEntity;
use crate::storage::repositories::Repository;
use crate::storage::sql_generators::operator::SQLOperator;
use crate::storage::utils::{build_in_clause, process_where_clause};
use crate::storage::Hydrate;
use crate::utils::integrity::{sign_content, verify_content};
use crate::FilterId;
use blake3::Hasher;
use rusqlite::{named_params, params_from_iter, Connection, DatabaseName, Error, Transaction};
use std::collections::HashMap;

pub(crate) type MapFilterIdOnFilterIncludes = HashMap<FilterId, Vec<FilterIncludeEntity>>;

/// Basic SQL-query with all fields
const BASIC_SELECT_SQL: &str = r"
    SELECT
        row_id,
        filter_id,
        absolute_url,
        body,
        rules_count,
        body_hash,
        integrity_signature
    FROM
        [filter_includes]
";

/// Repository for [`FilterIncludeEntity`]
pub(crate) struct FilterIncludesRepository;

impl FilterIncludesRepository {
    /// Ctor
    pub const fn new() -> Self {
        Self {}
    }

    /// Does few things:
    /// 1. deletes all includes for [`FilterId`] from all entities
    /// 2. inserts passed includes
    ///
    pub(crate) fn replace_entities_for_filters(
        &self,
        tx: &Transaction<'_>,
        entities: &[FilterIncludeEntity],
    ) -> rusqlite::Result<()> {
        if entities.is_empty() {
            return Ok(());
        }

        self.delete_for_filters(
            tx,
            entities.iter().map(|entity| entity.filter_id),
            entities.len(),
        )?;

        self.insert(tx, entities)
    }

    /// Gets rules_counts for list of [`FilterId`]
    pub(crate) fn get_rules_count_for_filters(
        &self,
        conn: &Connection,
        filters_ids: &[FilterId],
    ) -> rusqlite::Result<HashMap<FilterId, i32>> {
        if filters_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let sql = format!(
            r"
                SELECT
                    filter_id,
                    SUM(rules_count) as rules_count
                FROM
                    [filter_includes]
                WHERE
                    {}
                GROUP BY
                    filter_id",
            build_in_clause("filter_id", filters_ids.len())
        );

        let params = params_from_iter(filters_ids);

        let mut statement = conn.prepare(sql.as_str())?;
        let mut rows = statement.query(params)?;

        let mut out = HashMap::new();
        while let Some(row) = rows.next()? {
            out.insert(row.get(0)?, row.get(1)?);
        }

        Ok(out)
    }

    /// Deletes includes for list of [`FilterId`]
    pub(crate) fn delete_for_filters(
        &self,
        tx: &Transaction<'_>,
        ids: impl Iterator<Item = FilterId>,
        len: usize,
    ) -> rusqlite::Result<()> {
        let mut sql = String::from(
            r"
                DELETE FROM
                    [filter_includes]
                WHERE
                   ",
        );

        sql += build_in_clause("filter_id", len).as_str();

        let mut statement = tx.prepare(sql.as_str())?;
        statement.execute(params_from_iter(ids)).map(|_| ())
    }

    /// Gets entities mapped by [`FilterId`]
    pub(crate) fn select_mapped(
        &self,
        conn: &Connection,
        where_clause: Option<SQLOperator>,
    ) -> rusqlite::Result<MapFilterIdOnFilterIncludes> {
        let mut sql = String::from(BASIC_SELECT_SQL);
        let params = process_where_clause(&mut sql, where_clause)?;
        let mut statement = conn.prepare(sql.as_str())?;

        let rows = statement.query_map(params, FilterIncludeEntity::hydrate)?;

        let mut results = HashMap::new();
        for row in rows {
            let unwrapped = row?;

            results
                .entry(unwrapped.filter_id)
                .or_insert(vec![])
                .push(unwrapped);
        }

        Ok(results)
    }
}

impl FilterIncludesRepository {
    /// Iterates over all filter_includes rows, computes integrity signature for each
    /// using the derived key, and collects `(row_id, signature)` pairs
    /// without loading all include bodies into memory at once.
    pub(crate) fn sign_and_collect_signatures_streaming(
        &self,
        conn: &Connection,
        derived_key: &[u8; 32],
    ) -> rusqlite::Result<Vec<(i64, String)>> {
        let mut statement = conn.prepare(
            r"
            SELECT
                row_id,
                filter_id,
                body
            FROM
                [filter_includes]",
        )?;

        let mut signatures = Vec::new();
        let mut rows = statement.query([])?;

        while let Some(row) = rows.next()? {
            let row_id: i64 = row.get(0)?;
            let filter_id: FilterId = row.get(1)?;
            let body: String = row.get(2)?;

            let sig = sign_content(derived_key, filter_id, &body);
            signatures.push((row_id, sig));
        }

        Ok(signatures)
    }

    /// Iterates over all filter_includes rows and verifies integrity signatures
    /// without loading all include bodies into memory at once.
    /// Returns the `filter_id` of the first entity that fails verification.
    pub(crate) fn verify_all_streaming(
        &self,
        conn: &Connection,
        derived_key: &[u8; 32],
    ) -> rusqlite::Result<Option<FilterId>> {
        let mut statement = conn.prepare(
            r"
            SELECT
                filter_id,
                body,
                integrity_signature
            FROM
                [filter_includes]",
        )?;

        let mut rows = statement.query([])?;
        while let Some(row) = rows.next()? {
            let filter_id: FilterId = row.get(0)?;
            let body: String = row.get(1)?;
            let signature: Option<String> = row.get(2)?;

            if let Some(ref sig) = signature {
                if !verify_content(derived_key, filter_id, &body, sig) {
                    return Ok(Some(filter_id));
                }
            } else {
                return Ok(Some(filter_id));
            }
        }

        Ok(None)
    }

    /// Batch updates integrity_signature by row_id from `(row_id, signature)` pairs.
    pub(crate) fn batch_update_signatures(
        &self,
        tx: &Transaction<'_>,
        signatures: &[(i64, String)],
    ) -> rusqlite::Result<()> {
        let mut statement = tx.prepare(
            r"
            UPDATE
                [filter_includes]
            SET
                integrity_signature = :sig
            WHERE
                row_id = :row_id",
        )?;

        for (row_id, sig) in signatures {
            statement.execute(named_params! {
                ":row_id": row_id,
                ":sig": sig,
            })?;
        }

        Ok(())
    }
}

impl FilterIncludesRepository {
    /// Gets lightweight metadata for includes of a single filter, without loading body.
    /// Returns Vec of (row_id, filter_id, absolute_url, integrity_signature).
    pub(crate) fn get_include_metadata_for_filter(
        &self,
        conn: &Connection,
        filter_id: FilterId,
    ) -> rusqlite::Result<Vec<FilterIncludeMetadataEntity>> {
        let mut statement = conn.prepare(
            r"
            SELECT
                row_id,
                filter_id,
                absolute_url,
                integrity_signature
            FROM
                [filter_includes]
            WHERE
                filter_id = ?
        ",
        )?;

        let mut rows = statement.query([filter_id])?;
        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            out.push(FilterIncludeMetadataEntity::from_row(row)?);
        }

        Ok(out)
    }

    /// Verifies integrity of include body by streaming the blob through blake3
    /// incremental hasher, without loading the full body into memory.
    /// Returns `true` if the signature matches.
    pub(crate) fn verify_include_blob_integrity_streaming(
        &self,
        conn: &Connection,
        derived_key: &[u8; 32],
        metadata: &FilterIncludeMetadataEntity,
    ) -> rusqlite::Result<bool> {
        let signature = match metadata.integrity_signature {
            Some(ref sig) => sig,
            None => return Ok(false),
        };

        let blob = conn.blob_open(
            DatabaseName::Main,
            Self::TABLE_NAME,
            "body",
            metadata.row_id,
            true,
        )?;

        let mut hasher = Hasher::new_keyed(derived_key);
        hasher.update(&metadata.filter_id.to_le_bytes());

        let mut buffer = vec![0u8; BLOB_CHUNK_SIZE];
        let mut offset = 0;
        loop {
            let bytes_read = blob.read_at(&mut buffer, offset)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
            offset += bytes_read;
        }

        let computed = hasher.finalize();
        Ok(computed.to_hex().as_str() == signature.as_str())
    }

    /// Opens a blob handle on include body by row_id for streaming into output.
    pub(crate) fn get_include_blob_handle<'a>(
        &self,
        conn: &'a Connection,
        row_id: i64,
    ) -> rusqlite::Result<BlobHandleImpl<'a>> {
        let blob = conn.blob_open(DatabaseName::Main, Self::TABLE_NAME, "body", row_id, true)?;

        Ok(BlobHandleImpl::new(blob))
    }
}

impl Repository<FilterIncludeEntity> for FilterIncludesRepository {
    const TABLE_NAME: &'static str = "filter_includes";

    fn insert(
        &self,
        conn: &Transaction<'_>,
        entities: &[FilterIncludeEntity],
    ) -> rusqlite::Result<(), Error> {
        let mut statement = conn.prepare(
            r"
                INSERT OR REPLACE INTO
                    [filter_includes]
                    (
                        row_id,
                        filter_id,
                        absolute_url,
                        body,
                        rules_count,
                        body_hash,
                        integrity_signature
                    )
                VALUES
                    (
                        :row_id,
                        :filter_id,
                        :absolute_url,
                        :body,
                        :rules_count,
                        :body_hash,
                        :integrity_signature
                    )
                ",
        )?;

        for entity in entities.iter() {
            statement.execute(named_params! {
                ":row_id": entity.row_id,
                ":filter_id": entity.filter_id,
                ":absolute_url": entity.absolute_url,
                ":body": entity.body,
                ":rules_count": entity.rules_count,
                ":body_hash": entity.body_hash,
                ":integrity_signature": entity.integrity_signature,
            })?;
        }

        Ok(())
    }
}
