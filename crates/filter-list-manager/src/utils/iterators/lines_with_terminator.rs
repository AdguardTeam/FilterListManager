/// Special `str.chars()` case, which respects trailing line
pub(crate) fn lines_with_terminator(value: &str) -> impl Iterator<Item = &str> {
    value.lines().chain(if value.ends_with('\n') {
        Some("")
    } else {
        None
    })
}
