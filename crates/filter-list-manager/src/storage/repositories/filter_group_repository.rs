use crate::manager::models::configuration::Locale;
use crate::storage::entities::filter_group_entity::FilterGroupEntity;
use crate::storage::repositories::Repository;
use rusqlite::{named_params, Connection, Result, Row, Transaction};
#[cfg(test)]
use std::collections::HashMap;

/// Repository for filter group
pub(crate) struct FilterGroupRepository;

#[cfg(test)]
pub(crate) type MapGroupIdOnFilterGroupsList = HashMap<i32, FilterGroupEntity>;

impl FilterGroupRepository {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    #[cfg(test)]
    pub(crate) fn select_mapped(&self, conn: &Connection) -> Result<MapGroupIdOnFilterGroupsList> {
        let mut statement = conn.prepare(
            r"
            SELECT
               group_id,
               name,
               display_number
            FROM
               [filter_group]
        ",
        )?;

        let rows = statement.query_map((), |row: &Row| -> Result<FilterGroupEntity> {
            Ok(FilterGroupEntity {
                group_id: row.get(0)?,
                name: row.get(1)?,
                display_number: row.get(2)?,
            })
        })?;

        let mut results = HashMap::new();
        for row in rows {
            let unwrapped = row?;
            results.insert(unwrapped.group_id, unwrapped);
        }

        Ok(results)
    }

    pub(crate) fn select_localised_with_block<Block, Out>(
        &self,
        locale: &Locale,
        connection: &Connection,
        block: Block,
    ) -> Result<Vec<Out>>
    where
        Block: Fn(FilterGroupEntity) -> Out,
    {
        let mut statement = connection.prepare(
            r"
            SELECT
                fg.group_id,
                COALESCE(fgl.name, fg.name) as name,
                fg.display_number
            FROM
                [filter_group] fg
            LEFT JOIN
                [filter_group_localisation] fgl
            ON
                fg.group_id=fgl.group_id
                AND
                fgl.lang=?1
        ",
        )?;

        let mut out: Vec<Out> = vec![];
        let mut rows = statement.query([locale])?;
        while let Some(row) = rows.next()? {
            out.push(block(FilterGroupRepository::hydrate(row)?));
        }

        Ok(out)
    }

    /// This deletes groups only from index
    pub(crate) fn delete_index_groups(&self, transaction: &Transaction) -> Result<()> {
        let mut statement = transaction
            .prepare(format!("DELETE FROM {} WHERE group_id > 0", Self::TABLE_NAME).as_str())?;

        statement.execute(())?;

        Ok(())
    }

    fn hydrate(row: &Row) -> Result<FilterGroupEntity> {
        Ok(FilterGroupEntity {
            group_id: row.get(0)?,
            name: row.get(1)?,
            display_number: row.get(2)?,
        })
    }
}

impl Repository<FilterGroupEntity> for FilterGroupRepository {
    const TABLE_NAME: &'static str = "[filter_group]";

    fn insert(
        &self,
        transaction: &Transaction,
        entities: &[FilterGroupEntity],
    ) -> Result<(), rusqlite::Error> {
        let mut statement = transaction.prepare(
            r"
            INSERT OR REPLACE INTO
                [filter_group]
                (
                    group_id,
                    name,
                    display_number
                )
            VALUES
                (
                    :group_id,
                    :name,
                    :display_number
                )
            ",
        )?;

        for entity in entities.iter() {
            statement.execute(named_params! {
                ":group_id": entity.group_id,
                ":name": entity.name,
                ":display_number": entity.display_number
            })?;
        }

        Ok(())
    }

    fn clear(&self, _: &Transaction) -> Result<()> {
        // Cannot execute this, because
        Err(rusqlite::Error::InvalidQuery)
    }
}
