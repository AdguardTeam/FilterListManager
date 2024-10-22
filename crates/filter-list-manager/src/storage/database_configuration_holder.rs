use crate::storage::{DNS_FILTERS_DATABASE_FILENAME, STANDARD_FILTERS_DATABASE_FILENAME};
use crate::{Configuration, DbJournalMode, FLMError, FLMResult, FilterListType};
use std::env;
use std::path::PathBuf;

/// Structure for database configuration. Also, used to calculate the absolute path for a database.
/// This MUST build path in constructors.
#[cfg_attr(test, derive(Clone))]
pub struct DatabaseConfigurationHolder {
    calculated_path: PathBuf,
    db_journal_mode: DbJournalMode,
}

impl DatabaseConfigurationHolder {
    /// Path getter
    pub(crate) fn get_calculated_path(&self) -> &PathBuf {
        &self.calculated_path
    }

    /// Gets journal_mode
    pub(crate) fn get_journal_mode(&self) -> DbJournalMode {
        self.db_journal_mode
    }
}

impl DatabaseConfigurationHolder {
    /// Default ctor
    pub(crate) fn from_configuration(configuration: &Configuration) -> FLMResult<Self> {
        let calculated_dir = match configuration.working_directory {
            None => env::current_dir().map_err(FLMError::from_io),
            Some(ref str) => Ok(PathBuf::from(str)),
        }?;

        Ok(Self::build_with_dir(
            calculated_dir,
            configuration.filter_list_type,
            DbJournalMode::WAL, // TODO: Need tests
        ))
    }

    #[cfg(test)]
    #[inline]
    fn build_with_dir(
        mut dir: PathBuf,
        filter_list_type: FilterListType,
        db_journal_mode: DbJournalMode,
    ) -> Self {
        let file_name = match filter_list_type {
            FilterListType::STANDARD => STANDARD_FILTERS_DATABASE_FILENAME,
            FilterListType::DNS => DNS_FILTERS_DATABASE_FILENAME,
        };

        dir.push(file_name);

        use crate::test_utils::do_with_tests_helper;
        dir = do_with_tests_helper(|mut helper| helper.build_temporary_db_name(dir));

        Self {
            calculated_path: dir,
            db_journal_mode,
        }
    }

    #[cfg(not(test))]
    #[inline]
    fn build_with_dir(
        mut dir: PathBuf,
        filter_list_type: FilterListType,
        db_journal_mode: DbJournalMode,
    ) -> Self {
        let file_name = match filter_list_type {
            FilterListType::STANDARD => STANDARD_FILTERS_DATABASE_FILENAME,
            FilterListType::DNS => DNS_FILTERS_DATABASE_FILENAME,
        };

        dir.push(file_name);

        Self {
            calculated_path: dir,
            db_journal_mode,
        }
    }
}

#[cfg(test)]
impl DatabaseConfigurationHolder {
    pub(crate) fn factory_test() -> FLMResult<DatabaseConfigurationHolder> {
        let cwd = env::current_dir().map_err(FLMError::from_io)?;

        Ok(Self::build_with_dir(
            cwd,
            FilterListType::STANDARD,
            DbJournalMode::WAL,
        ))
    }
}
