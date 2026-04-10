use crate::manager::models::FilterId;
use crate::storage::db_bootstrap::get_bootstrapped_filter_id;
use crate::storage::entities::db_metadata_entity::DBMetadataEntity;
use crate::storage::entities::filter::filter_entity::FilterEntity;
use crate::storage::entities::filter::filter_inner_flag_entity::FilterInnerFlagEntity;
use crate::storage::entities::hydrate::Hydrate;
use crate::storage::repositories::db_metadata_repository::DBMetadataRepository;
use crate::storage::repositories::{BulkDeleteRepository, Repository};
use crate::storage::sql_generators::operator::SQLOperator;
use crate::storage::utils::{build_in_clause, process_where_clause};
use crate::utils::integrity::{sign_filter_metadata, verify_filter_entity};
use crate::utils::memory::heap;
use crate::{MAXIMUM_CUSTOM_FILTER_ID, MINIMUM_CUSTOM_FILTER_ID};
use rusqlite::types::Type;
use rusqlite::{
    named_params, params_from_iter, Connection, Error, OptionalExtension, Row, Rows, Transaction,
};
use std::collections::HashMap;

/// Basic SQL-query with all fields
const BASIC_SELECT_SQL: &str = r"
    SELECT
        f.filter_id,
        f.group_id,
        f.version,
        f.last_update_time,
        f.last_download_time,
        f.display_number,
        f.title,
        f.description,
        f.homepage,
        f.license,
        f.checksum,
        f.expires,
        f.download_url,
        f.subscription_url,
        f.is_enabled,
        f.is_installed,
        f.is_trusted,
        f.is_user_title,
        f.is_user_description,
        f.integrity_signature
    FROM
        [filter] f
";

/// Basic SQL-query for filters counting
const BASIC_COUNT_SQL: &str = r"
    SELECT
        COUNT(filter_id) as existed
    FROM
        [filter]
";

/// Database name as a constant
pub(crate) const FILTER_TABLE_NAME: &str = "filter";

/// Repository for filter table
pub(crate) struct FilterRepository;

/// Cooked SQL Operators and variable selects
impl FilterRepository {
    /// Factory for [`SQLOperator`] which represents group_ids for custom filter
    #[inline]
    pub(crate) fn custom_filter_operator<'a>() -> SQLOperator<'a> {
        SQLOperator::FieldLTValue("group_id", 1.into())
    }

    /// Factory for [`SQLOperator`] which represents exception, expect filters,
    /// which were added while bootstrapping
    #[inline]
    pub(crate) fn except_bootstrapped_filter_ids_operator<'a>() -> SQLOperator<'a> {
        SQLOperator::Not(heap(SQLOperator::FieldEqualValue(
            "filter_id",
            get_bootstrapped_filter_id().into(),
        )))
    }

    /// Constructs condition (IS_CUSTOM_FILTER() AND `rhs`)
    #[inline]
    pub(crate) fn custom_filter_with_extra(rhs: SQLOperator<'_>) -> SQLOperator<'_> {
        SQLOperator::And(heap(Self::custom_filter_operator()), heap(rhs))
    }

    /// Constructs condition for custom filter with specified `filter_id`
    #[inline]
    pub(crate) fn custom_filter_with_id<'a>(filter_id: FilterId) -> SQLOperator<'a> {
        Self::custom_filter_with_extra(SQLOperator::FieldEqualValue("filter_id", filter_id.into()))
    }

    /// Selects all filters except bootstrapped
    pub(crate) fn select_filters_except_bootstrapped(
        &self,
        conn: &Connection,
    ) -> rusqlite::Result<Option<Vec<FilterEntity>>> {
        self.select(
            conn,
            Some(FilterRepository::except_bootstrapped_filter_ids_operator()),
        )
    }
}

/// Misc methods
impl FilterRepository {
    pub(crate) const fn new() -> Self {
        Self {}
    }
}

/// Queries
impl FilterRepository {
    /// Counts filters by condition
    pub(crate) fn count(
        &self,
        conn: &Connection,
        where_clause: Option<SQLOperator>,
    ) -> rusqlite::Result<i32> {
        let mut sql = String::from(BASIC_COUNT_SQL);
        let params = process_where_clause(&mut sql, where_clause)?;
        let mut statement = conn.prepare(sql.as_str())?;

        let count_result = statement.query_row(params, |row| row.get(0)).optional()?;

        Ok(count_result.unwrap_or_default())
    }

