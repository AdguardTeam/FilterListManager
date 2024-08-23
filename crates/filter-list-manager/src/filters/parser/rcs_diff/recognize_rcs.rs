use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::{map_res, opt};
use nom::sequence::{pair, separated_pair};
use nom::Err::Error;
use nom::IResult;

/// Possible RCS diff operations
#[cfg_attr(test, derive(Debug))]
#[derive(PartialEq, Eq)]
pub(super) enum RCSOperations {
    Add,
    Delete,
}

/// Tuple contains recognized RCS result
pub(super) type RecognizedRCS = (RCSOperations, usize, usize);

/// Tries to recognize rcs format and preprocess string into a [`RecognizedRCS`]
pub(super) fn recognize_rcs(
    input: &str,
) -> IResult<&str, Option<RecognizedRCS>, nom::error::Error<&str>> {
    opt(map_res(
        separated_pair(pair(alt((char('a'), char('d'))), digit1), tag(" "), digit1),
        map_rcs_result,
    ))(input)
}

/// Flatten and process rcs result
fn map_rcs_result<'a>(
    ((raw_operation, line), count): ((char, &'a str), &'a str),
) -> Result<RecognizedRCS, nom::Err<&'a str>> {
    let operation = match raw_operation {
        'a' => RCSOperations::Add,
        'd' => RCSOperations::Delete,
        // Count this as non-rcs string, and return an empty error, which will be mapped to None (unrecognized rcs)
        _ => return Err(Error("")),
    };

    let line = str::parse::<i32>(line).map_err(|_| Error(""))?;

    let count = str::parse::<i32>(count).map_err(|_| Error(""))?;

    Ok((operation, line as usize, count as usize))
}

#[cfg(test)]
mod tests {
    use super::{recognize_rcs, RCSOperations};

    #[test]
    fn test_recognize_rcs() {
        let invariants = [
            ("d1 2", Some((RCSOperations::Delete, 1, 2))),
            ("a11 3", Some((RCSOperations::Add, 11, 3))),
            ("b11 3", None), // Unrecognized
            ("a3423432 34324", Some((RCSOperations::Add, 3423432, 34324))),
            ("aa", None),
            ("a-11 3", None),
            ("a11 -3", None),
            ("! Version", None),
        ];

        invariants.iter().for_each(|(invariant, expected)| {
            let actual = recognize_rcs(invariant).unwrap();

            assert_eq!(&actual.1, expected);
        })
    }
}
