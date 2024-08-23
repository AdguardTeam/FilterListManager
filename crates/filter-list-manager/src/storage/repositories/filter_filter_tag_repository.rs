use crate::storage::entities::filter_filter_tag_entity::FilterFilterTagEntity;
use crate::storage::repositories::{BulkDeleteRepository, Repository};
use crate::FilterId;
use rusqlite::{named_params, Transaction};

#[cfg(test)]
use rusqlite::{Connection, Row};

/// Repository for (filter <- tag) relations filter_id <- tag_id
pub(crate) struct FilterFilterTagRepository;

#[cfg(test)]
use crate::storage::sql_generators::operator::SQLOperator;

/// Basic SQL-query with all fields
#[cfg(test)]
const BASIC_SELECT_SQL: &str = r"
    SELECT
        tag_id,
        filter_id
    FROM
        [filter_filter_tag]
";

impl FilterFilterTagRepository {
    pub(crate) const fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
impl FilterFilterTagRepository {
    pub(crate) fn select(
        &self,
        conn: &Connection,
        where_clause: Option<SQLOperator>,
    ) -> rusqlite::Result<Vec<FilterFilterTagEntity>> {
        use crate::storage::utils::process_where_clause;

        let (sql, params) = process_where_clause(String::from(BASIC_SELECT_SQL), where_clause)?;

        let mut statement = conn.prepare(sql.as_str())?;

        let rows = statement.query_map(params, FilterFilterTagRepository::hydrate)?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }

        Ok(results)
    }

    fn hydrate(row: &Row) -> rusqlite::Result<FilterFilterTagEntity> {
        Ok(FilterFilterTagEntity {
            filter_id: row.get(0)?,
            tag_id: row.get(1)?,
        })
    }
}

impl Repository<FilterFilterTagEntity> for FilterFilterTagRepository {
    const TABLE_NAME: &'static str = "[filter_filter_tag]";

    fn insert(
        &self,
        transaction: &Transaction,
        entities: Vec<FilterFilterTagEntity>,
    ) -> Result<(), rusqlite::Error> {
        let mut statement = transaction.prepare(
            r"
            INSERT OR REPLACE INTO
                [filter_filter_tag]
                (
                    filter_id,
                    tag_id
                )
            VALUES
                (
                    :filter_id,
                    :tag_id
                )
            ",
        )?;

        for entity in entities.iter() {
            statement.execute(named_params! {
                ":filter_id": entity.filter_id,
                ":tag_id": entity.tag_id,
            })?;
        }

        Ok(())
    }
}

impl BulkDeleteRepository<FilterFilterTagEntity, FilterId> for FilterFilterTagRepository {
    const PK_FIELD: &'static str = "filter_id";
}
