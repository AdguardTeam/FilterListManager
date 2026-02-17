use crate::manager::models::FilterId;
use crate::storage::blob::BlobHandleImpl;
use crate::storage::entities::hydrate::Hydrate;
use crate::storage::entities::rules_list::disabled_rules_entity::DisabledRulesEntity;
use crate::storage::entities::rules_list::rules_count_entity::RulesCountEntity;
use crate::storage::entities::rules_list::rules_list_entity::RulesListEntity;
use crate::storage::repositories::{BulkDeleteRepository, Repository};
use crate::storage::sql_generators::operator::SQLOperator;
use crate::storage::utils::{build_in_clause, process_where_clause};
use crate::utils::integrity::{sign_content, verify_content};
use rusqlite::{
    named_params, params_from_iter, Connection, Error, OptionalExtension, Row, Transaction,
};
use rusqlite::{DatabaseName, Result};
use std::collections::HashMap;

pub(crate) type MapFilterIdOnRulesList = HashMap<FilterId, RulesListEntity>;

pub(crate) type MapFilterIdOnRulesString = HashMap<FilterId, String>;

/// Basic SQL-query with all fields
const BASIC_SELECT_SQL: &str = r"
    SELECT
        filter_id,
        rules_text,
        disabled_rules_text,
        rules_count,
        has_directives,
        text_hash,
        integrity_signature
    FROM
        [rules_list]
";

/// Repository for rules_list table
pub(crate) struct RulesListRepository;

impl RulesListRepository {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    #[cfg(test)]
    /// Gets all rules, except bootstrapped
    pub(crate) fn select_rules_except_bootstrapped(
        &self,
        connection: &Connection,
    ) -> Result<Option<Vec<RulesListEntity>>> {
        use crate::storage::db_bootstrap::get_bootstrapped_filter_id;
        use crate::storage::sql_generators::operator::SQLOperator::{FieldEqualValue, Not};
        use crate::utils::memory::heap;

        self.select(
            &connection,
            Some(Not(heap(FieldEqualValue(
                "filter_id",
                get_bootstrapped_filter_id().into(),
            )))),
        )
    }

    #[cfg(test)]
    /// Forces setting integrity_signature for a filter (for testing purposes)
    pub(crate) fn force_set_integrity_signature(
        &self,
        connection: &Connection,
        filter_id: FilterId,
        integrity_signature: Option<String>,
    ) -> Result<usize> {
        let sql = r"
            UPDATE
                [rules_list]
            SET
                integrity_signature = :integrity_signature
            WHERE
                filter_id = :filter_id
        ";

        let mut statement = connection.prepare(sql)?;

        statement.execute(named_params! {
            ":integrity_signature": integrity_signature,
            ":filter_id": filter_id
        })
    }

    /// Updates just `disabled_rules_text` column
    pub(crate) fn set_disabled_rules(
        &self,
        tx: &Transaction<'_>,
        filter_id: FilterId,
        disabled_rules: String,
    ) -> Result<usize> {
        let sql: &str = r"
            UPDATE
                [rules_list]
            SET
                disabled_rules_text = :disabled_rules_text
            WHERE
                filter_id = :filter_id
        ";

        let mut statement = tx.prepare(sql)?;

        statement.execute(named_params! {
            ":disabled_rules_text": disabled_rules,
            ":filter_id": filter_id
        })
    }

    /// Counts results by `where_clause`
    pub(crate) fn count(
        &self,
        connection: &Connection,
        where_clause: Option<SQLOperator>,
    ) -> Result<i32> {
        let mut sql = String::from(
            r"
            SELECT
                COUNT(1)
            FROM
                [rules_list]
        ",
        );

        let params = process_where_clause(&mut sql, where_clause)?;
        let mut statement = connection.prepare(sql.as_str())?;

        statement.query_row(params, |row: &Row| row.get(0))
    }

    /// Gets entities mapped by [`FilterId`]
    pub(crate) fn select_mapped(
        &self,
        conn: &Connection,
        where_clause: Option<SQLOperator>,
    ) -> Result<MapFilterIdOnRulesList> {
        let mut sql = String::from(BASIC_SELECT_SQL);
        let params = process_where_clause(&mut sql, where_clause)?;
        let mut statement = conn.prepare(sql.as_str())?;

        let rows = statement.query_map(params, RulesListEntity::hydrate)?;

        let mut results = HashMap::new();
        for row in rows {
            let unwrapped = row?;
            results.insert(unwrapped.filter_id, unwrapped);
        }

        Ok(results)
    }

