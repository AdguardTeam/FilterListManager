#[inline]
/// Box shortcut
pub(crate) fn heap<T>(boxed_value: T) -> Box<T> {
    Box::new(boxed_value)
}
