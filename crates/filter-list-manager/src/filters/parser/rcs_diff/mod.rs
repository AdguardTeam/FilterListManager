mod recognize_rcs;

use self::recognize_rcs::{recognize_rcs, RCSOperations};
use crate::filters::parser::metadata::collector::MetadataCollector;
use crate::filters::parser::metadata::KnownMetadataProperty;
use crate::utils::iterators::lines_with_terminator::lines_with_terminator;
use crate::FilterParserError;

/// Applies `diff` to `base_filter`
///
/// # Returns
///
/// - `Ok((patched_filter, next_diff_path))`
/// - `Err(FilterParserError)`
pub(crate) fn apply_patch(
    base_filter: &str,
    diff_lines: Vec<&str>,
) -> Result<(String, Option<String>), FilterParserError> {
    let mut diff_iter = diff_lines.iter().enumerate();
    let base_filter_lines: Vec<&str> = lines_with_terminator(base_filter).collect();

    let mut slices: Vec<&[&str]> = vec![];
    let mut base_filter_cursor = 0usize;
    let mut next_diff_path: Option<String> = None;

    let mut last_command_is_add = false;
    let last_base_filter_line_is_line_feed = base_filter_lines
        .last()
        .map(|line| line.is_empty())
        .unwrap_or(false);
    let last_diff_line_is_line_feed = diff_lines
        .last()
        .map(|line| line.is_empty())
        .unwrap_or(false);

    #[allow(clippy::while_let_on_iterator)]
    while let Some((index, line)) = diff_iter.next() {
        match recognize_rcs(line) {
            Ok((_, None)) => {
                // Just a regular line.
                if next_diff_path.is_none() {
                    next_diff_path =
                        MetadataCollector::parse_line_for(KnownMetadataProperty::DiffPath, line)
                }
            }
            Ok((_, Some((operation, line, count)))) => {
                // Recognized rcs
                match operation {
                    RCSOperations::Add => {
                        last_command_is_add = true;

                        // If we have a gap between current base_filter_cursor and current diff line
                        // we should write all lines in this gap first
                        if base_filter_cursor + 1 < line {
                            // Wrong diff encountered
                            if base_filter_lines.len() < line {
                                return make_line_out_of_bounds_error(
                                    line,
                                    base_filter_lines.as_slice(),
                                    diff_lines.as_slice(),
                                );
                            }

                            slices.push(
                                base_filter_lines[base_filter_cursor..line]
                                    .iter()
                                    .as_slice(),
                            );
                            base_filter_cursor = line;
                        }

                        // Here we add lines from diff in specified range (next_line..count_next_lines)
                        slices.push(
                            diff_lines[(index + 1)..(index + count + 1)]
                                .iter()
                                .as_slice(),
                        );
                    }
                    RCSOperations::Delete => {
                        last_command_is_add = false;

                        // Delete is first in diff, in that case we don't need to save something here,
                        // just move the cursor
                        if line != 1 {
                            // Wrong diff encountered
                            if base_filter_lines.len() < line {
                                return make_line_out_of_bounds_error(
                                    line,
                                    base_filter_lines.as_slice(),
                                    diff_lines.as_slice(),
                                );
                            }

                            // Delete means: Save all lines from cursor to the line "to be deleted"
                            // Subtracts 1 here, cuz we do not need to count current line
                            slices.push(
                                base_filter_lines[base_filter_cursor..line - 1]
                                    .iter()
                                    .as_slice(),
                            );
                        }

                        // Then move cursor onto position after deleted range
                        base_filter_cursor = line - 1 + count;
                    }
                }
            }
            Err(e) => return FilterParserError::other_err_from_to_string(e),
        }
    }

    // Add the remaining base filter lines
    if base_filter_cursor < base_filter_lines.len() {
        slices.push(
            base_filter_lines[base_filter_cursor..base_filter_lines.len()]
                .iter()
                .as_slice(),
        );
    }

    // I think, base_filter.len() initial capacity will be better than nothing
    let mut patch_result = slices
        .into_iter()
        .filter(|sub_slice| !sub_slice.is_empty())
        .fold(
            String::with_capacity(base_filter.len()),
            |mut acc, sub_slice| {
                sub_slice.iter().for_each(|line| {
                    acc.push_str(line);
                    acc.push('\n');
                });

                acc
            },
        );

    // This is because diff -n do not respect the final newline character.
    // If last command is `add` and one of (last_base_filter_line_is_line_feed, last_diff_line_is_line_feed), but not both
    // we need to remain empty line, because it was ignored by diff command.
    if !(last_command_is_add && last_base_filter_line_is_line_feed ^ last_diff_line_is_line_feed) {
        patch_result.pop();
    }

    Ok((patch_result, next_diff_path))
}