    /// Gets rules strings, disabled_rules strings and rules hashes mapped by [`FilterId`] for provided `for_ids`
    pub(crate) fn select_rules_maps(
        &self,
        conn: &Connection,
        for_ids: &[FilterId],
    ) -> Result<(
        MapFilterIdOnRulesString,
        MapFilterIdOnRulesString,
        MapFilterIdOnRulesString,
    )> {
        let mut sql = String::from(
            r"
            SELECT
                filter_id,
                rules_text,
                disabled_rules_text,
                rules_count,
                has_directives,
                text_hash,
                integrity_signature
            FROM
                [rules_list]
            WHERE ",
        );

        sql += build_in_clause("filter_id", for_ids.len()).as_str();

        let mut statement = conn.prepare(sql.as_str())?;

        let mut rows = statement.query(params_from_iter(for_ids))?;

        let mut rules = HashMap::new();
        let mut disabled_rules = HashMap::new();
        let mut text_hashes = HashMap::new();
        while let Some(row) = rows.next()? {
            let id: FilterId = row.get(0)?;

            rules.insert(id, row.get(1)?);
            disabled_rules.insert(id, row.get(2)?);

            let text_hash = row.get::<usize, Option<String>>(5)?;
            text_hashes.insert(id, text_hash.unwrap_or_default());
        }

        Ok((rules, disabled_rules, text_hashes))
    }

    pub(crate) fn select(
        &self,
        connection: &Connection,
        where_clause: Option<SQLOperator>,
    ) -> Result<Option<Vec<RulesListEntity>>> {
        let mut sql = String::from(BASIC_SELECT_SQL);
        let params = process_where_clause(&mut sql, where_clause)?;
        let mut statement = connection.prepare(sql.as_str())?;

        let option = statement
            .query_map(params, RulesListEntity::hydrate)
            .optional()?;

        let Some(rows) = option else {
            return Ok(None);
        };

        let mut results = vec![];
        for row in rows {
            results.push(row?);
        }

        if results.is_empty() {
            return Ok(None);
        }

        Ok(Some(results))
    }

    pub(crate) fn get_blob_handle_and_disabled_rules<'a>(
        &'a self,
        connection: &'a Connection,
        filter_id: FilterId,
    ) -> Result<(Vec<u8>, BlobHandleImpl<'a>)> {
        let mut statement = connection.prepare(
            r"
            SELECT
                rowid,
                CAST(disabled_rules_text AS BLOB)
            FROM
                [rules_list]
            WHERE
                filter_id = ?
        ",
        )?;

        let (row_id, disabled_rules) = statement.query_row([filter_id], |row| {
            Ok((row.get::<usize, i64>(0)?, row.get::<usize, Vec<u8>>(1)?))
        })?;

        let blob = connection.blob_open(
            DatabaseName::Main,
            Self::TABLE_NAME,
            "rules_text",
            row_id,
            true,
        )?;

        Ok((disabled_rules, BlobHandleImpl::new(blob)))
    }

    pub(crate) fn get_disabled_rules_by_ids(
        &self,
        connection: &Connection,
        ids: &[FilterId],
    ) -> Result<Vec<DisabledRulesEntity>> {
        let mut sql = String::from(
            r"
            SELECT
                filter_id,
                disabled_rules_text
            FROM
                [rules_list]
            WHERE ",
        );

        sql += build_in_clause("filter_id", ids.len()).as_str();
        let params = params_from_iter(ids);

        let mut statement = connection.prepare(sql.as_str())?;

        let mut out = vec![];
        let Some(rows) = statement
            .query_map(params, DisabledRulesEntity::hydrate)
            .optional()?
        else {
            return Ok(out);
        };

        for row in rows {
            out.push(row?);
        }

        Ok(out)
    }

    pub(crate) fn get_rules_count(
        &self,
        connection: &Connection,
        ids: &[FilterId],
    ) -> Result<Vec<RulesCountEntity>> {
        let mut sql = String::from(
            r"
            SELECT
                filter_id,
                rules_count
            FROM
                [rules_list]
            WHERE ",
        );

        sql += build_in_clause("filter_id", ids.len()).as_str();
        let params = params_from_iter(ids);

        let mut statement = connection.prepare(sql.as_str())?;

        let mut out = vec![];
        let Some(rows) = statement
            .query_map(params, RulesCountEntity::hydrate)
            .optional()?
        else {
            return Ok(out);
        };

        for row in rows {
            out.push(row?);
        }

        Ok(out)
    }
}

