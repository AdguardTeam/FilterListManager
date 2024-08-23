use crate::filters::indexes::entities::IndexEntity;
use crate::{FLMError, FLMResult};
use std::collections::HashSet;

/// Helper for checking downloaded index consistency
pub(super) struct IndexConsistencyChecker {
    existing_groups_ids: HashSet<i32>,
    existing_tags_ids: HashSet<i32>,
}

impl IndexConsistencyChecker {
    pub(super) fn new() -> Self {
        Self {
            existing_tags_ids: HashSet::new(),
            existing_groups_ids: HashSet::new(),
        }
    }

    /// Check consistency of downloaded `index`
    ///
    /// # Failure
    ///
    /// Returns an error if the `index` is inconsistent.
    pub(super) fn check(&mut self, index: &IndexEntity) -> FLMResult<()> {
        for filter in &index.filters {
            if filter.filterId <= 0 {
                return FLMError::make_err(format!(
                    "[IDX Consistency] Filter id must be > 0: \"{}\"",
                    filter.filterId
                ));
            }

            if filter.name.is_empty() {
                return FLMError::make_err(format!(
                    "[IDX Consistency] Filter name is empty for filter with id: \"{}\"",
                    filter.filterId
                ));
            }

            if !self.existing_groups_ids.contains(&filter.groupId) {
                let group_for_filter = index
                    .groups
                    .iter()
                    .find(|group| group.group_id == filter.groupId);

                if group_for_filter.is_none() {
                    return FLMError::make_err(format!(
                        "[IDX Consistency] Cannot find group with id \"{}\" for filter_id \"{}\"",
                        filter.groupId, filter.filterId
                    ));
                } else {
                    self.existing_groups_ids.insert(filter.groupId);
                }
            }

            for tag_id in &filter.tags {
                if !self.existing_tags_ids.contains(&tag_id) {
                    let tag_for_filter = index.tags.iter().find(|tag| tag.tag_id == *tag_id);

                    if tag_for_filter.is_none() {
                        return FLMError::make_err(format!(
                            "[IDX Consistency] Cannot find tag with id \"{}\" for filter_id \"{}\"",
                            tag_id, filter.filterId
                        ));
                    } else {
                        self.existing_groups_ids.insert(*tag_id);
                    }
                }
            }

            if filter.name.is_empty() {
                return FLMError::make_err(format!(
                    "[IDX Consistency] Title for filter with id \"{}\" is empty",
                    filter.filterId
                ));
            }
        }

        Ok(())
    }
}
