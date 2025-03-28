use crate::storage::entities::diff_update_entity::DiffUpdateEntity;
use crate::storage::entities::hydrate::Hydrate;
use crate::storage::repositories::Repository;
use crate::storage::utils::build_in_clause;
use crate::FilterId;
use rusqlite::{named_params, params_from_iter, Connection, Error, Transaction};
use std::collections::HashMap;

pub(crate) type DiffUpdatesMap = HashMap<FilterId, DiffUpdateEntity>;

/// Repository for `diff_updates` table. Diff-Path information stored here
pub(crate) struct DiffUpdateRepository;

impl DiffUpdateRepository {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    /// Selects entities mapped by [`FilterId`] for provided `for_ids`
    pub(crate) fn select_map(
        &self,
        conn: &Connection,
        for_ids: &[FilterId],
    ) -> rusqlite::Result<DiffUpdatesMap> {
        let mut sql = String::from(
            r"
            SELECT
                filter_id,
                next_path,
                next_check_time
            FROM
                [diff_updates]
            WHERE ",
        );

        sql += build_in_clause("filter_id", for_ids.len()).as_str();

        let mut statement = conn.prepare(sql.as_str())?;

        let mut rows = statement.query(params_from_iter(for_ids))?;

        let mut out = HashMap::new();
        while let Some(row) = rows.next()? {
            let filter_id: FilterId = row.get(0)?;

            out.insert(filter_id, DiffUpdateEntity::hydrate(row)?);
        }

        // Cuz will use .keys() after, which have O(capacity), not O(len)
        out.shrink_to_fit();

        Ok(out)
    }
}

impl Repository<DiffUpdateEntity> for DiffUpdateRepository {
    const TABLE_NAME: &'static str = "[diff_updates]";

    fn insert(&self, conn: &Transaction<'_>, entities: &[DiffUpdateEntity]) -> Result<(), Error> {
        let mut statements = conn.prepare(
            r"
            INSERT OR REPLACE INTO
                [diff_updates]
                (
                    filter_id,
                    next_path,
                    next_check_time
                ) VALUES (
                    :filter_id,
                    :next_path,
                    :next_check_time
                )
        ",
        )?;

        for entity in entities.iter() {
            statements.execute(named_params! {
                ":filter_id": entity.filter_id,
                ":next_path": entity.next_path,
                ":next_check_time": entity.next_check_time
            })?;
        }

        Ok(())
    }
}
