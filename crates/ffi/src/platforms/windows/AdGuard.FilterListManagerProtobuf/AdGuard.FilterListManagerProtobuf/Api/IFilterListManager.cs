using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using FilterListManager;

namespace AdGuard.FilterListManagerProtobuf.Api
{
    public interface IFilterListManager : IDisposable
    {
        /// <summary>
        /// Initializes inner RUST-based FLM under-the-hood the <see cref="IFilterListManager"/>
        /// instance according to the passed <see cref="configuration"/>
        /// </summary>
        void Init(Configuration configuration);
        
        /// <summary>
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
        /// </summary>
        FullFilterList InstallCustomFilterList(
            string downloadUrl,
            bool isTrusted,
            [Optional]
            string title,
            [Optional]
            string description);
        
        /// <summary>
        /// Toggles filter lists, using their `filter_id`.
        ///
        /// * `ids` - List of [`FilterId`].
        /// * `is_enabled` - Does this filter list enabled.
        ///
        /// Returns SQL's affected rows count.
        /// </summary>
        long EnableFilterLists(IEnumerable<int> ids, bool isEnabled);
        
        /// <summary>
        /// Toggles `is_installed` property of filter list.
        ///
        /// * `ids` - List of [`FilterId`].
        /// * `is_installed` - new flag value.
        ///
        /// Returns SQL's affected rows count.
        /// </summary>
        long InstallFilterLists(IEnumerable<int> ids, bool isInstalled);
        
        /// <summary>
        /// Deletes custom filter lists, using their filter_id.
        ///
        /// * `ids` - List of [`FilterId`].
        ///
        /// Returns SQL's affected rows count.
        /// </summary>
        long DeleteCustomFilterLists(IEnumerable<int> ids);
        
        /// <summary>
        /// Returns all filter data including its rules by [`FilterId`]. Fields [`title`, `description`] will be
        /// localised with selected [`Locale`].
        /// </summary>
        FullFilterList GetFullFilterListById(int filterId);
        
        /// <summary>
        /// Returns all stored filters metadata. This is the lightweight counterpart of `.get_full_filter_lists()`
        /// Fields [`title`, `description`] will be localised with selected [`Locale`].
        /// </summary>
        IEnumerable<StoredFilterMetadata> GetStoredFiltersMetadata();
        
        /// <summary>
        /// Returns stored filter metadata by  [`FilterId`]. This is the lightweight counterpart of `.get_full_filter_list_by_id(filter_id)`
        /// Fields [`title`, `description`] will be localised with selected [`Locale`].
        /// </summary>
        StoredFilterMetadata GetStoredFilterMetadataById(int filterId);
        
        /// <summary>
        /// Save custom filter list rules. Note that `filter.time_updated` will be updated too.
        ///
        /// # Failure
        ///
        /// Returns [`Err`] if the specified [`FilterId`] is not found in the
        /// database, or it is not from custom filter.
        /// </summary>
        void SaveCustomFilterRules(FilterListRules rules);
        
        /// <summary>
        /// Saves a set of disabled filters for a specific [`FilterId`]
        ///
        /// # Failure
        ///
        /// Fails if rules_list entity does not exist for passed `filter_id`.
        /// This because if you want to keep disabled filters, you should already
        /// have a `rules_list` entity.
        /// </summary>
        void SaveDisabledRules(int filterId, IEnumerable<string> disabledRules);
        
        /// <summary>
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
        /// </summary>
        UpdateResult UpdateFilters(
            bool ignoreFiltersExpiration,
            int looseTimeout,
            bool ignoreFilterStatus);
        
        /// <summary>
        /// Tries to update passed list of [`FilterId`].
        /// The logic is the same as in the filter update method [`FilterListManager::update_filters`]
        /// with exceptions:
        /// * This returns [`None`] if DB result set is empty.
        /// * This always ignores filters `expires` and `is_enabled` parameters.
        ///
        /// * `ids` - List of [`FilterId`].
        /// * `loose_timeout` - See [`FilterListManager::update_filters`]
        ///   `loose_timeout` parameter for explanation.
        /// </summary>
        UpdateResult ForceUpdateFiltersByIds(IEnumerable<int> filterIds, int looseTimeout);
        
