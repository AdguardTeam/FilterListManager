use rusqlite::Connection;

use crate::storage::repositories::filter_tag_repository::FilterTagRepository;
use crate::storage::DbConnectionManager;
use crate::FLMError;
use crate::FLMResult;
use crate::FilterTag;

/// Manager for filter tag logic
pub(crate) struct FilterTagManager;

impl FilterTagManager {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    /// Gets all tags
    pub(crate) fn get_all_tags(
        &self,
        connection_manager: &DbConnectionManager,
    ) -> FLMResult<Vec<FilterTag>> {
        let all_tags: Vec<FilterTag> = connection_manager.execute_db(|conn: Connection| {
            FilterTagRepository::new()
                .select_with_block(&conn, FilterTag::from)
                .map_err(FLMError::from_database)
        })?;

        Ok(all_tags)
    }
}