    /// This method must be used only for insert, not upsert
    pub(crate) fn only_insert_row(
        &self,
        transaction: &Transaction,
        entity: FilterEntity,
    ) -> Result<FilterEntity, Error> {
        let last_insert_id = self.insert_internal(transaction, &[entity], HashMap::new(), None)?;

        let mut sql = String::from(BASIC_SELECT_SQL);
        sql += "WHERE f.filter_id=?";

        transaction.query_row(sql.as_str(), [last_insert_id], FilterEntity::hydrate)
    }

    pub(crate) fn toggle_filter_lists(
        &self,
        tx: &Transaction,
        ids: &[FilterId],
        next_value: bool,
    ) -> Result<usize, Error> {
        if ids.is_empty() {
            return Ok(0);
        }

        let mut sql = String::from(
            r"
            UPDATE
                [filter]
            SET
                is_enabled=?1
            WHERE ",
        );

        sql += build_in_clause("filter_id", ids.len()).as_str();

        let mut statement = tx.prepare(sql.as_str())?;

        // Insert bool value at the first param position
        let first_param = [next_value as FilterId];

        let rows_updated = statement.execute(params_from_iter(first_param.iter().chain(ids)))?;

        Ok(rows_updated)
    }

    pub(crate) fn toggle_is_installed(
        &self,
        tx: &Transaction,
        ids: &[FilterId],
        next_value: bool,
    ) -> Result<usize, Error> {
        if ids.is_empty() {
            return Ok(0);
        }

        let mut sql = String::from(
            r"
            UPDATE
                [filter]
            SET
                is_installed=?1
            WHERE ",
        );

        sql += build_in_clause("filter_id", ids.len()).as_str();

        let mut statement = tx.prepare(sql.as_str())?;

        // Insert bool value at the first param position
        let first_param = [next_value as FilterId];

        let rows_updated = statement.execute(params_from_iter(first_param.iter().chain(ids)))?;

        Ok(rows_updated)
    }

    /// Filters passed `ids` and returns only ids for custom filters
    /// Note: Service filters must not be included
    pub(crate) fn filter_custom_filters(
        &self,
        conn: &Connection,
        ids: &[FilterId],
    ) -> rusqlite::Result<Vec<FilterId>> {
        let mut sql = String::from(
            r"
            SELECT
                group_id < 1 as is_custom,
                filter_id
            FROM
                [filter]
            WHERE ",
        );

        sql += build_in_clause("filter_id", ids.len()).as_str();

        // Check that filter is custom, by the group presence
        let mut statement = conn.prepare(sql.as_str())?;

        let mut rows = statement.query(params_from_iter(ids))?;

        let mut out: Vec<FilterId> = vec![];
        while let Some(row) = rows.next()? {
            #[allow(clippy::bool_comparison)]
            if row.get::<usize, bool>(0)? == true {
                out.push(row.get::<usize, FilterId>(1)?);
            }
        }

        Ok(out)
    }

    /// Check that db has at least one filters record
    pub(crate) fn has_at_least_one_record(&self, conn: &Connection) -> rusqlite::Result<bool> {
        let mut statement = conn.prepare(
            r"
            SELECT
                filter_id
            FROM
                [filter]
            LIMIT 1
        ",
        )?;

        let result = statement
            .query_row((), |row: &Row| -> rusqlite::Result<i32> { row.get(0) })
            .optional()?;

        Ok(result.is_some())
    }

    /// General select method for filters
    ///
    /// Returns [`None`] if selection list is empty
    pub(crate) fn select(
        &self,
        conn: &Connection,
        where_clause: Option<SQLOperator>,
    ) -> Result<Option<Vec<FilterEntity>>, Error> {
        let mut sql = String::from(BASIC_SELECT_SQL);
        let params = process_where_clause(&mut sql, where_clause)?;

        let mut statement = conn.prepare(sql.as_str())?;

        let Some(rows) = statement
            .query_map(params, FilterEntity::hydrate)
            .optional()?
        else {
            return Ok(None);
        };

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }

        if results.is_empty() {
            return Ok(None);
        }

        Ok(Some(results))
    }