/// Make special case error: when rcs diff `line` index is out of bounds for current file
fn make_line_out_of_bounds_error<R>(
    requested_line: usize,
    base_filter_lines: &[&str],
    diff_lines: &[&str],
) -> Result<R, FilterParserError> {
    let default_str = "";

    FilterParserError::other_err_from_to_string(
        format!(
            "Wrong diff. Request base file line {}, but it only has {} lines. \nFirst line of base filter: \"{}\".\n First diff line: \"{}\".",
            requested_line,
            base_filter_lines.len(),
            base_filter_lines.first().unwrap_or(&default_str),
            diff_lines.first().unwrap_or(&default_str)
        )
    )
}

#[cfg(test)]
mod tests {
    use super::apply_patch;
    use crate::utils::iterators::lines_with_terminator::lines_with_terminator;

    #[test]
    fn test_no_truncated_remainder() {
        const INPUT: &str = r#"! Checksum: LLmLOdAgjVJdHO1kvbeGPw
! Diff-Path: ../patches/1/1-s-1731418725-3600.patch
! Title: AdGuard Russian filter
! Description: Filter that enables ad blocking on websites in Russian language.
! Version: 2.0.95.29
! TimeUpdated: 2024-11-12T13:31:12+00:00
! Expires: 12 hours (update frequency)
! Homepage: https://github.com/AdguardTeam/AdGuardFilters
! License: https://github.com/AdguardTeam/AdguardFilters/blob/master/LICENSE
!
!-------------------------------------------------------------------------------!
!------------------ General JS API ---------------------------------------------!
!-------------------------------------------------------------------------------!
"#;

        const PATCH: &str = r#"d1 2
a2 2
! Checksum: qEw5hogMcYduk4X9z5Gq0g
! Diff-Path: ../patches/1/1-s-1731422306-3600.patch
d3 2
d5 2
a6 2
! Version: 2.0.95.30
! TimeUpdated: 2024-11-12T14:31:14+00:00
d7 4
"#;
        const OUTPUT: &str = r#"! Checksum: qEw5hogMcYduk4X9z5Gq0g
! Diff-Path: ../patches/1/1-s-1731422306-3600.patch
! Version: 2.0.95.30
! TimeUpdated: 2024-11-12T14:31:14+00:00
!-------------------------------------------------------------------------------!
!------------------ General JS API ---------------------------------------------!
!-------------------------------------------------------------------------------!
"#;
        let patch_list = lines_with_terminator(PATCH).collect();
        let (patched, _) = apply_patch(INPUT, patch_list).unwrap();

        assert_eq!(patched, OUTPUT);
    }

    #[test]
    fn test_apply_patch() {
        const LAO: &str = "The Way that can be told of is not the eternal Way;
The name that can be named is not the eternal name.
The Nameless is the origin of Heaven and Earth;
The Named is the mother of all things.
Therefore let there always be non-being,
  so we may see their subtlety,
And let there always be being,
  so we may see their outcome.
The two are the same,
But after they are produced,
  they have different names.";

        const TZU: &str = "The Nameless is the origin of Heaven and Earth;
The named is the mother of all things.

Therefore let there always be non-being,
  so we may see their subtlety,
And let there always be being,
  so we may see their outcome.
The two are the same,
But after they are produced,
  they have different names.
They both may be called deep and profound.
Deeper and more profound,
The door of all subtleties!";

        const PATCH: &str = "d1 2
d4 1
a4 2
The named is the mother of all things.

a11 3
They both may be called deep and profound.
Deeper and more profound,
The door of all subtleties!";

        let (patched, _) = apply_patch(LAO, PATCH.lines().collect()).unwrap();

        assert_eq!(patched, TZU);
    }

