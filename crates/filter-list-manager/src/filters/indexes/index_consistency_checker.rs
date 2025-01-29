use crate::filters::indexes::entities::IndexEntity;
use crate::{FLMError, FLMResult};
use std::collections::HashSet;

#[allow(clippy::bool_comparison)]
/// Check consistency of downloaded `index`
///
/// # Failure
///
/// Returns an error if the `index` is inconsistent.
pub(super) fn check_consistency(index: &IndexEntity) -> FLMResult<()> {
    let mut existing_tags_ids: HashSet<i32> = HashSet::new();
    let mut existing_groups_ids: HashSet<i32> = HashSet::new();
    let mut download_urls_set: HashSet<&str> = HashSet::new();

    for filter in &index.filters {
        if filter.filterId <= 0 {
            return FLMError::make_err(format!(
                "[IDX Consistency] Filter id must be > 0: \"{}\"",
                filter.filterId
            ));
        }

        if download_urls_set.insert(filter.downloadUrl.as_str()) == false {
            return FLMError::make_err(format!(
                "[IDX Consistency] Two or more filters have the same download_url: \"{}\"",
                filter.downloadUrl
            ));
        }

        if filter.name.is_empty() {
            return FLMError::make_err(format!(
                "[IDX Consistency] Filter name is empty for filter with id: \"{}\"",
                filter.filterId
            ));
        }

        if !existing_groups_ids.contains(&filter.groupId) {
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
                existing_groups_ids.insert(filter.groupId);
            }
        }

        for tag_id in &filter.tags {
            if !existing_tags_ids.contains(tag_id) {
                let tag_for_filter = index.tags.iter().find(|tag| tag.tag_id == *tag_id);

                if tag_for_filter.is_none() {
                    return FLMError::make_err(format!(
                        "[IDX Consistency] Cannot find tag with id \"{}\" for filter_id \"{}\"",
                        tag_id, filter.filterId
                    ));
                } else {
                    existing_tags_ids.insert(*tag_id);
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::filters::indexes::index_consistency_checker::check_consistency;
    use crate::test_utils::indexes_fixtures::build_filters_indices_fixtures;
    use crate::FLMError;

    #[test]
    fn test_consistency_fails() {
        let (index, _) = build_filters_indices_fixtures().unwrap();

        {
            // zero filter_id

            let mut test_index = index.clone();
            test_index.filters[0].filterId = 0;

            let actual = check_consistency(&test_index);

            assert_eq!(
                actual,
                FLMError::make_err(format!(
                    "[IDX Consistency] Filter id must be > 0: \"{}\"",
                    0
                ))
            );
        }

        {
            // negative filter_id
            [-100, -10_005, -2_000_000_000].into_iter().for_each(|id| {
                let mut test_index = index.clone();
                test_index.filters[0].filterId = id;

                let actual = check_consistency(&test_index);

                assert_eq!(
                    actual,
                    FLMError::make_err(format!(
                        "[IDX Consistency] Filter id must be > 0: \"{}\"",
                        id
                    ))
                );
            });
        }

        {
            // Duplicated download_url
            let mut test_index = index.clone();

            let test_url = test_index.filters[1].downloadUrl.clone();
            test_index.filters[0].downloadUrl = test_url.clone();

            let actual = check_consistency(&test_index);

            assert_eq!(
                actual,
                FLMError::make_err(format!(
                    "[IDX Consistency] Two or more filters have the same download_url: \"{}\"",
                    test_url
                ))
            );
        }

        {
            // Nonexistent group
            let mut test_index = index.clone();

            let new_group_id = 1_000_135;
            let filter_id = test_index.filters[0].filterId;

            test_index.filters[0].groupId = new_group_id;

            let actual = check_consistency(&test_index);

            assert_eq!(
                actual,
                FLMError::make_err(format!(
                    "[IDX Consistency] Cannot find group with id \"{}\" for filter_id \"{}\"",
                    new_group_id, filter_id
                ))
            );
        }

        {
            // Nonexistent tag
            let mut test_index = index.clone();

            let chosen_filter = test_index
                .filters
                .iter_mut()
                .find(|filter| filter.tags.len() > 0)
                .unwrap();

            let new_tag_id = 1_000_135;
            let filter_id = chosen_filter.filterId;

            chosen_filter.tags[0] = new_tag_id;

            let actual = check_consistency(&test_index);

            assert_eq!(
                actual,
                FLMError::make_err(format!(
                    "[IDX Consistency] Cannot find tag with id \"{}\" for filter_id \"{}\"",
                    new_tag_id, filter_id
                ))
            );
        }

        {
            // Empty name for filter
            let mut test_index = index.clone();
            let filter_id = test_index.filters[0].filterId;
            test_index.filters[0].name = String::new();

            let actual = check_consistency(&test_index);

            assert_eq!(
                actual,
                FLMError::make_err(format!(
                    "[IDX Consistency] Filter name is empty for filter with id: \"{}\"",
                    filter_id
                ))
            );
        }
    }
}
