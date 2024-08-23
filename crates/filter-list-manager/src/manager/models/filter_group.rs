//! FilterListGroup represents a group of filter lists that have similar purpose.
use serde::{Deserialize, Serialize};

/// FilterListGroup represents a group of filter lists that have similar
/// purpose.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FilterGroup {
    /// Group id
    pub id: i32,
    /// Group name
    pub name: String,
    /// Display number for ordering
    pub display_number: i32,
}
