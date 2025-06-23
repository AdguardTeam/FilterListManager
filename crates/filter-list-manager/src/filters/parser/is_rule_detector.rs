/// This marks line as "non-rule" line: comment, directive, etc.
pub const NON_RULE_MARKER: char = '!';

/// Also, comment line can start from "# " sequence
pub const EXTRA_COMMENT_MARKER: &str = "# ";

/// Determines if string is a rule
pub(crate) fn is_line_is_rule(line: &str) -> bool {
    !(line.is_empty()
        || line.starts_with(NON_RULE_MARKER)
        || line.starts_with(EXTRA_COMMENT_MARKER))
}