    /// Selects filters mapped by [`FilterId`] by `clause`.
    /// *Will fail if result set is empty*
    pub(crate) fn select_mapped(
        &self,
        conn: &Connection,
        where_clause: Option<SQLOperator>,
    ) -> rusqlite::Result<HashMap<FilterId, FilterEntity>> {
        let mut sql = String::from(BASIC_SELECT_SQL);
        let params = process_where_clause(&mut sql, where_clause)?;

        let mut statement = conn.prepare(sql.as_str())?;
        let mut map = HashMap::new();

        let rows = statement.query_map(params, FilterEntity::hydrate)?;

        for row in rows {
            let tmp = row?;

            let filter_id = match tmp.filter_id {
                None => {
                    return Err(Error::InvalidColumnType(
                        0,
                        String::from("filter_id"),
                        Type::Integer,
                    ));
                }
                Some(ref filter) => filter.to_owned(),
            };

            map.insert(filter_id, tmp);
        }

        Ok(map)
    }

    /// Gets download urls mapped by [`FilterId`]
    ///
    /// * `conn` - Connection
    /// * `ids` - iterator of proposed ['FilterId']
    /// * `len` - iterator length
    pub(crate) fn select_download_urls<'i>(
        &self,
        conn: &Connection,
        ids: impl Iterator<Item = &'i FilterId>,
        len: usize,
    ) -> rusqlite::Result<HashMap<FilterId, String>> {
        if len == 0 {
            return Ok(HashMap::new());
        }

        let mut sql = String::from(
            r"
            SELECT
                filter_id,
                download_url
            FROM
                [filter]
            WHERE ",
        );

        sql += build_in_clause("filter_id", len).as_str();

        let mut statement = conn.prepare(sql.as_str())?;

        let mut rows = statement.query(params_from_iter(ids))?;

        let mut out = HashMap::new();
        while let Some(row) = rows.next()? {
            out.insert(row.get(0)?, row.get(1)?);
        }

        Ok(out)
    }

    /// Update user defined metadata for custom_filter
    ///
    /// * `transaction` - Outer transaction
    /// * `filter_id` - ID
    /// * `title` - New title
    /// * `is_trusted` - Is this filter trusted
    /// * `is_title_set_by_user` - This title strictly set by user (true); or can be changed during update (false)
    pub(crate) fn update_user_metadata_for_custom_filter(
        &self,
        transaction: &Transaction,
        filter_id: FilterId,
        title: &str,
        is_trusted: bool,
        is_user_title: bool,
    ) -> rusqlite::Result<bool> {
        let mut statement = transaction.prepare(
            r"
            UPDATE
                [filter]
            SET
                title=:title,
                is_trusted=:is_trusted,
                is_user_title=:is_user_title
            WHERE
                filter_id=:filter_id
        ",
        )?;

        let usize = statement.execute(named_params! {
            ":title": title,
            ":is_trusted": is_trusted,
            ":filter_id": filter_id,
            ":is_user_title": is_user_title
        })?;

        Ok(usize > 0)
    }

    /// Selects inner flags for filters
    fn select_filter_inner_flag(
        &self,
        transaction: &Transaction<'_>,
        entities: &[FilterEntity],
    ) -> rusqlite::Result<HashMap<FilterId, FilterInnerFlagEntity>> {
        let mut sql = String::from(
            r"
            SELECT
                filter_id,
                is_user_title,
                is_user_description
            FROM
                [filter]
            WHERE ",
        );

        let ids: Vec<FilterId> = entities
            .iter()
            .filter_map(|entity| entity.filter_id)
            .collect();

        sql += build_in_clause("filter_id", ids.len()).as_str();

        let mut statement = transaction.prepare(sql.as_str())?;

        let mut rows = statement.query(params_from_iter(ids))?;

        let mut out: HashMap<FilterId, FilterInnerFlagEntity> = HashMap::new();
        while let Some(row) = rows.next()? {
            let entity: FilterInnerFlagEntity = FilterInnerFlagEntity {
                filter_id: row.get(0)?,
                is_user_title: row.get(1)?,
                is_user_description: row.get(2)?,
            };

            out.insert(entity.filter_id, entity);
        }

        Ok(out)
    }

    /// Inserts entities with callback, which will be called once for each entity right after this [`FilterId`] selection, but before insert operation itself.
    pub(crate) fn insert_with_chosen_filters_callback<'c, C>(
        &self,
        transaction: &Transaction<'_>,
        entities: &[FilterEntity],
        callback: C,
    ) -> rusqlite::Result<i64>
    where
        C: FnMut(&FilterEntity, Option<FilterId>) + 'c,
    {
        let flags = self.select_filter_inner_flag(transaction, entities)?;
        self.insert_internal(transaction, entities, flags, Some(Box::new(callback)))
    }

    #[allow(clippy::manual_range_contains)]
    fn insert_internal<'c>(
        &self,
        transaction: &Transaction<'_>,
        entities: &[FilterEntity],
        filter_inner_flag_entities: HashMap<FilterId, FilterInnerFlagEntity>,
        mut choose_filter_id_hook: Option<Box<dyn FnMut(&FilterEntity, Option<FilterId>) + 'c>>,
    ) -> rusqlite::Result<i64> {
        let mut statement = transaction.prepare(
            r"
            INSERT OR REPLACE INTO
                [filter]
                (
                    filter_id,
                    group_id,
                    version,
                    last_update_time,
                    last_download_time,
                    display_number,
                    title,
                    description,
                    homepage,
                    license,
                    checksum,
                    expires,
                    download_url,
                    subscription_url,
                    is_enabled,
                    is_installed,
                    is_trusted,
                    is_user_title,
                    is_user_description,
                    integrity_signature
                ) VALUES (
                    :filter_id,
                    :group_id,
                    :version,
                    COALESCE(:last_update_time, CURRENT_TIMESTAMP),
                    COALESCE(:last_download_time, CURRENT_TIMESTAMP),
                    :display_number,
                    :title,
                    :description,
                    :homepage,
                    :license,
                    :checksum,
                    :expires,
                    :download_url,
                    :subscription_url,
                    :is_enabled,
                    :is_installed,
                    :is_trusted,
                    :is_user_title,
                    :is_user_description,
                    :integrity_signature
                )",
        )?;

        let mut metadata_entity: Option<DBMetadataEntity> = None;
        for entity in entities.iter() {
            // Should take special autoincrement id for new custom filters
            let filter_id = if entity.is_custom_filter_has_invalid_or_empty_id() {
                if metadata_entity.is_none() {
                    metadata_entity = DBMetadataRepository::read(transaction)?;
                }

                if let Some(ref mut metadata_ref) = metadata_entity {
                    let tmp_counter: FilterId = metadata_ref.custom_filters_autoincrement_value - 1;

                    // Check negative autoincrement
                    if tmp_counter > MAXIMUM_CUSTOM_FILTER_ID
                        || tmp_counter < MINIMUM_CUSTOM_FILTER_ID
                    {
                        return Err(Error::InvalidParameterName(
                            "custom_filter_increment".to_string(),
                        ));
                    }

                    metadata_ref.custom_filters_autoincrement_value = tmp_counter;

                    Some(metadata_ref.custom_filters_autoincrement_value)
                } else {
                    return Err(Error::QueryReturnedNoRows);
                }
            } else {
                entity.filter_id
            };

            if let Some(ref mut on_choose_id) = choose_filter_id_hook {
                on_choose_id(entity, filter_id);
            }

            let is_user_title = entity.is_user_title.or_else(|| {
                filter_id
                    .as_ref()
                    .and_then(|id| filter_inner_flag_entities.get(id).map(|e| e.is_user_title))
                    .flatten()
            });

            let is_user_description = entity.is_user_description.or_else(|| {
                filter_id
                    .as_ref()
                    .and_then(|id| {
                        filter_inner_flag_entities
                            .get(id)
                            .map(|e| e.is_user_description)
                    })
                    .flatten()
            });

            statement.execute(named_params! {
                ":filter_id": filter_id,
                ":group_id": entity.group_id,
                ":version": entity.version,
                ":last_update_time": entity.last_update_time,
                ":last_download_time": entity.last_download_time,
                ":display_number": entity.display_number,
                ":title": entity.title,
                ":description": entity.description,
                ":homepage": entity.homepage,
                ":license": entity.license,
                ":checksum": entity.checksum,
                ":expires": entity.expires,
                ":download_url": entity.download_url,
                ":subscription_url": entity.subscription_url,
                ":is_enabled": entity.is_enabled,
                ":is_installed": entity.is_installed,
                ":is_trusted": entity.is_trusted,
                ":is_user_title": is_user_title,
                ":is_user_description": is_user_description,
                ":integrity_signature": entity.integrity_signature,
            })?;
        }

        let last_insert_id = transaction.last_insert_rowid();

        if let Some(value) = metadata_entity {
            DBMetadataRepository::save(transaction, &value)?;
        }

        Ok(last_insert_id)
    }
}

