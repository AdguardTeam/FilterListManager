use crate::manager::models::FilterId;
use crate::storage::db_bootstrap::get_bootstrapped_filter_id;
use crate::storage::entities::db_metadata_entity::DBMetadataEntity;
use crate::storage::entities::filter_inner_flag_entity::FilterInnerFlagEntity;
use crate::storage::repositories::db_metadata_repository::DBMetadataRepository;
use crate::storage::repositories::BulkDeleteRepository;
use crate::storage::sql_generators::operator::SQLOperator;
use crate::storage::utils::{build_in_clause, process_where_clause};
use crate::utils::memory::heap;
use crate::{
    storage::{entities::filter_entity::FilterEntity, repositories::Repository},
    MAXIMUM_CUSTOM_FILTER_ID, MINIMUM_CUSTOM_FILTER_ID,
};
use rusqlite::types::Type;
use rusqlite::{
    named_params, params_from_iter, Connection, Error, OptionalExtension, Row, Transaction,
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
        f.is_user_description
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

/// Queries
impl FilterRepository {
    pub(crate) const fn new() -> Self {
        Self {}
    }

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
        let last_insert_id = self.insert_internal(transaction, &[entity], HashMap::new())?;

        let mut sql = String::from(BASIC_SELECT_SQL);
        sql += "WHERE f.filter_id=?";

        transaction.query_row(sql.as_str(), [last_insert_id], FilterRepository::hydrate)
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
            .query_map(params, FilterRepository::hydrate)
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

        let rows = statement.query_map(params, FilterRepository::hydrate)?;

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

    /// Returns filled entity from row
    fn hydrate(row: &Row) -> rusqlite::Result<FilterEntity> {
        Ok(FilterEntity {
            filter_id: row.get(0)?,
            group_id: row.get(1)?,
            version: row.get(2)?,
            last_update_time: row.get(3)?,
            last_download_time: row.get(4)?,
            display_number: row.get(5)?,
            title: row.get(6)?,
            description: row.get(7)?,
            homepage: row.get(8)?,
            license: row.get(9)?,
            checksum: row.get(10)?,
            expires: row.get(11)?,
            download_url: row.get(12)?,
            subscription_url: row.get(13)?,
            is_enabled: row.get(14)?,
            is_installed: row.get(15)?,
            is_trusted: row.get(16)?,
            is_user_title: row.get(17)?,
            is_user_description: row.get(18)?,
        })
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

    fn insert_internal(
        &self,
        transaction: &Transaction<'_>,
        entities: &[FilterEntity],
        filter_inner_flag_entities: HashMap<FilterId, FilterInnerFlagEntity>,
    ) -> rusqlite::Result<i64> {
        let mut custom_filters_count: FilterId = 0;
        for entity in entities.iter() {
            if entity.filter_id.is_none() && entity.is_custom() {
                custom_filters_count += 1;
            }
        }

        // Need to use autoincrement
        let mut metadata_entity: Option<DBMetadataEntity> = None;
        if custom_filters_count > 0 {
            // If empty, there is a problem with db
            let db_metadata = match DBMetadataRepository::read(transaction)? {
                None => return Err(Error::QueryReturnedNoRows),
                Some(metadata) => metadata,
            };

            // Check negative autoincrement
            if db_metadata.custom_filters_autoincrement_value > MAXIMUM_CUSTOM_FILTER_ID
                || (db_metadata.custom_filters_autoincrement_value - custom_filters_count)
                    < MINIMUM_CUSTOM_FILTER_ID
            {
                return Err(Error::InvalidParameterName(
                    "custom_filter_increment".to_string(),
                ));
            }

            metadata_entity = Some(db_metadata);
        }

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
                    is_user_description
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
                    :is_user_description
                )",
        )?;

        for entity in entities.iter() {
            // Should take special autoincrement id for new custom filters
            let filter_id = match metadata_entity {
                None => entity.filter_id,
                Some(ref mut metadata_ref) => {
                    if entity.is_custom() && entity.filter_id.is_none() {
                        metadata_ref.custom_filters_autoincrement_value -= 1;
                        Some(metadata_ref.custom_filters_autoincrement_value)
                    } else {
                        // Non-custom filters always must have their own filter id
                        entity.filter_id
                    }
                }
            };

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
        self.insert_internal(transaction, entities, filter_inner_flag_entities)
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

#[cfg(test)]
mod tests {
    use crate::storage::entities::filter_entity::FilterEntity;
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
