//! Filter list manager library main facade interface.
pub mod filter_list_manager_impl;
pub(crate) mod filter_lists_builder;
pub mod managers;
pub mod models;
mod update_filters_action;

use crate::manager::models::active_rules_info::ActiveRulesInfo;
use crate::manager::models::configuration::request_proxy_mode::RequestProxyMode;
use crate::manager::models::configuration::Locale;
use crate::manager::models::disabled_rules_raw::DisabledRulesRaw;
use crate::manager::models::filter_group::FilterGroup;
use crate::manager::models::filter_list_rules::FilterListRules;
use crate::manager::models::filter_list_rules_raw::FilterListRulesRaw;
use crate::manager::models::filter_tag::FilterTag;
use crate::manager::models::rules_count_by_filter::RulesCountByFilter;
use crate::manager::models::{PullMetadataResult, UpdateResult};
use crate::{FLMResult, StoredFilterMetadata};
use models::configuration::Configuration;
use models::filter_list_metadata::FilterListMetadata;
use models::filter_list_metadata_with_body::FilterListMetadataWithBody;
use models::full_filter_list::FullFilterList;
use models::FilterId;
use std::path::Path;

/// FilterListManager is the interface of a filter list manager.
pub trait FilterListManager {
    /// In the constructor, the object is configured and initialized depending on the passed configuration.
    /// *NOTE:* You must create its own manager for different filter lists types.
    ///
    /// * `configuration` - Configuration object for this manager
    fn new(configuration: Configuration) -> FLMResult<Box<Self>>;

    /// Installs a custom filter list
    ///
    /// * `download_url` - Remote server or a `file://` URL. Can be an
    ///   *empty string*. In that case filter will be local. (e.g. won't be
    ///   updated).
    /// * `is_trusted` - Does this filter considered trusted.
    /// * `title` - Override title. If passed, title from metadata will be
    ///   ignored.
    /// * `description` - Override description. If passed, description from
    ///   metadata will be ignored.
    ///
    /// Returns the inserted filter list.
    fn install_custom_filter_list(
        &self,
        download_url: String,
        is_trusted: bool,
        title: Option<String>,
        description: Option<String>,
    ) -> FLMResult<FullFilterList>;

    /// Fetches filter list by url and returns its raw metadata.
    ///
    /// * `url` - Remote server or a `file://` URL.
    ///
    /// Returns filter list metadata.
    fn fetch_filter_list_metadata(&self, url: String) -> FLMResult<FilterListMetadata>;

    /// Fetches filter list by url and returns its raw metadata and body.
    ///
    /// * `url` - Remote server or a `file://` URL.
    ///
    /// Returns filter list metadata and body.
    fn fetch_filter_list_metadata_with_body(
        &self,
        url: String,
    ) -> FLMResult<FilterListMetadataWithBody>;

    /// Toggles filter lists, using their `filter_id`.
    ///
    /// * `ids` - List of [`FilterId`].
    /// * `is_enabled` - Does this filter list enabled.
    ///
    /// Returns SQL's affected rows count.
    fn enable_filter_lists(&self, ids: Vec<FilterId>, is_enabled: bool) -> FLMResult<usize>;

    /// Toggles `is_installed` property of filter list.
    ///
    /// * `ids` - List of [`FilterId`].
    /// * `is_installed` - new flag value.
    ///
    /// Returns SQL's affected rows count.
    fn install_filter_lists(&self, ids: Vec<FilterId>, is_installed: bool) -> FLMResult<usize>;

    /// Deletes custom filter lists, using their filter_id.
    ///
    /// * `ids` - List of [`FilterId`].
    ///
    /// Returns SQL's affected rows count.
    fn delete_custom_filter_lists(&self, ids: Vec<FilterId>) -> FLMResult<usize>;

    /// Gets all tags from DB.
    fn get_all_tags(&self) -> FLMResult<Vec<FilterTag>>;

    /// Gets all groups from DB.
    fn get_all_groups(&self) -> FLMResult<Vec<FilterGroup>>;

    /// Returns all filter data including its rules by [`FilterId`]. Fields [`title`, `description`] will be
    /// localised with selected [`Locale`].
    fn get_full_filter_list_by_id(&self, filter_id: FilterId) -> FLMResult<Option<FullFilterList>>;

    /// Returns all stored filters metadata. This is the lightweight counterpart of `.get_full_filter_lists()`
    /// Fields [`title`, `description`] will be localised with selected [`Locale`].
    fn get_stored_filters_metadata(&self) -> FLMResult<Vec<StoredFilterMetadata>>;

    /// Returns stored filter metadata by  [`FilterId`]. This is the lightweight counterpart of `.get_full_filter_list_by_id(filter_id)`
    /// Fields [`title`, `description`] will be localised with selected [`Locale`].
    fn get_stored_filter_metadata_by_id(
        &self,
        filter_id: FilterId,
    ) -> FLMResult<Option<StoredFilterMetadata>>;

