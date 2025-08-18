use nom::bytes::complete::take_while;
use nom::IResult;

/// Line feed 0xA character
pub(crate) const LF_BYTES_SLICE: u8 = b'\n';

/// Carriage return 0xD character
pub(crate) const CR_BYTES_SLICE: u8 = b'\r';

/// Is space allowed for parsing
pub(crate) fn is_allowed_space(c: char) -> bool {
    matches!(c, '\n' | '\r' | '\t' | ' ')
}

/// Go to first non-space symbol
pub(crate) fn take_spaces(input: &str) -> IResult<&str, &str, nom::error::Error<&str>> {
    take_while(is_allowed_space)(input)
}

/// Collapse all newlines in input
pub(crate) fn collapse_newlines(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut last_sym_was_newline = false;

    // We can get maximum perf here by using bytes instead of chars,
    // because all continuation bytes have code >= 0x80
    let mut iter = input.bytes().peekable();
    while let Some(byte) = iter.next() {
        match byte {
            LF_BYTES_SLICE => {
                if !last_sym_was_newline {
                    out.push(LF_BYTES_SLICE as char);
                }
                last_sym_was_newline = true;
            }
            CR_BYTES_SLICE => {
                // Skip LF too
                if iter.peek() == Some(&LF_BYTES_SLICE) {
                    iter.next();
                }

                if !last_sym_was_newline {
                    out.push(LF_BYTES_SLICE as char);
                }
                last_sym_was_newline = true;
            }
            other => {
                out.push(other as char);
                last_sym_was_newline = false;
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::take_spaces;
    use crate::utils::parsing::collapse_newlines;

    #[test]
    fn test_next_token_after_many_spaces() {
        let (token, _) = take_spaces("   test").unwrap();

        assert_eq!(token, "test")
    }

    #[test]
    fn test_next_token_if_there_are_no_spaces() {
        let (token, _) = take_spaces("test").unwrap();

        assert_eq!(token, "test")
    }

    #[test]
    fn test_next_token_if_eof_encountered() {
        let (token, _) = take_spaces("").unwrap();

        assert_eq!(token, "")
    }

    #[test]
    fn test_next_token_encountered_return_spaces() {
        let (_, spaces) = take_spaces("\r\t\n ").unwrap();

        assert_eq!(spaces.len(), 4)
    }

    #[test]
    fn test_take_spaces() {
        let (remainder, spaces) = take_spaces(" \t\n\rfewfe fww").unwrap();

        assert_eq!(spaces.len(), 4);
        assert_eq!(remainder.len(), 9);
    }

    #[test]
    fn test_collapse_newlines() {
        [
            ("line1\r\n\r\nline2\n\n\nline3\r\n", "line1\nline2\nline3\n"),
            ("", ""),
            ("\r", "\n"),
            ("\r\n", "\n"),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            let actual = collapse_newlines(input);
            assert_eq!(actual, expected);
        })
    }
}
