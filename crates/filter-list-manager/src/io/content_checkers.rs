use crate::FilterParserError;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::combinator::opt;
use nom::sequence::preceded;
use nom::IResult;

/// Search pattern for removing UTF-8 BOM
const BOM_SEARCH_PATTERN: &str = "\u{feff}";

/// Checks the input to see if it is possibly part of an (x)html document
fn is_likely_x_html(input: &str) -> IResult<&str, Option<&str>, nom::error::Error<&str>> {
    opt(preceded(
        tag("<"),
        alt((
            tag_no_case("!doctype"),
            tag_no_case("?xml"),
            tag_no_case("html"),
            tag_no_case("head"),
            tag_no_case("body"),
            tag_no_case("script"),
            tag_no_case("div"),
            tag_no_case("table"),
            tag_no_case("meta"),
            tag("!--"), // Comment
        )),
    ))(input)
}

#[inline]
/// Tries to discard an obviously unsuitable body for the filter
pub(super) fn check_contents_is_filter_contents(contents: &str) -> Result<(), FilterParserError> {
    let trimmed = contents.trim_start_matches(BOM_SEARCH_PATTERN).trim_start();

    let (_, result) = is_likely_x_html(trimmed).map_err(FilterParserError::other_from_to_string)?;

    if result.is_some() {
        return Err(FilterParserError::FilterContentIsLikelyNotAFilter);
    }

    Ok(())
}

/// Checks the binary input to see if it is likely a media/document file
/// supported by our quick sniffing: JPEG, PNG, GIF, or PDF.
///
/// The detection is based on well-known magic numbers at the beginning of the file:
/// - JPEG: FF D8 FF
/// - PNG:  89 50 4E 47 0D 0A 1A 0A
/// - GIF:  "GIF87a" or "GIF89a"
/// - PDF:  "%PDF-"
///
/// Returns true if any of the signatures match, false otherwise.
pub(crate) fn is_likely_media(input: &[u8]) -> bool {
    // Use nom's tag + alt to match any of the known signatures at the start.
    // Define a small parser to help type inference (IResult fixes the error type).
    fn media_signature(i: &[u8]) -> IResult<&[u8], &[u8]> {
        alt((
            // JPEG
            tag(b"\xFF\xD8\xFF"),
            // PNG
            tag(b"\x89PNG\r\n\x1a\n"),
            // GIF variants
            tag(b"GIF87a"),
            tag(b"GIF89a"),
            // PDF
            tag(b"%PDF-"),
        ))(i)
    }

    media_signature(input).is_ok()
}

#[cfg(test)]
mod tests {
    use super::check_contents_is_filter_contents;
    use crate::FilterParserError;

    #[test]
    fn test_check_filter_contents() {
        let bom_doctype = String::from_utf8(b"\xEF\xBB\xBF<!DOCTYPE".to_vec()).unwrap();

        let bom_doctype_twice =
            String::from_utf8(b"\xEF\xBB\xBF\xEF\xBB\xBF <!DOCTYPE".to_vec()).unwrap();

        [
            ("", Ok(())),
            (
                bom_doctype.as_str(),
                Err(FilterParserError::FilterContentIsLikelyNotAFilter),
            ),
            (
                bom_doctype_twice.as_str(),
                Err(FilterParserError::FilterContentIsLikelyNotAFilter),
            ),
            (
                " \t    \t <!DOCTYPE",
                Err(FilterParserError::FilterContentIsLikelyNotAFilter),
            ),
            (
                " \t    \t <?xml",
                Err(FilterParserError::FilterContentIsLikelyNotAFilter),
            ),
            (
                "<?xml-",
                Err(FilterParserError::FilterContentIsLikelyNotAFilter),
            ),
            (
                " \t    \t <html",
                Err(FilterParserError::FilterContentIsLikelyNotAFilter),
            ),
            (
                "\n \t  \n  \t <body",
                Err(FilterParserError::FilterContentIsLikelyNotAFilter),
            ),
            (
                "\n \t  \n  \t <div",
                Err(FilterParserError::FilterContentIsLikelyNotAFilter),
            ),
            (
                "\n \t  \n  \t <script",
                Err(FilterParserError::FilterContentIsLikelyNotAFilter),
            ),
            (
                "\n \t  \n  \t <meta",
                Err(FilterParserError::FilterContentIsLikelyNotAFilter),
            ),
            (
                "\n \t  \n  \t <table",
                Err(FilterParserError::FilterContentIsLikelyNotAFilter),
            ),
            (
                "<!-- Hello -->",
                Err(FilterParserError::FilterContentIsLikelyNotAFilter),
            ),
            ("! Checksum: abcdef", Ok(())),
        ]
        .into_iter()
        .for_each(|(input, expected_result)| {
            let actual = check_contents_is_filter_contents(input);

            assert_eq!(actual, expected_result);
        });
    }

    #[test]
    fn test_is_likely_media() {
        use super::is_likely_media;

        // JPEG
        assert!(is_likely_media(b"\xFF\xD8\xFF\xE0rest"));
        // PNG
        assert!(is_likely_media(b"\x89PNG\r\n\x1a\nrest"));
        // GIF 87a and 89a
        assert!(is_likely_media(b"GIF87a........"));
        assert!(is_likely_media(b"GIF89a........"));
        // PDF
        assert!(is_likely_media(b"%PDF-1.7\n%..."));

        // Not media
        assert!(!is_likely_media(b"Not a media file"));
        assert!(!is_likely_media(b"<!DOCTYPE html>"));
    }
}
