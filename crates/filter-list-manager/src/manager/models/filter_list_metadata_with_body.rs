//! Metadata for a remote filter list.
use crate::FilterListMetadata;

/// Metadata (with body) for a remote filter list.
#[derive(Clone, Debug)]
pub struct FilterListMetadataWithBody {
    /// Metadata for a remote filter list.
    pub metadata: FilterListMetadata,
    /// Filter body.
    pub filter_body: String,
}
