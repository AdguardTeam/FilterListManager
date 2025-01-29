use crate::filters::parser::diff_updates::validate_patch::validate_patch;
use crate::filters::parser::{
    diff_updates::batch_patches_container::BatchPatchesContainer,
    diff_updates::diff_directives::extract_patch, filter_contents_provider::FilterContentsProvider,
    paths::resolve_absolute_uri, rcs_diff::apply_patch,
};
use crate::io::fetch_by_schemes::fetch_by_scheme;
use crate::io::http::blocking_client::BlockingClient;
use crate::io::url_schemes::UrlSchemes;
use crate::io::{get_hash_from_url, get_scheme};
use crate::FilterParserError;
use std::cell::RefCell;
use std::rc::Rc;

/// This provider is used to download and process incremental filter updates
pub(crate) struct DiffPathProvider<'a> {
    /// Relative path from Diff-Path
    patch_relative_path: String,
    /// Contents of saved filter
    base_filter_contents: String,
    /// Container for shared patch files
    batch_patches_container: Rc<RefCell<BatchPatchesContainer>>,
    /// Shared sync http client
    shared_http_client: &'a BlockingClient,
}

impl<'a> DiffPathProvider<'a> {
    pub(crate) fn new(
        patch_relative_path: String,
        base_filter_contents: String,
        batch_patches_container: Rc<RefCell<BatchPatchesContainer>>,
        shared_http_client: &'a BlockingClient,
    ) -> Self {
        Self {
            patch_relative_path,
            base_filter_contents,
            batch_patches_container,
            shared_http_client,
        }
    }

    /// Will work here with patch contents, apply and validate them
    #[cfg_attr(not(test), inline)]
    fn do_patch(
        &self,
        patch_file_contents: &str,
        resource_name: Option<String>,
    ) -> Result<String, FilterParserError> {
        if patch_file_contents.is_empty() {
            return Err(FilterParserError::NoContent);
        }

        let (diff_directive_option, prepared_patch, end_of_chunk_is_eof) =
            extract_patch(patch_file_contents, resource_name)?;
        let mut patch_lines_count = prepared_patch.len();

        // If this chunk contains eof, we are truly knows
        if end_of_chunk_is_eof {
            patch_lines_count -= 1;
        }

        match apply_patch(self.base_filter_contents.as_str(), prepared_patch) {
            Ok(patch_result) if patch_result.is_empty() => {
                FilterParserError::other_err_from_to_string("Patch result is empty")
            }
            Ok(patch_result) => {
                if let Some(diff_directive) = diff_directive_option {
                    validate_patch(diff_directive, patch_lines_count, patch_result.as_str())?;
                }

                Ok(patch_result)
            }
            Err(e) => Err(e),
        }
    }
}

impl FilterContentsProvider for DiffPathProvider<'_> {
    fn get_filter_contents(&self, root_filter_url: &str) -> Result<String, FilterParserError> {
        let scheme = UrlSchemes::from(get_scheme(root_filter_url));

        let patch_file_absolute_uri =
            resolve_absolute_uri(scheme, root_filter_url, self.patch_relative_path.as_str())?;

        // If resource name exist we assume that this is batch patch file
        // According to <https://github.com/ameshkov/diffupdates/tree/master?tab=readme-ov-file#algorithm>
        // we must load patch diff file only once
        let (resource_name, file_contents) =
            match get_hash_from_url(patch_file_absolute_uri.as_str()) {
                Some((patch_path, resource_name)) => {
                    let mut pinned_container = self.batch_patches_container.borrow_mut();

                    let diff_contents = match pinned_container.get_a_copy(&patch_path) {
                        None => {
                            let body =
                                fetch_by_scheme(&patch_path, scheme, self.get_http_client())?;

                            pinned_container.insert(patch_path, body.clone());

                            body
                        }
                        Some(string) => string,
                    };

                    (Some(resource_name), diff_contents)
                }
                None => {
                    let diff_contents =
                        fetch_by_scheme(&patch_file_absolute_uri, scheme, self.get_http_client())?;

                    (None, diff_contents)
                }
            };

        self.do_patch(file_contents.as_str(), resource_name)
    }

    fn get_http_client(&self) -> &BlockingClient {
        self.shared_http_client
    }
}

#[cfg(test)]
mod tests {
    use crate::filters::parser::checksum_validator::validate_checksum;
    use crate::filters::parser::diff_updates::batch_patches_container::BatchPatchesContainer;
    use crate::filters::parser::filter_contents_provider::diff_path_provider::DiffPathProvider;
    use crate::filters::parser::filter_contents_provider::FilterContentsProvider;
    use crate::test_utils::{tests_path, SHARED_TEST_BLOCKING_HTTP_CLIENT};
    use url::Url;

