use crate::storage::{DNS_FILTERS_DATABASE_FILENAME, STANDARD_FILTERS_DATABASE_FILENAME};
use crate::{Configuration, FLMError, FLMResult, FilterListType};
use std::env;
use std::path::PathBuf;

/// Structure for determining final database file path.
/// This MUST build path in constructors.
#[cfg_attr(test, derive(Clone))]
pub struct DatabasePathHolder {
    calculated_path: PathBuf,
}

impl DatabasePathHolder {
    /// Path getter
    pub(crate) fn get_calculated_path(&self) -> &PathBuf {
        &self.calculated_path
    }
}

impl DatabasePathHolder {
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
        }
    }
}

#[cfg(test)]
impl DatabasePathHolder {
    pub(crate) fn from_filter_list_type(filter_list_type: FilterListType) -> FLMResult<Self> {
        let cwd = env::current_dir().map_err(FLMError::from_io)?;

        Ok(Self::build_with_dir(cwd, filter_list_type))
    }

    pub(crate) fn factory_test() -> FLMResult<DatabasePathHolder> {
        Self::from_filter_list_type(FilterListType::STANDARD)
    }
}
