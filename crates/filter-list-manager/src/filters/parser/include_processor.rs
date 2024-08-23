use super::DIRECTIVE_INCLUDE;
use crate::utils::parsing::take_spaces;
use crate::FilterParserError;
use nom::Slice;

/// Parses `!#include some://url` String
pub(super) fn get_include_path(line: &str) -> Result<Option<&str>, FilterParserError> {
    let remainder = line.slice(DIRECTIVE_INCLUDE.len()..);
    let (remainder, spaces) =
        take_spaces(remainder).map_err(|why| FilterParserError::Other(why.to_string()))?;

    if spaces.is_empty() || remainder.is_empty() {
        return Ok(None);
    }

    Ok(Some(remainder.trim_end()))
}

#[cfg(test)]
mod tests {
    use super::get_include_path;

    #[test]
    fn test_get_include_path() {
        [
            ("!#include ", None),
            ("!#includef", None),
            ("!#include f ", Some("f")),
        ]
        .into_iter()
        .for_each(|(line, expected)| {
            let actual = get_include_path(line).unwrap();

            assert_eq!(actual, expected);
        })
    }
}
