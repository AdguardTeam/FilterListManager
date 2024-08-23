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

/// Tries to find checksum, then tries to validate if success
pub(super) fn validate_checksum(contents: &str) -> Result<bool, FilterParserError> {
    let mut new_str = String::new();
    let mut checksum: &str = "";
    let mut is_checksum_found: bool = false;

    for (index, line) in contents.lines().enumerate() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        if !is_checksum_found {
            // Do not search checksum anymore
            if index > HOW_MUCH_FAR_CHECKSUM_MIGHT_BE {
                return Ok(false);
            }

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

        new_str += trimmed;
        new_str.push('\n');
    }

    new_str.pop();

    if checksum.is_empty() {
        return Ok(false);
    }

    let digest = md5::compute(new_str);
    let encoded_digest = BASE64_STANDARD_NO_PAD.encode(digest.as_bytes());

    if encoded_digest.as_str() != checksum {
        return FilterParserError::invalid_checksum(encoded_digest, checksum.to_string());
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
}