impl Repository<FilterEntity> for FilterRepository {
    const TABLE_NAME: &'static str = "[filter]";

    fn insert(
        &self,
        transaction: &Transaction<'_>,
        entities: &[FilterEntity],
    ) -> Result<(), Error> {
        let filter_inner_flag_entities = self.select_filter_inner_flag(transaction, entities)?;
        self.insert_internal(transaction, entities, filter_inner_flag_entities, None)
            .map(|_| ())
    }

    /// Do not clear filters repository
    fn clear(&self, _: &Transaction) -> rusqlite::Result<()> {
        Err(Error::InvalidQuery)
    }
}

impl BulkDeleteRepository<FilterEntity, FilterId> for FilterRepository {
    const PK_FIELD: &'static str = "filter_id";
}

/// Integrity-related methods for [`FilterRepository`]
impl FilterRepository {
    /// Updates `integrity_signature` for a single filter row by `filter_id`.
    pub(crate) fn update_integrity_signature(
        &self,
        tx: &Transaction<'_>,
        filter_id: FilterId,
        signature: &str,
    ) -> rusqlite::Result<()> {
        tx.execute(
            r"
            UPDATE
                [filter]
            SET
                integrity_signature = ?1
            WHERE
                filter_id = ?2",
            rusqlite::params![signature, filter_id],
        )
        .map(|_| ())
    }

