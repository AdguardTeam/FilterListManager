use rusqlite::Connection;

use crate::storage::repositories::filter_group_repository::FilterGroupRepository;
use crate::storage::DbConnectionManager;
use crate::Configuration;
use crate::FLMError;
use crate::FLMResult;
use crate::FilterGroup;

/// Manager for filter group logic
pub(crate) struct FilterGroupManager;

impl FilterGroupManager {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    /// Gets all groups
    pub(crate) fn get_all_groups(
        &self,
        connection_manager: &DbConnectionManager,
        configuration: &Configuration,
    ) -> FLMResult<Vec<FilterGroup>> {
        let all_groups: Vec<FilterGroup> = connection_manager.execute_db(|conn: Connection| {
            FilterGroupRepository::new()
                .select_localised_with_block(&configuration.locale, &conn, FilterGroup::from)
                .map_err(FLMError::from_database)
        })?;

        Ok(all_groups)
    }
}