        /// <summary>
        /// Fetches filter list by url and returns its raw metadata.
        ///
        /// * `url` - Remote server or a `file://` URL.
        ///
        /// Returns filter list metadata.
        /// </summary>
        FilterListMetadata FetchFilterListMetadata(string url);
        
        /// <summary>
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
        /// </summary>
        void LiftUpDatabase();
        
        /// <summary>
        /// Gets all tags from DB.
        /// </summary>
        IEnumerable<FilterTag> GetAllTags();
        
        /// <summary>
        /// Gets all groups from DB.
        /// </summary>
        IEnumerable<FilterGroup> GetAllGroups();
        
        /// <summary>
        /// Tries to change [`Locale`] in configuration.
        /// Will search `suggested_locale` in database. If it cannot find exact
        /// locale, like `en_GB`, it will try to find language code - `en`. Locales
        /// with "-", like `en-GB`, will be normalised to internal format - `en_GB`.
        ///
        /// Returns a [`bool`] indicating the success of changing the locale.
        /// If the locale is not found, `false` will be returned.
        /// </summary>
        bool ChangeLocale(string suggestedLocale);
        
        /// <summary>
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
        /// 7. Fill in our updated filters along with the raw filters from the index.
        /// </summary>
        void PullMetadata();
        
        /// <summary>
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
        /// </summary>
        bool UpdateCustomFilterMetadata(int filterId, string title, bool isTrusted);
        
        /// <summary>
        /// Gets absolute path for current database.
        /// </summary>
        string GetDatabasePath();
        
        /// <summary>
        /// Gets version of current database scheme.
        ///
        /// # Special case
        ///
        /// Can return [`None`] if database file exists, but metadata table does not
        /// exist.
        /// </summary>
        int GetDatabaseVersion();
        
        /// <summary>
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
        /// </summary>
        FullFilterList InstallCustomFilterFromString(
            string downloadUrl, 
            long lastDownloadTime, 
            bool isEnabled, 
            bool isTrusted, 
            string filterBody, 
            [Optional]
            string customTitle, 
            [Optional]
            string customDescription);
        
        /// <summary>
        /// Gets a list of [`ActiveRulesInfo`] from filters with `filter.is_enabled=true` flag.
        /// </summary>
        IEnumerable<ActiveRulesInfo> GetActiveRules();
        
        /// <summary>
        /// Gets a list of [`FilterListRulesRaw`] structures containing.
        /// `rules` and `disabled_rules` as strings, directly from database fields.
        ///
        /// This method acts in the same way as the `IN` database operator. Only found entities will be returned
        /// </summary>
        IEnumerable<FilterListRulesRaw> GetFilterRulesAsStrings(IEnumerable<int> filterIds);
        
        /// <summary>
        /// Reads the rule list for a specific filter in chunks, applying exceptions from the disabled_rules list on the fly.
        /// The default size of the read buffer is 1 megabyte. But this size can be exceeded if a longer string appears in the list of filter rules.
        /// The main purpose of this method is to reduce RAM consumption when reading large size filters.
        ///
        /// # Failure
        ///
        /// May return [`crate::FLMError::EntityNotFound()`] with [`FilterId`] if rule list is not found for such id
        /// </summary>
        void SaveRulesToFileBlob(int filterId, string filePath);
        
        /// <summary>
        /// Returns lists of disabled rules by list of filter IDs
        /// </summary>
        IEnumerable<DisabledRulesRaw> GetDisabledRules(IEnumerable<int> filterIds);

        /// <summary>
        /// Sets the proxy mode request
        /// </summary>
        void SetProxyMode(string customProxyAddr, RawRequestProxyMode proxyMode);
    }
}