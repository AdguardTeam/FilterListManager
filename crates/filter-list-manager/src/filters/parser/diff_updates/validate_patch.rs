use super::diff_directives::RecognizedDiffDirective;
use crate::FilterParserError;
use faster_hex::hex_decode;
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
    if patch_result_lines_count != recognize_diff_directive.lines {
        return FilterParserError::other_err_from_to_string(
            format!(
                "The number of lines in the patch ({}) differs from the number in the patch header ({})",
                patch_result_lines_count,
                recognize_diff_directive.lines
            )
        );
    }

    let mut digest = Sha1::new();
    digest.update(patch_result.as_bytes());
    let result = digest.finalize();

    let mut checksum_hex: [u8; 20] = [0; 20];

    hex_decode(
        recognize_diff_directive.checksum.as_bytes(),
        &mut checksum_hex,
    )
    .or_else(|why| FilterParserError::other_err_from_to_string(why.to_string()))?;

    if result.as_bytes() != checksum_hex {
        return FilterParserError::invalid_checksum(
            format!("{:x?}", result.as_bytes()),
            recognize_diff_directive.checksum.to_string(),
        );
    }

    Ok(())
}
