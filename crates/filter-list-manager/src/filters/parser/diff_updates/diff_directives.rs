use crate::utils::iterators::lines_with_terminator::lines_with_terminator;
use crate::FilterParserError;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, space1},
    combinator::{map_res, opt},
    multi::many1,
    sequence::{pair, preceded, terminated, tuple},
    Err::Error,
    IResult,
};

/// Max length of directive "name"
///
/// See <https://github.com/ameshkov/diffupdates?tab=readme-ov-file#changes-to-filter-lists-metadata> for details
const MAX_DIRECTIVE_NAME_LENGTH: usize = 64;

const DIFF_DIRECTIVE: &str = "diff";
const DIFF_DIRECTIVE_NAME: &str = "name";
const DIFF_DIRECTIVE_CHECKSUM: &str = "checksum";
const DIFF_DIRECTIVE_LINE: &str = "lines";

/// Successfully parsed diff directive data: (name, checksum, lines)
///
/// See [`recognize_diff_directive`] for details
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct RecognizedDiffDirective<'a> {
    /// Resource name
    pub(crate) name: Option<String>,
    /// Compiled patch checksum
    pub(crate) checksum: &'a str,
    /// Line count
    pub(crate) lines: usize,
}

/// Tries to extract current patch from patch file.
///
/// Execution variants:
/// - Patch file doesn't have diff_directive: We just return all file contents
/// - Patch file has directive without `name`. Here we will return file contents without directive + directive as [`RecognizedDiffDirective`]
/// - Patch file has directive with `name`. Thus, the file may be batch file, and we must find a chunk for resource `resource_name_option`
///
/// Returns [`Option<RecognizedDiffDirective>`] if it in file, lines of concrete diff as [Vec<&str>] and an indicator that the end of this diff is the end of the file
pub(crate) fn extract_patch(
    patch_str: &str,
    resource_name_option: Option<String>,
) -> Result<(Option<RecognizedDiffDirective>, Vec<&str>, bool), FilterParserError> {
    let mut diff_lines_raw = lines_with_terminator(patch_str).peekable();
    let mut recognized_directive: RecognizedDiffDirective;

    // Early return for patches without diff directive
    match diff_lines_raw.peek() {
        None => {
            return FilterParserError::other_err_from_to_string(
                "Cannot preprocess patch, because it looks like empty",
            )
        }
        Some(first_line) => {
            match recognize_diff_directive(first_line) {
                Ok((_, diff_recognition_result)) => {
                    let Some(recognized) = diff_recognition_result else {
                        // First string is not diff directive.
                        // Looks like this file does not contain them,
                        // so we should return all patch
                        return Ok((None, diff_lines_raw.collect(), true));
                    };

                    recognized_directive = recognized;
                }
                Err(e) => return FilterParserError::other_err_from_to_string(e),
            }
        }
    }

    // Early return for:
    // Directive may be in file, but not have name.
    // So this is not batch, we just return remaining contents of file.
    if recognized_directive.name.is_none() {
        // Skip directive line
        diff_lines_raw.next();

        return Ok((Some(recognized_directive), diff_lines_raw.collect(), true));
    }

    // Resource name must be defined here
    // because we need to match it against directives in batch file
    let Some(resource_name) = resource_name_option else {
        return FilterParserError::other_err_from_to_string(
            "Patch have found, but resource name is not given",
        );
    };

    let mut out: Vec<&str> = vec![];
    let mut lower_bound_found = false;
    let mut upper_bound_found = false;

    // If ‘break’ is encountered, it means that we had to interrupt the recording of the patch piece,
    // it means that this patch piece is not the last one in the file.
    // Otherwise, we would simply exit the loop when it is finished
    let mut break_encountered = false;

    for line in diff_lines_raw {
        if !lower_bound_found {
            if let Ok((_, Some(directive_found))) = recognize_diff_directive(line) {
                match &directive_found.name {
                    Some(current_name) => {
                        // Found needed directive, then start consuming
                        if current_name == &resource_name {
                            recognized_directive = directive_found;

                            lower_bound_found = true;
                        }
                    }
                    None => {
                        // We've found diff directive, but it doesn't contain name
                        // So patch is broken
                        return FilterParserError::other_err_from_to_string(
                            r"We've found diff directive, but it doesn't contain name.
So patch is broken",
                        );
                    }
                }
            }
        } else if !upper_bound_found {
            match recognize_diff_directive(line) {
                Ok((_, Some(_))) => {
                    upper_bound_found = true;
                }
                _ => out.push(line),
            }
        } else {
            break_encountered = true;
            break;
        }
    }

    Ok((Some(recognized_directive), out, !break_encountered))
}

/// Recognizes diff directive
/// See more: <https://github.com/ameshkov/diffupdates?tab=readme-ov-file#diff-files-format>
pub(crate) fn recognize_diff_directive(
    line: &str,
) -> IResult<&str, Option<RecognizedDiffDirective>, nom::error::Error<&str>> {
    // Do not throw if not match
    opt(map_res(
        preceded(
            // Do not catch "diff "
            pair(tag(DIFF_DIRECTIVE), space1),
            tuple((
                // Name may be optional
                opt(
                    // From pair "name:some_name " we will catch only "some_name"
                    // https://github.com/ameshkov/diffupdates?tab=readme-ov-file#resource-name
                    terminated(
                        preceded(
                            pair(tag(DIFF_DIRECTIVE_NAME), tag(":")),
                            many1(alt((alphanumeric1, tag("-"), tag("_")))),
                        ),
                        space1,
                    ),
                ),
                // Checksum is mandatory.
                // Will be caught only checksum itself, like in previous pair
                terminated(
                    preceded(pair(tag(DIFF_DIRECTIVE_CHECKSUM), tag(":")), alphanumeric1),
                    space1,
                ),
                // Lines is mandatory.
                preceded(pair(tag(DIFF_DIRECTIVE_LINE), tag(":")), digit1),
            )),
        ),
        map_diff_directive,
    ))(line)
}