    /// Save custom filter list rules. Note that `filter.time_updated` will be updated too.
    ///
    /// # Failure
    ///
    /// Returns [`Err`] if the specified [`FilterId`] is not found in the
    /// database, or it is not from custom filter.
    fn save_custom_filter_rules(&self, rules: FilterListRules) -> FLMResult<()>;

    /// Saves a set of disabled filters for a specific [`FilterId`]
    ///
    /// # Failure
    ///
    /// Fails if rules_list entity does not exist for passed `filter_id`.
    /// This because if you want to keep disabled filters, you should already
    /// have a `rules_list` entity.
    fn save_disabled_rules(
        &self,
        filter_id: FilterId,
        disabled_rules: Vec<String>,
    ) -> FLMResult<()>;

    /// Filters updates is conducted in the multiple steps:
    /// - Search for filters ready for update.
    /// - Fetch them.
    /// - Save `last_download_time`, and update metadata.
    /// - Collect updated filters.
    ///
    /// * `ignore_filters_expiration` - Does not rely on filter's expire
    ///   information.
    /// * `loose_timeout` - Not a strict timeout, checked after processing each
    ///   filter. If the total time exceeds this value, filters processing will
    ///   stop, and the number of unprocessed filters will be set in result
    ///   value. Pass 0 to disable timeout.
    /// * `ignore_filters_status` - Include disabled filters
    ///
    /// Returns [`UpdateResult`] with update information.
    ///
    /// # Failure
    ///
    /// Returns [`None`] if DB is empty.
    ///
    /// Returns [`Err`] if you can not get records from db, or common error
    /// encountered.
    ///
    /// Note: should be used once an hour or less.
    fn update_filters(
        &self,
        ignore_filters_expiration: bool,
        loose_timeout: i32,
        ignore_filters_status: bool,
    ) -> FLMResult<Option<UpdateResult>>;

    /// Tries to update passed list of [`FilterId`].
    /// The logic is the same as in the filter update method [`FilterListManager::update_filters`]
    /// with exceptions:
    /// * This returns [`None`] if DB result set is empty.
    /// * This always ignores filters `expires` and `is_enabled` parameters.
    ///
    /// * `ids` - List of [`FilterId`].
    /// * `loose_timeout` - See [`FilterListManager::update_filters`]
    ///   `loose_timeout` parameter for explanation.
    ///
    /// Note: should be used once an hour or less.
    fn force_update_filters_by_ids(
        &self,
        ids: Vec<FilterId>,
        loose_timeout: i32,
    ) -> FLMResult<Option<UpdateResult>>;

    /// Tries to change [`Locale`] in configuration.
    /// Will search `suggested_locale` in database. If it cannot find exact
    /// locale, like `en_GB`, it will try to find language code - `en`. Locales
    /// with "-", like `en-GB`, will be normalised to internal format - `en_GB`.
    ///
    /// Returns a [`bool`] indicating the success of changing the locale.
    /// If the locale is not found, `false` will be returned.
    fn change_locale(&mut self, suggested_locale: Locale) -> FLMResult<bool>;

    /// The method is used for creating a database and downloading filters.
    /// If the database exists, it attempts to bring it to a state compatible
    /// with the current indexes. Also, migrations update will be processed in this method, too.
    /// Additionally, the method checks the downloaded indexes for consistency.
    ///
    /// This method follows the algorithm below:
    ///
    /// 1. Downloads the filters index (registry).
    /// 2. Checks the index consistency.
    ///     a. Take filters from the index.
    ///     b. For each filter check that `filter.group_id` > 0 and the group
    ///        is present in the index.
    ///     c. For each filter check that tag is present in the index.
    ///     d. `filter.name` (title) is not empty.
    ///     e. `filter.download_url` must be unique.
    ///     f. Everything else is not a critical issue.
    /// 3. Opens the database with `O_CREAT`.
    ///    a. Check that the database is empty (by the presence of the `filter`
    ///       table).
    ///    b. If empty, pour the schema and save the data from the indexes and
    ///       finish the exercise.
    ///    c. Otherwise, go to the next step.
    /// 4. Select all filters from the database, then iterate on every filter.
    ///    When comparing filters from the index and the database, we rely on
    ///    the filter.id.
    ///    a. If it is a custom filter - (`group_id` < 1) -> continue.
    ///    b. Do not work with `filter_id` < 1 (reserved filters) -> continue.
    ///    c. If a filter is enabled and is not in the new index -> move it to
    ///       the custom group and change its ID.
    ///    d. If the filter is disabled or not installed -> delete it.
    ///    e. Take the filter and replace the following fields with values from
    ///       the index:
    ///       * `display_number`
    ///       * `title`
    ///       * `description`
    ///       * `homepage`
    ///       * `expires`
    ///       * `download_url`
    ///       * `subscription_url`
    ///       * `last_update_time`
    ///    f. Mark the filter in the index as processed.
    /// 5. Remove old groups/tags/locales.
    /// 6. Fill in new groups/tags/locales.
    /// 7. Fill in our updated filters along with the raw filters from the
    /// index.
    ///
    /// Note: should be used once a week or less.
    ///
    /// # Returns
    ///
    /// [`PullMetadataResult`] - the result of index metadata update
    fn pull_metadata(&self) -> FLMResult<PullMetadataResult>;