impl RulesListRepository {
    /// Iterates over all rules_list rows, computes integrity signature for each
    /// using the derived key, and collects `(filter_id, signature)` pairs
    /// without loading all rule bodies into memory at once.
    pub(crate) fn sign_and_collect_signatures_streaming(
        &self,
        conn: &Connection,
        derived_key: &[u8; 32],
    ) -> Result<Vec<(FilterId, String)>> {
        let mut statement = conn.prepare(
            r"
            SELECT
                filter_id,
                rules_text
            FROM
                [rules_list]",
        )?;

        let mut signatures = Vec::new();
        let mut rows = statement.query([])?;
        while let Some(row) = rows.next()? {
            let filter_id: FilterId = row.get(0)?;
            let text: String = row.get(1)?;

            let sig = sign_content(derived_key, filter_id, &text);
            signatures.push((filter_id, sig));
        }

        Ok(signatures)
    }

    /// Iterates over all rules_list rows and verifies integrity signatures
    /// without loading all rule bodies into memory at once.
    /// Returns the `filter_id` of the first entity that fails verification.
    pub(crate) fn verify_all_streaming(
        &self,
        conn: &Connection,
        derived_key: &[u8; 32],
    ) -> Result<Option<FilterId>> {
        let mut statement = conn.prepare(
            r"
                SELECT
                    filter_id,
                    rules_text,
                    integrity_signature
                FROM
                    [rules_list]",
        )?;

        let mut rows = statement.query([])?;
        while let Some(row) = rows.next()? {
            let filter_id: FilterId = row.get(0)?;
            let text: String = row.get(1)?;
            let signature: Option<String> = row.get(2)?;

            if let Some(ref sig) = signature {
                if !verify_content(derived_key, filter_id, &text, sig) {
                    return Ok(Some(filter_id));
                }
            } else {
                return Ok(Some(filter_id));
            }
        }

        Ok(None)
    }

    /// Batch updates integrity_signature by filter_id from `(filter_id, signature)` pairs.
    pub(crate) fn batch_update_signatures(
        &self,
        tx: &Transaction<'_>,
        signatures: &[(FilterId, String)],
    ) -> Result<()> {
        let mut statement = tx.prepare(
            r"
            UPDATE
                [rules_list]
            SET
                integrity_signature = :sig
            WHERE
                filter_id = :filter_id",
        )?;

        for (filter_id, sig) in signatures {
            statement.execute(named_params! {
                ":filter_id": filter_id,
                ":sig": sig,
            })?;
        }

        Ok(())
    }
}

impl BulkDeleteRepository<RulesListEntity, FilterId> for RulesListRepository {
    const PK_FIELD: &'static str = "filter_id";
}

impl Repository<RulesListEntity> for RulesListRepository {
    const TABLE_NAME: &'static str = "rules_list";

    fn insert(&self, conn: &Transaction<'_>, entities: &[RulesListEntity]) -> Result<(), Error> {
        let mut statement = conn.prepare(
            r"
                INSERT OR REPLACE INTO
                    [rules_list]
                    (
                        filter_id,
                        rules_text,
                        disabled_rules_text,
                        rules_count,
                        text_hash,
                        has_directives,
                        integrity_signature
                    )
                VALUES
                    (
                        :filter_id,
                        :rules_text,
                        :disabled_rules_text,
                        :rules_count,
                        :text_hash,
                        :has_directives,
                        :integrity_signature
                    )
            ",
        )?;

        for entity in entities.iter() {
            statement.execute(named_params! {
                ":filter_id": entity.filter_id,
                ":rules_text": entity.text,
                ":disabled_rules_text": entity.disabled_text,
                ":rules_count": entity.rules_count,
                ":text_hash": entity.text_hash,
                ":has_directives": entity.has_directives,
                ":integrity_signature": entity.integrity_signature
            })?;
        }

        Ok(())
    }
}
