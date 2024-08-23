use crate::manager::models::FilterId;
use crate::storage::entities::rules_list_entity::RulesListEntity;
use crate::storage::repositories::{BulkDeleteRepository, Repository};
use crate::storage::sql_generators::operator::SQLOperator;
use crate::storage::utils::{build_in_clause, process_where_clause};
use rusqlite::Result;
use rusqlite::{
    named_params, params_from_iter, Connection, Error, OptionalExtension, Row, Transaction,
};
use std::collections::HashMap;

pub(crate) type MapFilterIdOnRulesList = HashMap<FilterId, RulesListEntity>;

pub(crate) type MapFilterIdOnRulesString = HashMap<FilterId, String>;

/// Basic SQL-query with all fields
const BASIC_SELECT_SQL: &str = r"
    SELECT
        filter_id,
        rules_text,
        disabled_rules_text
    FROM
        [rules_list]
";

pub(crate) struct RulesListRepository;

impl RulesListRepository {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    pub(crate) fn insert_row(&self, conn: &mut Connection, entity: RulesListEntity) -> Result<()> {
        let transaction = conn.transaction()?;

        self.insert(&transaction, vec![entity])
            .and_then(|_| transaction.commit())
    }

    /// Gets entities mapped by [`FilterId`]
    pub(crate) fn select_mapped(
        &self,
        conn: &Connection,
        where_clause: Option<SQLOperator>,
    ) -> Result<MapFilterIdOnRulesList> {
        let (sql, params) = process_where_clause(String::from(BASIC_SELECT_SQL), where_clause)?;
        let mut statement = conn.prepare(sql.as_str())?;

        let rows = statement.query_map(params, RulesListRepository::hydrate)?;

        let mut results = HashMap::new();
        for row in rows {
            let unwrapped = row?;
            results.insert(unwrapped.filter_id, unwrapped);
        }

        Ok(results)
    }

    /// Gets rules strings mapped by [`FilterId`] for provided `for_ids`
    pub(crate) fn select_rules_string_map(
        &self,
        conn: &Connection,
        for_ids: &[FilterId],
    ) -> Result<MapFilterIdOnRulesString> {
        let mut sql = String::from(
            r"
            SELECT
                filter_id,
                rules_text
            FROM
                [rules_list]
            WHERE
                filter_id
        ",
        );

        sql += build_in_clause(for_ids.len()).as_str();

        let mut statement = conn.prepare(sql.as_str())?;

        let mut rows = statement.query(params_from_iter(for_ids))?;

        let mut results = HashMap::new();
        while let Some(row) = rows.next()? {
            results.insert(row.get(0)?, row.get(1)?);
        }

        Ok(results)
    }

    pub(crate) fn select(
        &self,
        connection: &Connection,
        where_clause: Option<SQLOperator>,
    ) -> Result<Option<Vec<RulesListEntity>>> {
        let (sql, params) = process_where_clause(String::from(BASIC_SELECT_SQL), where_clause)?;
        let mut statement = connection.prepare(sql.as_str())?;

        let option = statement
            .query_map(params, RulesListRepository::hydrate)
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

    pub(crate) fn hydrate(row: &Row) -> Result<RulesListEntity> {
        Ok(RulesListEntity {
            filter_id: row.get(0)?,
            text: row.get(1)?,
            disabled_text: row.get(2)?,
        })
    }
}

impl BulkDeleteRepository<RulesListEntity, FilterId> for RulesListRepository {
    const PK_FIELD: &'static str = "filter_id";
}

impl Repository<RulesListEntity> for RulesListRepository {
    const TABLE_NAME: &'static str = "[rules_list]";

    fn insert(&self, conn: &Transaction<'_>, entities: Vec<RulesListEntity>) -> Result<(), Error> {
        let mut statement = conn.prepare(
            r"
                INSERT OR REPLACE INTO
                    [rules_list]
                    (
                        filter_id,
                        rules_text,
                        disabled_rules_text
                    )
                VALUES
                    (
                        :filter_id,
                        :rules_text,
                        :disabled_rules_text
                    )
            ",
        )?;

        for entity in entities.iter() {
            statement.execute(named_params! {
                ":filter_id": entity.filter_id,
                ":rules_text": entity.text,
                ":disabled_rules_text": entity.disabled_text
            })?;
        }

        Ok(())
    }
}
