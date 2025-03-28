use crate::manager::models::FilterId;
use crate::storage::blob::BlobHandleImpl;
use crate::storage::entities::hydrate::Hydrate;
use crate::storage::entities::rules_list::disabled_rules_entity::DisabledRulesEntity;
use crate::storage::entities::rules_list::rules_count_entity::RulesCountEntity;
use crate::storage::entities::rules_list::rules_list_entity::RulesListEntity;
use crate::storage::repositories::{BulkDeleteRepository, Repository};
use crate::storage::sql_generators::operator::SQLOperator;
use crate::storage::utils::{build_in_clause, process_where_clause};
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
        rules_count
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

    /// Gets rules strings and disabled_rules strings mapped by [`FilterId`] for provided `for_ids`
    pub(crate) fn select_rules_maps(
        &self,
        conn: &Connection,
        for_ids: &[FilterId],
    ) -> Result<(MapFilterIdOnRulesString, MapFilterIdOnRulesString)> {
        let mut sql = String::from(
            r"
            SELECT
                filter_id,
                rules_text,
                disabled_rules_text
            FROM
                [rules_list]
            WHERE ",
        );

        sql += build_in_clause("filter_id", for_ids.len()).as_str();

        let mut statement = conn.prepare(sql.as_str())?;

        let mut rows = statement.query(params_from_iter(for_ids))?;

        let mut rules = HashMap::new();
        let mut disabled_rules = HashMap::new();
        while let Some(row) = rows.next()? {
            let id: FilterId = row.get(0)?;

            rules.insert(id, row.get(1)?);
            disabled_rules.insert(id, row.get(2)?);
        }

        Ok((rules, disabled_rules))
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
                        rules_count
                    )
                VALUES
                    (
                        :filter_id,
                        :rules_text,
                        :disabled_rules_text,
                        :rules_count
                    )
            ",
        )?;

        for entity in entities.iter() {
            statement.execute(named_params! {
                ":filter_id": entity.filter_id,
                ":rules_text": entity.text,
                ":disabled_rules_text": entity.disabled_text,
                ":rules_count": entity.rules_count
            })?;
        }

        Ok(())
    }
}
