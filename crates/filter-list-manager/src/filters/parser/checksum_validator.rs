use crate::utils::iterators::lines_with_terminator::lines_with_terminator;
use crate::FilterParserError;
use base64::prelude::BASE64_STANDARD_NO_PAD;
use base64::Engine;
use nom::bytes::complete::{tag_no_case, take_while};
use nom::bytes::streaming::tag;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::sequence::{pair, preceded, separated_pair};
use nom::{AsBytes, IResult};

/// How much lines we must scan to possible get Checksum
const HOW_MUCH_FAR_CHECKSUM_MIGHT_BE: usize = 50;

/// Check this char is in base64 chars set
fn is_base64_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '='
}

/// Gets checksum value from `! Checksum: <checksum>`-like string
fn parse_checksum(input: &str) -> IResult<&str, Option<&str>, nom::error::Error<&str>> {
    opt(preceded(
        pair(
            separated_pair(tag("!"), space0, tag_no_case("Checksum:")),
            space0,
        ),
        take_while(is_base64_char),
    ))(input)
}

/// Calculates checksum from contents
fn calculate_checksum(contents: &str) -> String {
    let digest = md5::compute(contents);
    BASE64_STANDARD_NO_PAD.encode(digest.as_bytes())
}

/// Tries to find checksum, then tries to validate if success
pub(super) fn validate_checksum(contents: &str) -> Result<bool, FilterParserError> {
    let mut new_str = String::new();
    let mut checksum: &str = "";
    let mut is_checksum_found: bool = false;

    for (index, line) in lines_with_terminator(contents).enumerate() {
        let trimmed = line.trim();

        if !is_checksum_found {
            // Do not search checksum anymore
            if index > HOW_MUCH_FAR_CHECKSUM_MIGHT_BE {
                return Ok(false);
            }

            if !trimmed.is_empty() {
                match parse_checksum(trimmed) {
                    Ok((_, Some(result))) => {
                        checksum = result;
                        is_checksum_found = true;

                        continue;
                    }
                    Err(err) => return FilterParserError::other_err_from_to_string(err),
                    _ => {}
                }
            }
        }

        new_str += trimmed;
        new_str.push('\n');
    }

    // Pop the last line
    new_str.pop();

    if checksum.is_empty() {
        return Ok(false);
    }

    // Try compiled string
    let original_digest = calculate_checksum(&new_str);
    if original_digest.as_str() != checksum {
        // Try pop once, if there is newline
        if new_str.ends_with('\n') {
            new_str.pop();

            let digest_with_popped_line = calculate_checksum(&new_str);
            if digest_with_popped_line.as_str() == checksum {
                return Ok(true);
            } else {
                // If checksum is not valid, add back the newline
                new_str.push('\n');
            }
        }

        // Try with extra newline
        new_str.push('\n');
        let digest_with_newline = calculate_checksum(&new_str);

        if digest_with_newline.as_str() != checksum {
            // Throw an original checksum
            return FilterParserError::invalid_checksum(original_digest, checksum.to_string());
        }
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::{parse_checksum, validate_checksum};

    #[test]
    fn test_resolve_checksum() {
        [
            (
                "!checksum:z0Dim2pMXWoAwkhAa4Yk8g",
                Some("z0Dim2pMXWoAwkhAa4Yk8g"),
            ),
            (
                "! Checksum: z0Dim2pMXWoAwkhAa4Yk8g",
                Some("z0Dim2pMXWoAwkhAa4Yk8g"),
            ),
            ("! Diff-Path: ../patches/1/1-s-1722440227-3600.patch", None),
        ]
        .into_iter()
        .for_each(|(line, expected)| {
            let (_, actual) = parse_checksum(line).unwrap();

            assert_eq!(actual, expected)
        });
    }

    #[test]
    fn test_validate_checksum() {
        // In this file extra empty lines was added, because checksum must ignore "\r" and "\n"
        // Also, checksum string moved to 40-th line
        let filter = include_str!("../../../tests/fixtures/test_checksum.txt");

        let found_and_validate = validate_checksum(filter).unwrap();

        assert!(found_and_validate)
    }

    #[test]
    fn test_validate_checksum_w_newline_on_end() {
        let filter = include_str!("../../../tests/fixtures/test-checksum-nl.txt");

        let found_and_validate = validate_checksum(filter).unwrap();

        assert!(found_and_validate)
    }
}
