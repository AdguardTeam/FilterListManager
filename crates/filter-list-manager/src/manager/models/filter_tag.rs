//! Filter list tag representation.
use serde::{Deserialize, Serialize};

/// FilterListTag represents a tag of a filter list. A filter list may have
/// multiple tags.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FilterTag {
    /// Filter tag id.
    pub id: i32,

    /// Filter keyword.
    /// Mostly represents special instruction, like:
    /// - purpose:privacy
    /// - lang:de
    /// - platform:mobile
    /// ... and so on
    pub keyword: String,
}