    /// Updates custom filter data.
    ///
    /// * `filter_id` - Custom filter id.
    /// * `title` - New `title` for filter.
    /// * `is_trusted` - New `is_trusted` status for filter.
    ///
    /// # Failure
    ///
    /// Fails if manager couldn't find a filter by `filter_id` or if `filter_id`
    /// is not from a custom filter. Fails if title is empty.
    fn update_custom_filter_metadata(
        &self,
        filter_id: FilterId,
        title: String,
        is_trusted: bool,
    ) -> FLMResult<bool>;

    /// Gets absolute path for current database.
    fn get_database_path(&self) -> FLMResult<String>;

    /// The method “raises” the state of the database to the working state.
    ///
    /// **If the database doesn't exist:**
    /// - Creates database
    /// - Rolls up the schema
    /// - Rolls migrations
    /// - Performs bootstrap.
    ///
    /// **If the database is an empty file:**
    /// - Rolls the schema
    /// - Rolls migrations
    /// - Performs bootstrap.
    ///
    ///... and so on.
    fn lift_up_database(&self) -> FLMResult<()>;

    /// Gets version of current database scheme.
    ///
    /// # Special case
    ///
    /// Can return [`None`] if database file exists, but metadata table does not
    /// exist.
    fn get_database_version(&self) -> FLMResult<Option<i32>>;

    /// Installs custom filter from string
    ///
    /// * `download_url` - Download url for filter. String will be placed
    ///   *as is*.  See [FilterListManager::install_custom_filter_list] for the
    ///   format.
    /// * `last_download_time` - Set `filter.last_download_time` value, which
    ///   will be added to `filter.expires` and compared to `now()` at
    ///   [`Self::update_filters`] method.
    /// * `is_enabled` - True if the filter is enabled.
    /// * `is_trusted` - True if the filter is trusted.
    /// * `filter_body` - Filter contents.
    /// * `custom_title` - Filter may have customized title.
    ///   See [FilterListManager::install_custom_filter_list].
    /// * `custom_description` - Filter may have customized description.
    ///   See [FilterListManager::install_custom_filter_list].
    ///
    /// # Failure
    ///
    /// Returns [`Err`] if `last_download_time` has unsupported format.
    fn install_custom_filter_from_string(
        &self,
        download_url: String,
        last_download_time: i64,
        is_enabled: bool,
        is_trusted: bool,
        filter_body: String,
        custom_title: Option<String>,
        custom_description: Option<String>,
    ) -> FLMResult<FullFilterList>;

    /// Gets a list of [`ActiveRulesInfo`] from filters with `filter.is_enabled=true` flag.
    fn get_active_rules(&self) -> FLMResult<Vec<ActiveRulesInfo>>;

    /// Gets a list of [`FilterListRulesRaw`] structures containing.
    /// `rules` and `disabled_rules` as strings, directly from database fields.
    ///
    /// This method acts in the same way as the `IN` database operator. Only found entities will be returned
    fn get_filter_rules_as_strings(&self, ids: Vec<FilterId>)
        -> FLMResult<Vec<FilterListRulesRaw>>;

    /// Reads the rule list for a specific filter in chunks, applying exceptions from the disabled_rules list on the fly.
    /// The default size of the read buffer is 1 megabyte. But this size can be exceeded if a longer string appears in the list of filter rules.
    /// The main purpose of this method is to reduce RAM consumption when reading large size filters.
    ///
    /// # Failure
    ///
    /// May return [`crate::FLMError::EntityNotFound()`] with [`FilterId`] if rule list is not found for such id
    fn save_rules_to_file_blob<P: AsRef<Path>>(
        &self,
        filter_id: FilterId,
        file_path: P,
    ) -> FLMResult<()>;

    /// Returns lists of disabled rules by list of filter IDs
    fn get_disabled_rules(&self, ids: Vec<FilterId>) -> FLMResult<Vec<DisabledRulesRaw>>;

    /// Sets a new proxy mode. Value will be applied on next method call
    fn set_proxy_mode(&mut self, mode: RequestProxyMode);

    /// Returns lists of rules count by list of filter IDs
    fn get_rules_count(&self, ids: Vec<FilterId>) -> FLMResult<Vec<RulesCountByFilter>>;
}
