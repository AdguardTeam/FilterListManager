use crate::storage::entities::filter::filter_include_entity::FilterIncludeEntity;
use crate::storage::repositories::Repository;
use crate::storage::sql_generators::operator::SQLOperator;
use crate::storage::utils::{build_in_clause, process_where_clause};
use crate::storage::Hydrate;
use crate::FilterId;
use rusqlite::{named_params, params_from_iter, Connection, Error, Transaction};
use std::collections::HashMap;

pub(crate) type MapFilterIdOnFilterIncludes = HashMap<FilterId, Vec<FilterIncludeEntity>>;

/// Basic SQL-query with all fields
const BASIC_SELECT_SQL: &str = r"
    SELECT
        row_id,
        filter_id,
        absolute_url,
        body,
        rules_count
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
                        rules_count
                    )
                VALUES
                    (
                        :row_id,
                        :filter_id,
                        :absolute_url,
                        :body,
                        :rules_count
                    )
                ",
        )?;

        for entity in entities.iter() {
            statement.execute(named_params! {
                ":row_id": entity.row_id,
                ":filter_id": entity.filter_id,
                ":absolute_url": entity.absolute_url,
                ":body": entity.body,
                ":rules_count": entity.rules_count
            })?;
        }

        Ok(())
    }
}
