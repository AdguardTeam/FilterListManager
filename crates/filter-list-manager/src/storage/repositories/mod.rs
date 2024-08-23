use crate::storage::utils::build_in_clause;
use rusqlite::{params_from_iter, ToSql, Transaction};

pub(crate) mod db_metadata_repository;
pub(crate) mod db_schema_repository;
pub(crate) mod diff_updates_repository;
pub(crate) mod filter_filter_tag_repository;
pub(crate) mod filter_group_repository;
pub(crate) mod filter_locale_repository;
pub(crate) mod filter_repository;
pub(crate) mod filter_tag_repository;
pub(crate) mod localisation;
pub(crate) mod rules_list_repository;

pub(crate) trait Repository<Entity> {
    const TABLE_NAME: &'static str;

    fn insert(&self, conn: &Transaction<'_>, entities: Vec<Entity>) -> Result<(), rusqlite::Error>;

    fn clear(&self, transaction: &Transaction) -> rusqlite::Result<()> {
        let mut statement =
            transaction.prepare(format!("DELETE FROM {} WHERE 1", Self::TABLE_NAME).as_str())?;

        statement.execute(())?;

        Ok(())
    }
}

pub(crate) trait BulkDeleteRepository<Entity, PK = i32>: Repository<Entity>
where
    PK: ToSql,
{
    const PK_FIELD: &'static str;

    fn bulk_delete(&self, transaction: &Transaction, ids: &Vec<PK>) -> rusqlite::Result<usize> {
        let mut statement = transaction.prepare(
            format!(
                "DELETE FROM {} WHERE {} {}",
                Self::TABLE_NAME,
                Self::PK_FIELD,
                build_in_clause(ids.len())
            )
            .as_str(),
        )?;

        statement.execute(params_from_iter(ids))
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::repositories::filter_group_repository::FilterGroupRepository;
    use crate::storage::repositories::filter_repository::FilterRepository;
    use crate::storage::repositories::{BulkDeleteRepository, Repository};
    use crate::storage::with_transaction;
    use crate::test_utils::{do_with_tests_helper, spawn_test_db_with_metadata};
    use crate::FilterId;
    use rusqlite::Transaction;

    #[test]
    fn test_bulk_delete_filters() {
        do_with_tests_helper(|mut helper| helper.increment_postfix());

        let (_, mut conn, index_filters) = spawn_test_db_with_metadata();
        let filter_repository = FilterRepository::new();
        let original_len = filter_repository
            .select(&conn, None)
            .unwrap()
            .unwrap()
            .len();

        let delete_count: usize = 5;

        let ids_source_vec = &index_filters[0..delete_count].to_vec();
        let ids: Vec<FilterId> = ids_source_vec
            .iter()
            .map(|f| f.filter_id.unwrap())
            .collect();

        let deleted_size = with_transaction(&mut conn, |transaction: &Transaction| {
            Ok(filter_repository.bulk_delete(&transaction, &ids))
        })
        .unwrap()
        .unwrap();

        let new_filters_count = filter_repository
            .select(&conn, None)
            .unwrap()
            .unwrap()
            .len();

        assert_eq!(deleted_size, delete_count);
        assert_eq!(new_filters_count, original_len - delete_count)
    }

    #[test]
    fn test_clear_table() {
        do_with_tests_helper(|mut helper| helper.increment_postfix());

        let (_, mut conn, _) = spawn_test_db_with_metadata();
        let group_repository = FilterGroupRepository::new();
        let original_len = group_repository.select_mapped(&conn).unwrap().len();

        assert_ne!(original_len, 0);

        with_transaction(&mut conn, |transaction: &Transaction| {
            Ok(group_repository.clear(&transaction))
        })
        .unwrap()
        .unwrap();

        assert_eq!(group_repository.select_mapped(&conn).unwrap().len(), 0);
    }
}
