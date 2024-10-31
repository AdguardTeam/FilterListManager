use nom::bytes::complete::take_while;
use nom::IResult;

/// Line feed 0xA character
pub(crate) const LF_BYTES_SLICE: u8 = b'\n';

/// Is space allowed for parsing
pub(crate) fn is_allowed_space(c: char) -> bool {
    match c {
        '\n' | '\r' | '\t' | ' ' => true,
        _ => false,
    }
}

/// Go to first non-space symbol
pub(crate) fn take_spaces(input: &str) -> IResult<&str, &str, nom::error::Error<&str>> {
    take_while(is_allowed_space)(input)
}

#[cfg(test)]
mod tests {
    use super::take_spaces;

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
}
