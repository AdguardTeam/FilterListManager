/// Special `str.chars()` case, which respects trailing line
pub(crate) fn lines_with_terminator<'a>(value: &'a str) -> impl Iterator<Item = &str> + 'a {
    value.lines().chain(if value.ends_with('\n') {
        Some("")
    } else {
        None
    })
}