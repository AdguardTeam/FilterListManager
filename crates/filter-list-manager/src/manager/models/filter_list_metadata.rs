//! Metadata for a remote filter list.

/// Metadata for a remote filter list. This structure represents metadata parsed from
/// the list content and it is exposed via `fetch_filter_list_metadata`.
#[derive(Clone, Debug)]
pub struct FilterListMetadata {
    /// Title from the `! Title:` metadata field.
    pub title: String,
    /// Description from the `! Description:` metadata field.
    pub description: String,
    /// Time updated from the `! TimeUpdated:` metadata field.
    pub time_updated: String,
    /// Version from the `! Version:` metadata field.
    pub version: String,
    /// Homepage from the `! Homepage:` metadata field.
    pub homepage: String,
    /// License from the `! License:` metadata field.
    pub license: String,
    /// Checksum from the `! Checksum:` metadata field.
    pub checksum: String,
    /// URL or local path where the filter content was downloaded from.
    pub url: String,
    /// Rules count in this filter list. Simply a number of non-empty lines
    /// and does not start with a comment marker.
    pub rules_count: i32,
}