    /// Iterates over all filter rows, computes integrity signatures for their
    /// critical metadata fields using the derived key, and returns
    /// `(filter_id, signature)` pairs without loading everything into memory.
    pub(crate) fn sign_and_collect_metadata_signatures_streaming(
        &self,
        conn: &Connection,
        derived_key: &[u8; 32],
    ) -> rusqlite::Result<Vec<(FilterId, String)>> {
        let mut statement = conn.prepare(
            r"
            SELECT
                filter_id,
                download_url,
                subscription_url,
                is_trusted,
                is_enabled,
                is_installed,
                version,
                last_update_time,
                last_download_time,
                expires
            FROM
                [filter]",
        )?;

        let mut rows = statement.query([])?;
        Self::collect_metadata_signatures_from_rows(&mut rows, derived_key)
    }

    /// Reads metadata fields from each row of `rows` and computes integrity
    /// signatures, returning `(filter_id, signature)` pairs.
    ///
    /// Expected column order: filter_id(0), download_url(1), subscription_url(2),
    /// is_trusted(3), is_enabled(4), is_installed(5), version(6),
    /// last_update_time(7), last_download_time(8), expires(9).
    fn collect_metadata_signatures_from_rows(
        rows: &mut Rows<'_>,
        derived_key: &[u8; 32],
    ) -> rusqlite::Result<Vec<(FilterId, String)>> {
        let mut signatures = Vec::new();

        while let Some(row) = rows.next()? {
            let filter_id: FilterId = row.get(0)?;
            let download_url: String = row.get(1)?;
            let subscription_url: String = row.get(2)?;
            let is_trusted: bool = row.get(3)?;
            let is_enabled: bool = row.get(4)?;
            let is_installed: bool = row.get(5)?;
            let version: String = row.get(6)?;
            let last_update_time: i64 = row.get(7)?;
            let last_download_time: i64 = row.get(8)?;
            let expires: i32 = row.get(9)?;

            let sig = sign_filter_metadata(
                derived_key,
                filter_id,
                &download_url,
                &subscription_url,
                is_trusted,
                is_enabled,
                is_installed,
                &version,
                last_update_time,
                last_download_time,
                expires,
            );
            signatures.push((filter_id, sig.to_string()));
        }

        Ok(signatures)
    }