    #[test]
    fn test_apply_patch_with_sequential_addition_and_deletion_on_last_line() {
        const LAO: &str = "The Way that can be told of is not the eternal Way;
The name that can be named is not the eternal name.
The Nameless is the origin of Heaven and Earth;
The Named is the mother of all things.
Therefore let there always be non-being,
  so we may see their subtlety,
And let there always be being,
  so we may see their outcome.
The two are the same,
But after they are produced,
  they have different names.";

        const TZU: &str = "The Nameless is the origin of Heaven and Earth;
The named is the mother of all things.

Therefore let there always be non-being,
  so we may see their subtlety,
And let there always be being,
  so we may see their outcome.
The two are the same,
They both may be called deep and profound.
Deeper and more profound,
The door of all subtleties!
But after they are produced,";

        const PATCH: &str = "d1 2
d4 1
a4 2
The named is the mother of all things.

a9 3
They both may be called deep and profound.
Deeper and more profound,
The door of all subtleties!
d11 1";

        let (patched, _) = apply_patch(LAO, PATCH.lines().collect()).unwrap();

        assert_eq!(patched, TZU);
    }

    #[test]
    fn test_addition_first_and_emojis() {
        const LAO: &str = "The Way that can be told of is not the eternal Way;
The name that can be named is not the eternal name.
The Nameless is the origin of Heaven and Earth;
The Named is the mother of all things.
Therefore let there always be non-being,
  so we may see their subtlety,
And let there always be being,
  so we may see their outcome.
The two are the same,
But after they are produced,
  they have different names.";

        const TZU: &str = "FirstLine
😀😎
The Way that can be told of is not the eternal Way;
The name that can be named is not the eternal name.
The Nameless is the origin of Heaven and Earth;
The named is the mother of all things.

Therefore let there always be non-being,
  so we may see their subtlety,
And let there always be being,
  so we may see their outcome.
The two are the same,
But after they are produced,
  they have different names.
They both may be called deep and profound.
Deeper and more profound,
The door of all subtleties!";

        const PATCH: &str = "a1 2
FirstLine
😀😎
d4 1
a4 2
The named is the mother of all things.

a11 3
They both may be called deep and profound.
Deeper and more profound,
The door of all subtleties!";

        let (patched, _) = apply_patch(LAO, PATCH.lines().collect()).unwrap();

        assert_eq!(patched, TZU);
    }

    #[test]
    fn test_checksum_should_be_ignored() {
        const LAO: &str = "The Way that can be told of is not the eternal Way;
The name that can be named is not the eternal name.
The Nameless is the origin of Heaven and Earth;
The Named is the mother of all things.
Therefore let there always be non-being,
  so we may see their subtlety,
And let there always be being,
  so we may see their outcome.
The two are the same,
But after they are produced,
  they have different names.";

        const TZU: &str = "The Nameless is the origin of Heaven and Earth;
The named is the mother of all things.

Therefore let there always be non-being,
  so we may see their subtlety,
And let there always be being,
  so we may see their outcome.
The two are the same,
But after they are produced,
  they have different names.
They both may be called deep and profound.
Deeper and more profound,
The door of all subtleties!";

        const PATCH: &str = "d1 2
d4 1
a4 2
The named is the mother of all things.

a11 3
They both may be called deep and profound.
Deeper and more profound,
The door of all subtleties!";

        let (patched, _) = apply_patch(LAO, PATCH.lines().collect()).unwrap();

        assert_eq!(patched, TZU);
    }

    #[test]
    fn test_with_diff_directive() {
        const ORIGINAL: &str = "! Title: Diff Updates Simple Example List
! Version: v1.0.0
! Diff-Path: patches/v1.0.0-m-28334060-60.patch
||example.org^";

        const PATCH: &str = "diff checksum:1ce52b527d56a245f32138e014b1571c19cfb659 lines:4
d2 3
a4 3
! Version: v1.0.1
! Diff-Path: patches/v1.0.1-m-28334120-60.patch
||example.com^";

        const CHANGED: &str = "! Title: Diff Updates Simple Example List
! Version: v1.0.1
! Diff-Path: patches/v1.0.1-m-28334120-60.patch
||example.com^";

        let (result, _) = apply_patch(ORIGINAL, PATCH.lines().collect()).unwrap();

        assert_eq!(result, CHANGED);
    }

    #[test]
    fn test_wrong_diff() {
        const ORIGINAL: &str = "! Title: Diff Updates Simple Example List
! Version: v1.0.0
! Diff-Path: patches/v1.0.0-m-28334060-60.patch
||example.org^";

        const PATCH: &str = "d2 3
a400 350
! Version: v1.0.1
! Diff-Path: patches/v1.0.1-m-28334120-60.patch
||example.com^";

        apply_patch(ORIGINAL, PATCH.lines().collect())
            .err()
            .unwrap();
    }
}
