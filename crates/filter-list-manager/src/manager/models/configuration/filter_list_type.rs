//! A configuration switch specifying exactly what type of filters this [`crate::FilterListManager`] instance works with.
//! If you need to work with both standard filters and dns filters, you should create two different manager instances with two different configurations

/// A filter lists type of current [`crate::FilterListManager`] instance
#[derive(Copy, Clone)]
pub enum FilterListType {
    /// Standard filters
    STANDARD,
    /// DNS Filters
    DNS,
}