    #[test]
    fn test_batch_validation() {
        let list1_v100 = include_str!(
            "../../../../tests/fixtures/diffupdates/examples/03_batch/list1/list1_v1.0.0.txt"
        );
        let list1_v101 = include_str!(
            "../../../../tests/fixtures/diffupdates/examples/03_batch/list1/list1_v1.0.1.txt"
        );
        let list2_v100 = include_str!(
            "../../../../tests/fixtures/diffupdates/examples/03_batch/list2/list2_v1.0.0.txt"
        );
        let list2_v101 = include_str!(
            "../../../../tests/fixtures/diffupdates/examples/03_batch/list2/list2_v1.0.1.txt"
        );
        let batch_patch =
            include_str!("../../../../tests/fixtures/diffupdates/examples/03_batch/patches/batch_v1.0.0-s-1700045842-3600.patch");

        let container = BatchPatchesContainer::factory();
        container.borrow_mut().insert(
            "https://example.org/patches/batch_v1.0.0-s-1700045842-3600.patch".to_string(),
            batch_patch.to_string(),
        );

        let provider1 = DiffPathProvider::new(
            "../patches/batch_v1.0.0-s-1700045842-3600.patch#list1".to_string(),
            list1_v100.to_string(),
            container.clone(),
            &SHARED_TEST_BLOCKING_HTTP_CLIENT,
        );
        assert_eq!(
            list1_v101,
            provider1
                .get_filter_contents("https://example.org/lists/list1.txt")
                .unwrap()
        );

        let provider2 = DiffPathProvider::new(
            "../patches/batch_v1.0.0-s-1700045842-3600.patch#list2".to_string(),
            list2_v100.to_string(),
            container.clone(),
            &SHARED_TEST_BLOCKING_HTTP_CLIENT,
        );
        assert_eq!(
            list2_v101,
            provider2
                .get_filter_contents("https://example.org/lists/list2.txt")
                .unwrap()
        );
    }

    #[test]
    fn test_diff_path_add_newlines() {
        let v1 = "! Title: Batch-Updatable List 1
! Diff-Path: ../patches/batch_v1.0.0-s-1700045842-3600.patch#list1
||example.org^";

        let v2 = "! Title: Batch-Updatable List 1
! Diff-Path: ../patches/batch_v1.0.1-s-1700049442-3600.patch#list1



||example.com^
";

        let patch = "diff name:list1 checksum:b473858bee9887c7711032513e15b7fc9d1b367e lines:7
d2 2
a3 6
! Diff-Path: ../patches/batch_v1.0.1-s-1700049442-3600.patch#list1



||example.com^
";
        let container = BatchPatchesContainer::factory();
        container.borrow_mut().insert(
            "https://example.org/patches/batch_v1.0.0-s-1700045842-3600.patch".to_string(),
            patch.to_string(),
        );

        let provider1 = DiffPathProvider::new(
            "../patches/batch_v1.0.0-s-1700045842-3600.patch#list1".to_string(),
            v1.to_string(),
            container.clone(),
            &SHARED_TEST_BLOCKING_HTTP_CLIENT,
        );

        let final_filter = provider1
            .get_filter_contents("https://example.org/lists/list1.txt")
            .unwrap();

        assert_eq!(v2, final_filter)
    }

    #[test]
    fn test_diff_path_remove_newlines() {
        let v1 = "
! Title: Batch-Updatable List 1
! Diff-Path: ../patches/batch_v1.0.1-s-1700049442-3600.patch#list1



||example.com^
";
        let v2 = "! Title: Batch-Updatable List 1
! Diff-Path: ../patches/batch_v1.0.0-s-1700045842-3600.patch#list1
||example.org^";

        let patch = "d1 1
d3 6
a8 2
! Diff-Path: ../patches/batch_v1.0.0-s-1700045842-3600.patch#list1
||example.org^";

        let container = BatchPatchesContainer::factory();
        container.borrow_mut().insert(
            "https://example.org/patches/batch_v1.0.0-s-1700045842-3600.patch".to_string(),
            patch.to_string(),
        );

        let provider1 = DiffPathProvider::new(
            "../patches/batch_v1.0.0-s-1700045842-3600.patch#list1".to_string(),
            v1.to_string(),
            container.clone(),
            &SHARED_TEST_BLOCKING_HTTP_CLIENT,
        );

        let final_filter = provider1
            .get_filter_contents("https://example.org/lists/list1.txt")
            .unwrap();

        assert_eq!(v2, final_filter)
    }

    #[test]
    fn test_validation_without_newline() {
        let base_filter_contents = include_str!(
            "../../../../tests/fixtures/diffupdates/examples/02_validation/filter_v1.0.0.txt"
        );
        let expected_filter = include_str!(
            "../../../../tests/fixtures/diffupdates/examples/02_validation/filter_v1.0.1.txt"
        );
        let base_filter_path =
            tests_path("fixtures/diffupdates/examples/02_validation/filter_v1.0.0.txt");

        let provider1 = DiffPathProvider::new(
            "patches/v1.0.0-m-28334060-60.patch".to_string(),
            String::from(base_filter_contents),
            BatchPatchesContainer::factory(),
            &SHARED_TEST_BLOCKING_HTTP_CLIENT,
        );

        let base_filter_url = Url::from_file_path(base_filter_path).unwrap();

        let actual_filter = provider1
            .get_filter_contents(base_filter_url.to_string().as_str())
            .unwrap();

        assert_eq!(actual_filter, expected_filter);
    }

    #[test]
    fn test_with_checksum() {
        let base_filter_contents = include_str!(
            "../../../../tests/fixtures/diffupdates/examples/04_checksum/filter_v1.0.0.txt"
        );
        let expected_filter =
            include_str!("../../../../tests/fixtures/diffupdates/examples/04_checksum/filter.txt");
        let base_filter_path =
            tests_path("fixtures/diffupdates/examples/04_checksum/filter_v1.0.0.txt");

        let provider = DiffPathProvider::new(
            "patches/v1.0.0-472234-1.patch".to_string(),
            String::from(base_filter_contents),
            BatchPatchesContainer::factory(),
            &SHARED_TEST_BLOCKING_HTTP_CLIENT,
        );

        let base_filter_url = Url::from_file_path(base_filter_path).unwrap();
        let patched_contents = provider
            .get_filter_contents(base_filter_url.to_string().as_str())
            .unwrap();

        validate_checksum(patched_contents.as_str()).unwrap();

        assert_eq!(patched_contents, expected_filter);
    }
}
