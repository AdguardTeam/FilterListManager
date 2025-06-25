//! FullFilterList represents a filter list and all its associated metadata.
use super::filter_tag::FilterTag;
use super::FilterId;
use crate::manager::models::filter_list_rules::FilterListRules;
use crate::storage::entities::filter::filter_entity::FilterEntity;
use crate::StoredFilterMetadata;

/// FullFilterList represents a filter list and all its associated metadata.
///
/// Keep in mind that this structure has a lightweight counterpart without filter contents - [`crate::StoredFilterMetadata`]
#[derive(Clone, Debug)]
pub struct FullFilterList {
    /// Filter list unique ID. If the filter list comes from a registry, this
    /// ID comes from the registry and must be unique inside of it.
    /// If the filter list is a custom list or a pre-installed one (user rules)
    /// the ID is passed by the caller and also must be unique.
    /// In order to provide more consistency, the library provides several
    /// constants that are recommended to use for such lists,
    /// see FilterListManagerConstants for more details.
    pub id: FilterId,
    /// Group ID this filter list belongs to. If the filter list comes from a
    /// registry, this field is filled with a valid group ID and the group
    /// metadata also comes from the registry. If the filter list is a custom
    /// or a pre-defined one, the group ID is passed by the caller. In order
    /// to provide more consistency, the library provides several constants
    /// that are recommended to use for such groups, see
    /// FilterListManagerConstants for more details.
    pub group_id: i32,
    /// Timestamp (seconds from epoch) when this filter was updated on the
    /// server. This field comes either from the registry or parsed for the list
    /// content's `! TimeUpdated:` metadata field. For filter lists that
    /// are already installed version in the registry is ignored.
    ///
    /// The TimeUpdated field is a string in the format `2024-07-31T12:31:19+00:00`
    /// If the value isn't specified or is incorrectly formatted, the current timestamp will be used
    pub time_updated: i64,
    /// Timestamp (seconds from epoch) when this filter list's content was
    /// last downloaded from the `download_url`.
    pub last_download_time: i64,
    /// Title either from the list metadata in the registry or parsed from the
    /// list content's `! Title:` metadata field. If the field comes from
    /// registry, the parsed value will be ignored.
    pub title: String,
    /// Description either from the list metadata in the registry or parsed from
    /// the list content's `! Description:` metadata field. If the field comes
    /// from registry, the parsed value will be ignored.
    pub description: String,
    /// Version either from the list metadata in the registry or parsed from
    /// the list content's `! Version:` metadata field. For filter lists that
    /// are already installed version in the registry is ignored.
    pub version: String,
    /// Filter list display number. Comes from the list metadata in the
    /// registry.
    ///
    /// This is a metadata field that is used in the products UI, it
    /// is not used by the library itself.
    ///
    /// In the case of a custom or pre-defined list `display_number` is 0.
    pub display_number: i32,
    /// Filter list download URL. Comes either from the list metadata in the
    /// registry or passed by the caller in the case of a custom list. This
    /// field can be empty. In this case the library won't attempt to download
    /// the filter list updates.
    pub download_url: String,
    /// Filter list "source" subscription url. This field only makes sense for
    /// third-party lists that come from the registry. The idea is that these
    /// lists are re-hosted by the registry and downloaded not from the source
    /// URL. For instance, let's take Easylist. Its original source URL is
    /// <https://easylist.to/easylist/easylist.txt>, but the registry re-hosts it
    /// on different URLs like <https://filters.adtidy.org/extension/chromium/filters/101.txt>
    /// However, the caller may need to know the source URL in order to properly
    /// handle abp:subscribe links.
    ///
    /// This is a metadata field that is used in the products UI, it
    /// is not used by the library itself.
    pub subscription_url: String,
    /// An array of tags of this filter list. Every filter list in the registry
    /// can have multiple tags attached to it. Tags describe the list's purpose
    /// or other properties (like "lang" tags for instance).
    pub tags: Vec<FilterTag>,
    /// Number of seconds that needs to pass since the last full filter update
    /// until the filter list is considered outdated and needs to be updated.
    /// Comes either from the list metadata in the registry or passed by the
    /// caller in the case of a custom list or parsed from the list content's
    /// `! Expires:` metadata field. The last received value is used by the
    /// library (make sure that it's consistent in registry & content).
    pub expires: i32,
    /// Indicates if the filter list is marked as trusted or not. This flag must
    /// be passed by the caller further to the filtering engine, and it controls
    /// what types of rules are allowed in the list. Comes either from the list
    /// metadata in the registry or passed by the caller in the case of a custom
    /// list.
    pub is_trusted: bool,
    /// Indicates whether the filter list came from a registry or was created by
    /// the caller or by the service filter lists. This field is used to
    /// distinguish between custom and registry filters when parsing the list
    /// and evaluating metadata fields.
    ///
    /// Custom filter lists can be added using `install_custom_filter_list`. In
    /// this case the library sets `group_id` to `CUSTOM_FILTERS_GROUP_ID`.
    ///
    /// Alternatively, if a registry-based filter list was removed in the new
    /// registry version, it is automatically converted to a custom list.
    ///
    /// Service filter lists are created by the library itself when initiating
    /// the database and cannot be removed. At the moment the only service list
    /// is the so-called "User rules" list.
    pub is_custom: bool,
    /// Indicates whether the filter list is enabled or not. This flag is
    /// controlled by the caller.
    ///
    /// 1. If the filter list is disabled, it is not used by the filtering
    ///    engine (see `get_active_rules`).
    /// 2. If the filter list is disabled, the library does not attempt to
    ///    download the filter updates.
    pub is_enabled: bool,
    /// Indicates if the filter is installed or not. This field is purely
    /// metadata and is not used by the library itself. It is used in AdGuard
    /// for Windows and macOS to indicate which lists has been installed by
    /// the user.
    pub is_installed: bool,
    /// List homepage URL. Comes either from the list metadata in the registry
    /// or parsed from the list content's `! Homepage:` metadata field.
    ///
    /// Note, that the library does not validate that it is a valid URL.
    pub homepage: String,
    /// List license URL. Parsed from the list content's `! License:` metadata
    /// field.
    ///
    /// Note, that the library does not validate that it is a valid URL.
    pub license: String,
    /// Filter checksum. Parsed from the list content's `! Checksum:` metadata
    /// field.
    pub checksum: String,
    /// A list of languages if the list is regional. This list of languages can
    /// only be set in the filters registry. It is either a two letter language
    /// code without locale (i.e. `en`, `zh`) or with locale (i.e. `en-GB`,
    /// etc.)
    pub languages: Vec<String>,
    /// Container for rules.
    ///
    /// This field can be empty when the filter list is not yet downloaded, but
    /// it was received from the registry (using `pull_metadata`).
    pub rules: Option<FilterListRules>,
}