    /// Iterates over all filter rows and verifies their metadata integrity
    /// signatures without loading all data into memory.
    /// Returns the `filter_id` of the first row that fails verification.
    pub(crate) fn verify_all_metadata_streaming(
        &self,
        conn: &Connection,
        derived_key: &[u8; 32],
    ) -> rusqlite::Result<Option<FilterId>> {
        let sql = format!("{} ORDER BY f.filter_id", BASIC_SELECT_SQL);
        let mut statement = conn.prepare(&sql)?;

        let mut rows = statement.query_map([], FilterEntity::hydrate)?;
        for entity in rows.by_ref() {
            let entity = entity?;
            if !verify_filter_entity(derived_key, &entity) {
                // filter_id should always be Some for persisted rows
                return Ok(Some(entity.filter_id.unwrap_or(0)));
            }
        }

        Ok(None)
    }

    /// Batch-updates `integrity_signature` from `(filter_id, signature)` pairs.
    pub(crate) fn batch_update_metadata_signatures(
        &self,
        tx: &Transaction<'_>,
        signatures: &[(FilterId, String)],
    ) -> rusqlite::Result<()> {
        let mut statement = tx.prepare(
            r"
            UPDATE
                [filter]
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

    /// Counts all filters in the database (used for integrity count signing).
    pub(crate) fn count_all(&self, conn: &Connection) -> rusqlite::Result<i64> {
        conn.query_row(
            r"
            SELECT
                COUNT(*)
            FROM
                [filter]",
            [],
            |row| row.get(0),
        )
    }

    /// Re-reads and re-signs metadata for specific filter_ids within a
    /// transaction. Used after operations that modify critical metadata fields
    /// (e.g. toggle is_enabled, is_installed, is_trusted).
    pub(crate) fn resign_filters_in_tx(
        &self,
        tx: &Transaction<'_>,
        filter_ids: &[FilterId],
        derived_key: &[u8; 32],
    ) -> rusqlite::Result<()> {
        if filter_ids.is_empty() {
            return Ok(());
        }

        let mut sql = String::from(
            r"
            SELECT
                filter_id,
                download_url,
                subscription_url,
                is_trusted,
                is_enabled,
                is_installed,
                version,
                last_update_time,
                last_download_time,
                expires
            FROM
                [filter]
            WHERE ",
        );

        sql += build_in_clause("filter_id", filter_ids.len()).as_str();

        let mut statement = tx.prepare(&sql)?;
        let mut rows = statement.query(params_from_iter(filter_ids))?;

        let signatures = Self::collect_metadata_signatures_from_rows(&mut rows, derived_key)?;

        self.batch_update_metadata_signatures(tx, &signatures)
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::entities::filter::filter_entity::FilterEntity;
    use crate::storage::repositories::filter_repository::FilterRepository;
    use crate::storage::repositories::Repository;
    use crate::storage::sql_generators::operator::SQLOperator;
    use crate::storage::with_transaction;
    use crate::storage::DbConnectionManager;
    use crate::test_utils::spawn_test_db_with_metadata;
    use crate::CUSTOM_FILTERS_GROUP_ID;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use rusqlite::types::Value;
    use rusqlite::{Connection, Transaction};

    #[test]
    fn test_count_negative_filters() {
        let source = DbConnectionManager::factory_test().unwrap();
        let _ = spawn_test_db_with_metadata(&source);
        let filter_repository = FilterRepository::new();

        {
            let inserted_entity = FilterEntity {
                filter_id: None,
                title: "Custom filter".to_string(),
                group_id: CUSTOM_FILTERS_GROUP_ID,
                description: "".to_string(),
                last_update_time: Default::default(),
                last_download_time: Default::default(),
                download_url: "https://example.com".to_string(),
                subscription_url: String::new(),
                is_enabled: false,
                version: "".to_string(),
                checksum: "".to_string(),
                license: "".to_string(),
                display_number: 0,
                is_trusted: false,
                expires: 0,
                homepage: "".to_string(),
                is_installed: false,
                is_user_title: None,
                is_user_description: None,
                integrity_signature: None,
            };

            source
                .execute_db(|mut connection: Connection| {
                    with_transaction(&mut connection, |transaction: &Transaction| {
                        filter_repository.insert(transaction, &[inserted_entity])
                    })
                    .unwrap();
                    Ok(())
                })
                .unwrap();
        }

        // Return all custom filters, except bootstrapped
        let cond = Some(FilterRepository::custom_filter_with_extra(
            FilterRepository::except_bootstrapped_filter_ids_operator(),
        ));

        let count = source
            .execute_db(|connection: Connection| {
                let custom_filters = filter_repository
                    .select(&connection, cond)
                    .unwrap()
                    .unwrap();

                let inserted_filter_id = custom_filters.first().unwrap().filter_id.unwrap();

                assert!(inserted_filter_id.is_negative());

                let count = filter_repository
                    .count(
                        &connection,
                        Some(SQLOperator::FieldEqualValue(
                            "filter_id",
                            inserted_filter_id.into(),
                        )),
                    )
                    .unwrap();

                Ok(count)
            })
            .unwrap();

        assert_eq!(count, 1);
    }

    #[test]
    fn test_update_custom_filter() {
        let source = DbConnectionManager::factory_test().unwrap();
        let _ = spawn_test_db_with_metadata(&source);
        let filter_repository = FilterRepository::new();

        {
            let inserted_entity = FilterEntity {
                filter_id: None,
                title: "Custom filter".to_string(),
                group_id: CUSTOM_FILTERS_GROUP_ID,
                description: "".to_string(),
                last_update_time: Default::default(),
                last_download_time: Default::default(),
                download_url: "https://example.com".to_string(),
                subscription_url: String::new(),
                is_enabled: false,
                version: "".to_string(),
                display_number: 0,
                checksum: "".to_string(),
                license: "".to_string(),
                is_trusted: false,
                expires: 0,
                homepage: "".to_string(),
                is_installed: false,
                is_user_title: None,
                is_user_description: None,
                integrity_signature: None,
            };

            source
                .execute_db(|mut connection: Connection| {
                    with_transaction(&mut connection, |transaction: &Transaction| {
                        filter_repository.insert(transaction, &[inserted_entity])
                    })
                    .unwrap();
                    Ok(())
                })
                .unwrap();
        }

        // Return all custom filters, except bootstrapped
        let cond = Some(FilterRepository::custom_filter_with_extra(
            FilterRepository::except_bootstrapped_filter_ids_operator(),
        ));

        let new_title = String::from("New title");
        let filter_id = source
            .execute_db(|mut connection: Connection| {
                let custom_filters = filter_repository
                    .select(&connection, cond)
                    .unwrap()
                    .unwrap();

                let inserted_filter = custom_filters.first().unwrap();

                let filter_id = inserted_filter.filter_id.unwrap();

                assert!(filter_id.is_negative());
                assert_eq!(inserted_filter.is_trusted, false);
                assert_eq!(inserted_filter.title, "Custom filter".to_string());

                with_transaction(&mut connection, |transaction: &Transaction| {
                    filter_repository.update_user_metadata_for_custom_filter(
                        transaction,
                        filter_id,
                        &new_title,
                        true,
                        true,
                    )
                })
                .unwrap();

                Ok(filter_id)
            })
            .unwrap();

        {
            source
                .execute_db(|connection: Connection| {
                    let updated_filters = filter_repository
                        .select(
                            &connection,
                            Some(SQLOperator::FieldEqualValue("filter_id", filter_id.into())),
                        )
                        .unwrap()
                        .unwrap();

                    let updated_filter = updated_filters.first().unwrap();

                    assert!(updated_filter.is_trusted);
                    assert_eq!(updated_filter.title, new_title);

                    Ok(())
                })
                .unwrap();
        }
    }

    #[test]
    fn test_toggle_filter_lists() {
        let filter_repository = FilterRepository::new();

        let source = DbConnectionManager::factory_test().unwrap();
        let (_, filter_lists) = spawn_test_db_with_metadata(&source);

        let mut rng = thread_rng();

        let mut ids = Vec::with_capacity(3);
        for filter in filter_lists.choose_multiple(&mut rng, ids.capacity()) {
            ids.push(filter.filter_id.unwrap());
            assert!(!filter.is_enabled);
        }

        source
            .execute_db(|mut connection: Connection| {
                let tx = connection.transaction().unwrap();
                let result = filter_repository
                    .toggle_filter_lists(&tx, ids.as_slice(), true)
                    .unwrap();
                tx.commit().unwrap();

                assert_eq!(result, ids.len());

                let values: Vec<Value> = ids.into_iter().map(|id| Value::from(id)).collect();

                let selected_filters = filter_repository
                    .select(&connection, Some(SQLOperator::FieldIn("filter_id", values)))
                    .unwrap()
                    .unwrap();

                for selected_filter in selected_filters {
                    assert!(selected_filter.is_enabled);
                }

                Ok(())
            })
            .unwrap()
    }

    #[test]
    fn test_toggle_is_installed() {
        let filter_repository = FilterRepository::new();

        let source = DbConnectionManager::factory_test().unwrap();
        let (_, filter_lists) = spawn_test_db_with_metadata(&source);

        let mut rng = thread_rng();

        let mut ids = Vec::with_capacity(3);
        for filter in filter_lists.choose_multiple(&mut rng, ids.capacity()) {
            ids.push(filter.filter_id.unwrap());
            assert!(!filter.is_installed);
        }

        source
            .execute_db(|mut connection: Connection| {
                let tx = connection.transaction().unwrap();
                let result = filter_repository
                    .toggle_is_installed(&tx, ids.as_slice(), true)
                    .unwrap();
                tx.commit().unwrap();

                assert_eq!(result, ids.len());

                let values: Vec<Value> = ids.into_iter().map(|id| Value::from(id)).collect();

                let selected_filters = filter_repository
                    .select(&connection, Some(SQLOperator::FieldIn("filter_id", values)))
                    .unwrap()
                    .unwrap();

                for selected_filter in selected_filters {
                    assert!(selected_filter.is_installed);
                }

                Ok(())
            })
            .unwrap();
    }

    #[test]
    fn test_insert_must_not_update_is_user_title_and_description_columns() {
        let source = DbConnectionManager::factory_test().unwrap();
        spawn_test_db_with_metadata(&source);

        let filter_id = -10011;

        // insert new filter
        let mut filter_entity = FilterEntity::default();
        filter_entity.filter_id = Some(filter_id);

        source
            .execute_db(|mut conn: Connection| {
                with_transaction(&mut conn, |tx: &Transaction| {
                    FilterRepository::new().insert(tx, &[filter_entity])
                })
                .unwrap();
                Ok(())
            })
            .unwrap();

        source
            .execute_db(|conn: Connection| {
                let filters = FilterRepository::new()
                    .select(
                        &conn,
                        Some(SQLOperator::FieldEqualValue("filter_id", filter_id.into())),
                    )
                    .unwrap()
                    .unwrap();

                assert!(!filters[0].is_user_title());
                assert!(!filters[0].is_user_description());

                Ok(())
            })
            .unwrap();

        let filter_id = -10012;

        // insert new filter with flags
        let mut filter_entity = FilterEntity::default();
        filter_entity.filter_id = Some(filter_id);
        filter_entity.set_is_user_title(true);
        filter_entity.set_is_user_description(true);

        source
            .execute_db(|mut conn: Connection| {
                with_transaction(&mut conn, |tx: &Transaction| {
                    FilterRepository::new().insert(tx, &[filter_entity])
                })
                .unwrap();
                Ok(())
            })
            .unwrap();

        source
            .execute_db(|conn: Connection| {
                let filters = FilterRepository::new()
                    .select(
                        &conn,
                        Some(SQLOperator::FieldEqualValue("filter_id", filter_id.into())),
                    )
                    .unwrap()
                    .unwrap();

                assert!(filters[0].is_user_title());
                assert!(filters[0].is_user_description());

                Ok(())
            })
            .unwrap();

        // insert existed filter
        let mut filter_entity = FilterEntity::default();
        filter_entity.filter_id = Some(filter_id);

        source
            .execute_db(|mut conn: Connection| {
                with_transaction(&mut conn, |tx: &Transaction| {
                    FilterRepository::new().insert(tx, &[filter_entity])
                })
                .unwrap();
                Ok(())
            })
            .unwrap();

        source
            .execute_db(|conn: Connection| {
                let filters = FilterRepository::new()
                    .select(
                        &conn,
                        Some(SQLOperator::FieldEqualValue("filter_id", filter_id.into())),
                    )
                    .unwrap()
                    .unwrap();

                assert!(filters[0].is_user_title());
                assert!(filters[0].is_user_description());

                Ok(())
            })
            .unwrap();
    }
}
