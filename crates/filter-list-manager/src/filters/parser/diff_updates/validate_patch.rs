use super::diff_directives::RecognizedDiffDirective;
use crate::FilterParserError;
use nom::AsBytes;
use sha1::{Digest, Sha1};

/// Validates applied patch
///
/// * `recognize_diff_directive` - Diff directive container
/// * `patch_result_lines_count` - lines.len() of filter contents after patch applying
/// * `patch_result` - the filter contents after patch applying
pub(crate) fn validate_patch(
    recognize_diff_directive: RecognizedDiffDirective,
    patch_result_lines_count: usize,
    patch_result: &str,
) -> Result<(), FilterParserError> {
    // Actually lines is *line feeds*, not lines
    if patch_result_lines_count - 1 != recognize_diff_directive.lines {
        return FilterParserError::other_err_from_to_string(
            "The number of lines in the patch differs from the number in the patch header",
        );
    }

    let digest = Sha1::digest(patch_result);
    if digest.as_bytes() == recognize_diff_directive.checksum.as_bytes() {
        return FilterParserError::invalid_checksum(
            format!("{:x?}", digest.as_bytes()),
            recognize_diff_directive.checksum.to_string(),
        );
    }

    Ok(())
}
