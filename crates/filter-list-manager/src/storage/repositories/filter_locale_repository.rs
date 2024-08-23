use crate::manager::models::FilterId;
use crate::storage::entities::filter_locale_entity::FilterLocaleEntity;
use crate::storage::repositories::Repository;
use rusqlite::Result;
use rusqlite::{named_params, Connection, Error, Row, Transaction};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub(crate) type MapFilterIdOnFilterLocaleEntry = HashMap<FilterId, Vec<FilterLocaleEntity>>;

/// Repository for relations (filter <- lang). Lang is marker for language-specific filters
pub(crate) struct FilterLocaleRepository;

impl FilterLocaleRepository {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    pub(crate) fn select_mapped(
        &self,
        conn: &Connection,
    ) -> Result<MapFilterIdOnFilterLocaleEntry> {
        let mut statement = conn.prepare(
            r"
            SELECT
                filter_id,
                lang
            FROM
                [filter_locale]
        ",
        )?;

        let rows = statement.query_map((), FilterLocaleRepository::hydrate)?;

        let mut results = HashMap::new();
        for row in rows {
            let unwrapped = row?;
            let key = unwrapped.filter_id;

            match results.entry(key) {
                Entry::Occupied(vec) => vec.into_mut(),
                Entry::Vacant(map) => map.insert(vec![]),
            }
            .push(unwrapped);
        }

        Ok(results)
    }

    /// Returns filled entity from row
    fn hydrate(row: &Row) -> Result<FilterLocaleEntity> {
        Ok(FilterLocaleEntity {
            filter_id: row.get(0)?,
            lang: row.get(1)?,
        })
    }
}

impl Repository<FilterLocaleEntity> for FilterLocaleRepository {
    const TABLE_NAME: &'static str = "[filter_locale]";

    fn insert(
        &self,
        transaction: &Transaction<'_>,
        entities: Vec<FilterLocaleEntity>,
    ) -> Result<(), Error> {
        let mut statement = transaction.prepare(
            r"
            INSERT OR REPLACE INTO
                [filter_locale]
                (
                    filter_id,
                    lang
                )
            VALUES
                (
                    :filter_id,
                    :lang
                )
            ",
        )?;

        for entity in entities.iter() {
            statement.execute(named_params! {
                ":filter_id": entity.filter_id,
                ":lang": entity.lang,
            })?;
        }

        Ok(())
    }
}
