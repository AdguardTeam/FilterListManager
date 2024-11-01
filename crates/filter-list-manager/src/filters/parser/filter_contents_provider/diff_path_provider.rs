use crate::filters::parser::diff_updates::validate_patch::validate_patch;
use crate::filters::parser::{
    diff_updates::batch_patches_container::BatchPatchesContainer,
    diff_updates::diff_directives::extract_patch, filter_contents_provider::FilterContentsProvider,
    paths::resolve_absolute_uri, rcs_diff::apply_patch,
};
use crate::io::fetch_by_schemes::fetch_by_scheme;
use crate::io::url_schemes::UrlSchemes;
use crate::io::{get_hash_from_url, get_scheme};
use crate::FilterParserError;
use std::cell::RefCell;
use std::rc::Rc;

/// This provider is used to download and process incremental filter updates
pub(crate) struct DiffPathProvider {
    /// Request timeout, in milliseconds
    request_timeout: Option<i32>,
    /// Relative path from Diff-Path
    patch_relative_path: String,
    /// Contents of saved filter
    base_filter_contents: String,
    /// Container for shared patch files
    batch_patches_container: Rc<RefCell<BatchPatchesContainer>>,
}

impl DiffPathProvider {
    pub(crate) fn new(
        patch_relative_path: String,
        base_filter_contents: String,
        batch_patches_container: Rc<RefCell<BatchPatchesContainer>>,
    ) -> Self {
        Self {
            patch_relative_path,
            request_timeout: None,
            base_filter_contents,
            batch_patches_container,
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

        let (diff_directive_option, prepared_patch) =
            extract_patch(patch_file_contents, resource_name)?;
        let patch_lines_count = prepared_patch.len();

        match apply_patch(self.base_filter_contents.as_str(), prepared_patch) {
            Ok(patch_result) if patch_result.is_empty() => {
                FilterParserError::other_err_from_to_string("Patch result is empty")
            }
            Ok(patch_result) => {
                if let Some(diff_directive) = diff_directive_option {
                    validate_patch(diff_directive, patch_lines_count, patch_result.as_str())?;
                }

                return Ok(patch_result);
            }
            Err(e) => return Err(e),
        }
    }
}

impl FilterContentsProvider for DiffPathProvider {
    fn get_filter_contents(&self, root_filter_url: &str) -> Result<String, FilterParserError> {
        let scheme = UrlSchemes::from(get_scheme(root_filter_url));

        let patch_file_absolute_uri =
            resolve_absolute_uri(scheme, root_filter_url, self.patch_relative_path.as_str())?;

        // If resource name exist we assume that this is batch patch file
        // According to <https://github.com/ameshkov/diffupdates/tree/master?tab=readme-ov-file#algorithm>
        // we must load patch diff file only once
        let (resource_name, file_contents) = match get_hash_from_url(&patch_file_absolute_uri) {
            Some((patch_path, resource_name)) => {
                let mut pinned_container = self.batch_patches_container.borrow_mut();

                let diff_contents = match pinned_container.get_a_copy(&patch_path) {
                    None => {
                        let body =
                            fetch_by_scheme(&patch_path, scheme, self.get_request_timeout())?;

                        pinned_container.insert(patch_path, body.clone());

                        body
                    }
                    Some(string) => string,
                };

                (Some(resource_name), diff_contents)
            }
            None => {
                let diff_contents =
                    fetch_by_scheme(&patch_file_absolute_uri, scheme, self.get_request_timeout())?;

                (None, diff_contents)
            }
        };

        self.do_patch(file_contents.as_str(), resource_name)
    }

    fn get_request_timeout(&self) -> i32 {
        self.request_timeout.clone().unwrap_or_default()
    }

    fn set_request_timeout_once(&mut self, request_timeout: i32) {
        if self.request_timeout.is_none() {
            self.request_timeout = Some(request_timeout);
        }
    }
}