/// Maps successful parser result into [`RecognizedDiffDirective`]
fn map_diff_directive<'a>(
    (name, checksum, lines): (Option<Vec<&'a str>>, &'a str, &'a str),
) -> Result<RecognizedDiffDirective, nom::Err<&'a str>> {
    let lines = str::parse::<i32>(lines).map_err(|_| Error(""))?;

    let final_name = name.map(|list| list.join(""));

    for value in final_name.iter() {
        if value.len() > MAX_DIRECTIVE_NAME_LENGTH {
            return Err(Error(
                "The length of the \"name\" field must not exceed 64 characters",
            ));
        }
    }

    Ok(RecognizedDiffDirective {
        name: final_name,
        checksum,
        lines: lines as usize,
    })
}

#[cfg(test)]
mod tests {
    use super::{extract_patch, recognize_diff_directive, RecognizedDiffDirective};

    #[test]
    fn test_recognize_diff_directive() {
        [
            (
                "diff checksum:1ce52b527d56a245f32138e014b1571c19cfb659 lines:4",
                Some((None, "1ce52b527d56a245f32138e014b1571c19cfb659", 4))
            ),
            (
                "diff name:list-1_ checksum:1ce52b527d56a245f32138e014b1571c19cfb659 lines:7",
                Some((Some(String::from("list-1_")), "1ce52b527d56a245f32138e014b1571c19cfb659", 7))
            ),
            (
                "diff",
                None
            ),
            (
                "diff name: checksum:1ce52b527d56a245f32138e014b1571c19cfb659 lines:7",
                None,
            ),
            (
                "diff name:12345678901234567890123456789012345678901234567890123456789012345 checksum:1ce52b527d56a245f32138e014b1571c19cfb659 lines:7",
                None
            ),
        ].into_iter().for_each(|(invariant, expected)| {
            let (_, result) = recognize_diff_directive(invariant)
                .unwrap();

            match result {
                None => assert!(expected.is_none()),
                Some(directive) => {
                    let expected_data = expected.unwrap();

                    assert_eq!(directive.name, expected_data.0);
                    assert_eq!(directive.checksum, expected_data.1);
                    assert_eq!(directive.lines, expected_data.2);
                }
            }
        })
    }

    #[test]
    fn test_extract_patch() {
        const SIMPLE_PATCH: &str = r"d1 2
d4 1
a4 2
The named is the mother of all things.

a11 3
They both may be called deep and profound.
Deeper and more profound,
The door of all subtleties!";

        let (directive_list_2, strings_list_2, _) = extract_patch(SIMPLE_PATCH, None).unwrap();

        let joined = strings_list_2.join("\n");

        assert_eq!(None, directive_list_2);
        assert_eq!(joined, SIMPLE_PATCH.to_string());
    }

    #[test]
    fn test_extract_batch_patch() {
        const BATCH_PATCH: &str = r"diff name:list1 checksum:f0ecb30059277cbae9736e2bf4fcdfa4a7cac751 lines:4
d2 2
a3 2
! Diff-Path: ../patches/batch_v1.0.1-s-1700049442-3600.patch#list1
||example.com^
diff name:list2 checksum:9db9474484edf99f9112d3654a00d1a0d20e92eb lines:4
d2 2
a3 2
! Diff-Path: ../patches/batch_v1.0.1-s-1700049442-3600.patch#list2
||test.com^
diff name:list3 checksum:7db9414414edf99f1112d3654a00d2a0d20e92ea lines:5
d1 2
a3 2
! Diff-Path: ../patches/batch_v1.0.1-s-1700049442-3600.patch#list3
d1 5
||adguard.com^";

        {
            let (directive_list_2, strings_list_2, _) =
                extract_patch(BATCH_PATCH, Some(String::from("list2"))).unwrap();

            let actual = directive_list_2.unwrap();
            assert_eq!(
                actual,
                RecognizedDiffDirective {
                    name: Some(String::from("list2")),
                    checksum: "9db9474484edf99f9112d3654a00d1a0d20e92eb",
                    lines: 4,
                }
            );

            assert!(strings_list_2
                .into_iter()
                .find(|s| *s == "||test.com^")
                .is_some());
        }

        {
            let (directive_list_1, strings_list_1, _) =
                extract_patch(BATCH_PATCH, Some(String::from("list1"))).unwrap();

            let actual = directive_list_1.unwrap();
            assert_eq!(
                actual,
                RecognizedDiffDirective {
                    name: Some(String::from("list1")),
                    checksum: "f0ecb30059277cbae9736e2bf4fcdfa4a7cac751",
                    lines: 4,
                }
            );

            assert!(strings_list_1
                .into_iter()
                .find(|s| *s == "||example.com^")
                .is_some());
        }

        {
            let (directive_list_3, strings_list_3, _) =
                extract_patch(BATCH_PATCH, Some(String::from("list3"))).unwrap();

            let actual = directive_list_3.unwrap();
            assert_eq!(
                actual,
                RecognizedDiffDirective {
                    name: Some(String::from("list3")),
                    checksum: "7db9414414edf99f1112d3654a00d2a0d20e92ea",
                    lines: 5,
                }
            );

            assert!(strings_list_3
                .into_iter()
                .find(|s| *s == "||adguard.com^")
                .is_some());
        }
    }
}