impl FullFilterList {
    /// Builds `[Self]` from `[StoredFilterMetadata]` and rules
    pub(crate) fn from_stored_filter_metadata(
        stored_filter_metadata: StoredFilterMetadata,
        rules: Option<FilterListRules>,
    ) -> Self {
        Self {
            id: stored_filter_metadata.id,
            group_id: stored_filter_metadata.group_id,
            time_updated: stored_filter_metadata.time_updated,
            last_download_time: stored_filter_metadata.last_download_time,
            title: stored_filter_metadata.title,
            description: stored_filter_metadata.description,
            version: stored_filter_metadata.version,
            display_number: stored_filter_metadata.display_number,
            download_url: stored_filter_metadata.download_url,
            subscription_url: stored_filter_metadata.subscription_url,
            tags: stored_filter_metadata.tags,
            expires: stored_filter_metadata.expires,
            is_trusted: stored_filter_metadata.is_trusted,
            is_custom: stored_filter_metadata.is_custom,
            is_enabled: stored_filter_metadata.is_enabled,
            is_installed: stored_filter_metadata.is_installed,
            homepage: stored_filter_metadata.homepage,
            license: stored_filter_metadata.license,
            checksum: stored_filter_metadata.checksum,
            languages: stored_filter_metadata.languages,
            rules,
        }
    }
    /// Builds `[Self]` from `[FilterEntity]` and friends
    pub(crate) fn from_filter_entity(
        entity: FilterEntity,
        tags: Vec<FilterTag>,
        languages: Vec<String>,
        rules: Option<FilterListRules>,
    ) -> Option<Self> {
        if let Some(filter_id) = entity.filter_id {
            let is_custom = entity.is_custom();

            return Some(Self {
                id: filter_id,
                group_id: entity.group_id,
                time_updated: entity.last_update_time,
                last_download_time: entity.last_download_time,
                title: entity.title,
                description: entity.description,
                version: entity.version,
                display_number: entity.display_number,
                download_url: entity.download_url,
                subscription_url: entity.subscription_url,
                tags,
                expires: entity.expires,
                is_trusted: entity.is_trusted,
                is_custom,
                is_enabled: entity.is_enabled,
                homepage: entity.homepage,
                license: entity.license,
                checksum: entity.checksum,
                languages,
                rules,
                is_installed: entity.is_installed,
            });
        }

        None
    }
}
