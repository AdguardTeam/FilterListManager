//! A configuration switch specifying exactly what type of filters this [`crate::FilterListManager`] instance works with.
//! If you need to work with both standard filters and dns filters, you should create two different manager instances with two different configurations

use enum_stringify::EnumStringify;

/// A filter lists type of current [`crate::FilterListManager`] instance
#[derive(Copy, Clone, EnumStringify)]
pub enum FilterListType {
    /// Standard filters
    STANDARD,
    /// DNS Filters
    DNS,
    /// Container for misc filters. For example, you can put
    MISC,
}

impl FilterListType {
    /// Generates option|path name for enum element
    pub fn to_name(self) -> String {
        self.to_string().to_lowercase()
    }
}
