mod connect;

use self::connect::{connect, connect_with_create};
use crate::storage::database_status::{get_database_status, DatabaseStatus};
use crate::storage::db_bootstrap::db_bootstrap;
use crate::storage::migrations::run_migrations;
use crate::{
    Configuration, FLMError, FLMResult, FilterListType, DNS_FILTERS_DATABASE_FILENAME,
    STANDARD_FILTERS_DATABASE_FILENAME,
};
use rusqlite::Connection;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Original database schema as string
const SCHEMA_STR: &str = include_str!("../../../resources/sql/schema.sql");

/// Structure for database configuration. Also, used to calculate the absolute path for a database.
/// This MUST build path in constructors.
#[cfg_attr(test, derive(Clone))]
pub struct DbConnectionManager {
    calculated_path: PathBuf,
    db_mutex: Arc<Mutex<()>>,
}

impl DbConnectionManager {
    /// Path getter
    pub(crate) fn get_calculated_path(&self) -> &PathBuf {
        &self.calculated_path
    }

    // TODO: Can add something for calls interference blocking?
    /// Database execution block.
    /// *ALL* database queries *except* for `lift_up_database` *must* take place inside this `block`
    pub(crate) fn execute_db<Block, Out>(&self, block: Block) -> FLMResult<Out>
    where
        Block: FnOnce(Connection) -> FLMResult<Out>,
    {
        let _guard = self.db_mutex.lock();
        let connection = connect(self)?;
        block(connection)
    }

    /// "Lifting" [`DatabaseStatus`] for filling database with index
    /// SAFETY: YOU SHOULD NEVER CALL THIS INSIDE [`Self::execute_db`] method.
    pub(crate) unsafe fn lift_up_database(&self) -> FLMResult<()> {
        let _guard = self.db_mutex.lock();

        // First of all, create folder
        crate::storage::database_status::create_db_folder_if_it_does_not_exist(
            self.get_calculated_path().to_owned(),
        )?;

        let mut conn = connect_with_create(self)?;
        let mut tx = conn.transaction().map_err(FLMError::from_database)?;

        match get_database_status(&tx, self.get_calculated_path())? {
            DatabaseStatus::NoFile | DatabaseStatus::NoSchema => {
                tx.execute_batch(SCHEMA_STR)
                    .map_err(FLMError::from_database)?;

                db_bootstrap(&mut tx).map_err(FLMError::from_database)?;
            }
            DatabaseStatus::OnlySchema => {
                db_bootstrap(&mut tx).map_err(FLMError::from_database)?;
            }
            DatabaseStatus::Filled => {}
        };

        run_migrations(&mut tx)?;

        tx.commit().map_err(FLMError::from_database)
    }
}

impl DbConnectionManager {
    /// Default ctor
    pub(crate) fn from_configuration(configuration: &Configuration) -> FLMResult<Self> {
        let calculated_dir = match configuration.working_directory {
            None => env::current_dir().map_err(FLMError::from_io),
            Some(ref str) => Ok(PathBuf::from(str)),
        }?;

        Ok(Self::build_with_dir(
            calculated_dir,
            configuration.filter_list_type,
        ))
    }

    #[cfg(test)]
    #[inline]
    fn build_with_dir(mut dir: PathBuf, filter_list_type: FilterListType) -> Self {
        let file_name = match filter_list_type {
            FilterListType::STANDARD => STANDARD_FILTERS_DATABASE_FILENAME,
            FilterListType::DNS => DNS_FILTERS_DATABASE_FILENAME,
        };

        dir.push(file_name);

        use crate::test_utils::do_with_tests_helper;
        dir = do_with_tests_helper(|mut helper| helper.build_temporary_db_name(dir));

        Self {
            calculated_path: dir,
            db_mutex: Arc::new(Mutex::new(())),
        }
    }

    #[cfg(not(test))]
    #[inline]
    fn build_with_dir(mut dir: PathBuf, filter_list_type: FilterListType) -> Self {
        let file_name = match filter_list_type {
            FilterListType::STANDARD => STANDARD_FILTERS_DATABASE_FILENAME,
            FilterListType::DNS => DNS_FILTERS_DATABASE_FILENAME,
        };

        dir.push(file_name);

        Self {
            calculated_path: dir,
            db_mutex: Arc::new(Mutex::new(())),
        }
    }
}

#[cfg(test)]
impl DbConnectionManager {
    pub(crate) fn factory_test() -> FLMResult<DbConnectionManager> {
        let cwd = env::current_dir().map_err(FLMError::from_io)?;

        Ok(Self::build_with_dir(cwd, FilterListType::STANDARD))
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::entities::filter_entity::FilterEntity;
    use crate::storage::repositories::filter_repository::FilterRepository;
    use crate::storage::DbConnectionManager;
    use crate::Configuration;
    use rusqlite::Connection;
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn test_dumb_multithreading_database_execution() {
        let dcm =
            Arc::new(DbConnectionManager::from_configuration(&Configuration::default()).unwrap());

        unsafe { dcm.lift_up_database().unwrap() }

        let t2_dcm = Arc::clone(&dcm);
        let t3_dcm = Arc::clone(&dcm);

        let t2_handle = std::thread::spawn(move || {
            t2_dcm
                .execute_db(|mut connection: Connection| {
                    let tx = connection.transaction().unwrap();

                    let mut entity = FilterEntity::default();
                    entity.title = String::from("test multithreading t2");

                    std::thread::sleep(Duration::from_millis(350));

                    FilterRepository::new()
                        .only_insert_row(&tx, entity)
                        .unwrap();
                    tx.commit().unwrap();

                    std::thread::sleep(Duration::from_millis(600));

                    Ok(())
                })
                .unwrap();
        });

        // Wait until first thread joined
        std::thread::sleep(Duration::from_millis(50));

        let t3_handle = std::thread::spawn(move || {
            t3_dcm
                .execute_db(|mut connection: Connection| {
                    let tx = connection.transaction().unwrap();

                    let mut entity = FilterEntity::default();
                    entity.title = String::from("test multithreading t3");

                    FilterRepository::new()
                        .only_insert_row(&tx, entity)
                        .unwrap();
                    tx.commit().unwrap();

                    std::thread::sleep(Duration::from_millis(900));

                    Ok(())
                })
                .unwrap();
        });

        // Wait until both threads joined
        std::thread::sleep(Duration::from_millis(50));

        let filters = dcm
            .execute_db(|connection: Connection| {
                let filters = FilterRepository::new()
                    .select_filters_except_bootstrapped(&connection)
                    .unwrap()
                    .unwrap();

                Ok(filters)
            })
            .unwrap();

        assert_eq!(
            filters.len(),
            2usize,
            "Both filters must be inserted before joining main thread"
        );

        // T2 was inserted first, BUT because custom filters ids are decrementing,
        // it will be second in vec
        assert_eq!(
            filters[1].title, "test multithreading t2",
            "t2 must be inserted first"
        );

        t2_handle.join().unwrap();
        t3_handle.join().unwrap();
    }
}
