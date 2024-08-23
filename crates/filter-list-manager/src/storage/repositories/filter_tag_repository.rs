use crate::manager::models::FilterId;
use crate::storage::entities::filter_tag_entity::FilterTagEntity;
use crate::storage::repositories::Repository;
use rusqlite::{named_params, Connection, Error, Result, Row, Transaction};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

/// Repository for tags
pub(crate) struct FilterTagRepository;

type FilterTagWithFilterId = (FilterTagEntity, FilterId);
pub(crate) type MapFilterIdOnFilterTagList = HashMap<FilterId, Vec<FilterTagEntity>>;

impl FilterTagRepository {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    pub(crate) fn select_with_filter_tag(
        &self,
        conn: &Connection,
    ) -> Result<MapFilterIdOnFilterTagList> {
        let mut statement = conn.prepare(
            r"
            SELECT
                ft.tag_id,
                ft.keyword,
                fft.filter_id
            FROM
                [filter_tag] ft
            JOIN
                [filter_filter_tag] fft
            USING(tag_id)
        ",
        )?;

        let rows = statement.query_map((), |row: &Row| -> Result<FilterTagWithFilterId> {
            Ok((
                FilterTagEntity {
                    tag_id: row.get(0)?,
                    keyword: row.get(1)?,
                },
                row.get(2)?,
            ))
        })?;

        let mut results = HashMap::new();
        for row in rows {
            let unwrapped = row?;
            let key = unwrapped.1;

            match results.entry(key) {
                Entry::Occupied(vec) => vec.into_mut(),
                Entry::Vacant(map) => map.insert(vec![]),
            }
            .push(unwrapped.0);
        }

        Ok(results)
    }

    pub(crate) fn select_with_block<Block, Out>(
        &self,
        connection: &Connection,
        block: Block,
    ) -> Result<Vec<Out>>
    where
        Block: Fn(FilterTagEntity) -> Out,
    {
        let mut statement = connection.prepare(
            r"
            SELECT
                tag_id,
                keyword
            FROM
                [filter_tag]
        ",
        )?;

        let mut out: Vec<Out> = vec![];

        let mut rows = statement.query(())?;
        while let Some(row) = rows.next()? {
            out.push(block(FilterTagRepository::hydrate(row)?));
        }

        Ok(out)
    }

    #[inline]
    fn hydrate(row: &Row) -> Result<FilterTagEntity> {
        Ok(FilterTagEntity {
            tag_id: row.get(0)?,
            keyword: row.get(1)?,
        })
    }
}

impl Repository<FilterTagEntity> for FilterTagRepository {
    const TABLE_NAME: &'static str = "[filter_tag]";

    fn insert(&self, conn: &Transaction, entities: Vec<FilterTagEntity>) -> Result<(), Error> {
        let mut statement = conn.prepare(
            r"
            INSERT OR REPLACE INTO
                [filter_tag]
                (
                    tag_id,
                    keyword
                )
                VALUES
                (
                    :tag_id,
                    :keyword
                )
            ",
        )?;

        for entity in entities.iter() {
            statement.execute(named_params! {
                ":tag_id": entity.tag_id,
                ":keyword": entity.keyword
            })?;
        }

        Ok(())
    }
}
