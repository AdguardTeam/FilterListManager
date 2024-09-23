using System.Collections.Generic;
using AdGuard.FilterListManager.MarshalLogic;

namespace AdGuard.FilterListManager
{
    /// <summary>
    /// FilterListManager is the interface of a filter list manager.
    /// See the original Rust interface filter-list-manager/browse/crates/filter-list-manager/src/manager/mod.rs
    /// </summary>
    public interface IFilterListManager
    {
        /// <summary>
        /// Tries to change [`Locale`] in configuration.
        /// Will search `suggested_locale` in database. If it cannot find exact locale, like `en_GB`, it will try to find language code - `en`.
        /// Locales with "-", like `en-GB`, will be normalised to internal format - `en_GB`.
        /// </summary>
        /// <param name="suggestedLocale">The suggested locale.</param>
        /// <returns>Returns a [`bool`] indicating the success of changing the locale. If the locale is not found, `false` will be returned.</returns>
        /// <exception cref="AgOuterException" />
        bool ChangeLocale(string suggestedLocale);
        
        /// <summary>
        /// Deletes custom filter lists, using their filter_id
        /// </summary>
        /// <param name="ids">List of [`FilterId`]</param>
        /// <returns>Returns SQL's affected rows count</returns>
        /// <exception cref="AgOuterException" />
        long DeleteCustomFilterLists(List<long> ids);
        
        /// <summary>
        /// Toggles filter lists, using their filter_id
        /// </summary>
        /// <param name="ids">List of [`FilterId`]</param>
        /// <param name="isEnabled">Does this filter list enabled</param>
        /// <returns>Returns SQL's affected rows count</returns>
        /// <exception cref="AgOuterException" />
        long EnableFilterLists(List<long> ids, bool isEnabled);

        /// <summary>
        /// Fetches filter list by url and returns its raw metadata
        /// </summary>
        /// <param name="url">Remote server or a `file://` URL</param>
        /// <returns>Returns filter list metadata</returns>
        /// <exception cref="AgOuterException" />
        FilterListMetadata FetchFilterListMetadata(string url);

        /// <summary>
        /// Tries to update passed list of [`FilterId`].
        /// The logic is the same as in the filter update method [`FilterListManager::update_filters`] with exceptions:
        /// * This returns [`None`] if DB result set is empty
        /// * This always ignores filters `expires` and `is_enabled` parameters
        ///
        /// * `ids` - List of [`FilterId`]
        /// * `loose_timeout` - See [`FilterListManager::update_filters`] `loose_timeout` parameter for explanation
        /// </summary>
        /// <param name="ids">List of [`FilterId`]</param>
        /// <param name="looseTimeout">See [`FilterListManager::update_filters`] `loose_timeout` parameter for explanation</param>
        UpdateResult ForceUpdateFiltersByIds(List<long> ids, int looseTimeout);

        /// <summary>
        /// Gets a list of <see cref="ActiveRulesInfo"/> from filters with `filter.is_enabled=true` flag.
        /// </summary>
        /// <exception cref="AgOuterException" />
        List<ActiveRulesInfo> GetActiveRules();

        /// <summary>
        /// Gets all groups from DB
        /// </summary>
        /// <exception cref="AgOuterException" />
        List<FilterGroup> GetAllGroups();

        /// <summary>
        /// Gets all tags from DB
        /// </summary>
        /// <exception cref="AgOuterException" />
        List<FilterTag> GetAllTags();

        /// <summary>
        /// Gets absolute path for current database
        /// </summary>
        /// <exception cref="AgOuterException" />
        string GetDatabasePath();

        /// <summary>
        ///  Gets version of current database scheme
        ///
        /// # Special case
        ///
        /// Can return null if database file exists, but metadata table does not exist
        /// </summary>
        int? GetDatabaseVersion();

        /// <summary>
        /// Returns all filter data by [`FilterId`]. Filter metadata will be localised
        /// </summary>
        /// <param name="id">The filter identifier.</param>
        /// <exception cref="AgOuterException" />
        FullFilterList GetFullFilterListById(long id);
        
        /// <summary>
        /// Gets the stored filters metadata.
        /// </summary>
        /// <exception cref="AgOuterException" />
        List<StoredFilterMetadata> GetStoredFiltersMetadata();

        /// <summary>
        /// Gets the stored filters metadata by identifier.
        /// </summary>
        /// <param name="id">The identifier.</param>
        /// <exception cref="AgOuterException" />
        StoredFilterMetadata GetStoredFiltersMetadataById(long id);

        /// <summary>
        /// Returns all filters with all their data. Filter metadata will be localised
        /// </summary>
        /// <exception cref="AgOuterException" />
        List<FullFilterList> GetFullFilterLists();

        /// <summary>
        /// Installs custom filter from string
        ///
        /// * `download_url` - Download url for filter. String will be placed *as is*.  See [FilterListManager::install_custom_filter_list] for format.
        /// * `last_download_time` - Set `filter.last_download_time` value, which will be added to `filter.expires` and compared to `now()` at [`Self::update_filters`] method.
        /// * `is_enabled` - Filter is enabled?
        /// * `is_trusted` - Filter is trusted?
        /// * `filter_body` - Filter contents
        /// * `custom_title` - Filter may have customized title. See [FilterListManager::install_custom_filter_list]
        /// * `custom_description` - Filter may have customized description. See [FilterListManager::install_custom_filter_list]
        /// </summary>
        /// <param name="downloadUrl"> Download url for filter. String will be placed *as is*.  See [FilterListManager::install_custom_filter_list] for format.</param>
        /// <param name="lastDownloadTime">Set `filter.last_download_time` value, which will be added to `filter.expires` and compared to `now()` at [`Self::update_filters`] method.</param>
        /// <param name="isEnabled"> Filter is enabled?</param>
        /// <param name="isTrusted">Filter is trusted?</param>
        /// <param name="filterBody">Filter content</param>
        /// <param name="customTitle">Filter may have customized title. See [FilterListManager::install_custom_filter_list]</param>
        /// <param name="customDescription">Filter may have customized description. See [FilterListManager::install_custom_filter_list]</param>
        /// <exception cref="AgOuterException" >Throws if `last_download_time` has unsupported format</exception>
        FullFilterList InstallCustomFilterFromString(
            string downloadUrl,
            long lastDownloadTime,
            bool isEnabled,
            bool isTrusted,
            string filterBody,
            string customTitle,
            string customDescription
        );

        /// <summary>
        /// Installs a custom filter list
        /// </summary>
        /// <param name="downloadUrl">Remote server or a `file://` URL. Can be an *empty string*. In that case filter will be local. (e.g. won't be updated)</param>
        /// <param name="isTrusted">Does this filter considered trusted</param>
        /// <param name="title">Override title. If passed, title from metadata will be ignored</param>
        /// <param name="description">Override description. If passed, description from metadata will be ignored</param>
        /// <returns>Returns inserted filter list</returns>
        /// <exception cref="AgOuterException" />
        FullFilterList InstallCustomFilterList(
            string downloadUrl,
            bool isTrusted,
            string title,
            string description
        );

        /// <summary>
        /// Toggles `is_installed` property of filter list
        /// </summary>
        /// <param name="ids">List of [`FilterId`]</param>
        /// <param name="isInstalled">new flag value</param>
        /// <returns>Returns SQL's affected rows count</returns>
        /// <exception cref="AgOuterException" />
        long InstallFilterLists(List<long> ids, bool isInstalled);

        /// <summary>
        /// The method is used for creating a database and downloading filters.
        /// If the database exists, it attempts to bring it to a state compatible with the current indexes.
        /// Additionally, the method checks the downloaded indexes for consistency.
        ///
        /// TODO ?-> Returns [`PullMetadataResult`] as a result
        /// </summary>
        void PullMetadata();

        /// <summary>
        /// Saves the custom filter rules.
        /// </summary>
        /// <param name="rules">The rules to save.</param>
        /// <exception cref="AgOuterException">Throws if the specified [`FilterId`] is not found in the database, or it is not from custom filter</exception>
        void SaveCustomFilterRules(FilterListRules rules);

        /// <summary>
        /// Saves a set of disabled filters for a specific [`FilterId`]
        /// </summary>
        /// <param name="filterId">The filter identifier.</param>
        /// <param name="disabledRules">The disabled rules.</param>
        /// <exception cref="AgOuterException">Throws if rules_list entity does not exist for passed `filter_id`.
        /// This because if you want to keep disabled filters, you should already have a `rules_list` entity</exception>
        void SaveDisabledRules(long filterId, List<string> disabledRules);

        /// <summary>
        /// Updates custom filter data.
        /// </summary>
        /// <param name="filterId">Custom filter id.</param>
        /// <param name="title">New `title` for filter</param>
        /// <param name="isTrusted">New `is_trusted` status for filter</param>
        /// <returns>TODO ?</returns>
        /// <exception cref="AgOuterException">Throws if manager couldn't find a filter by `filter_id` or if `filter_id` is not from a custom filter. Throws if title is empty</exception>
        bool UpdateCustomFilterMetadata(long filterId, string title, bool isTrusted);

        /// <summary>
        /// Filters updates occur in several steps:
        /// - Search for filters ready for update
        /// - Fetch them
        /// - Save last_download_time, and update metadata
        /// - Collect updated filters 
        /// </summary>
        /// <param name="ignoreFiltersExpiration">Does not rely on filter's expires information</param>
        /// <param name="looseTimeout">Not a strict timeout, checked after processing each filter. If the total time exceeds this value, filters processing will stop, and the number of unprocessed filters will be set in result value. Pass 0 to disable timeout</param>
        /// <param name="ignoreFiltersStatus">Include disabled filters</param>
        /// <returns>Returns [`UpdateResult`] with update information, Returns null if DB is empty</returns>
        /// <exception cref="AgOuterException">Throws if you can not get records from db, or common error encountered</exception>
        UpdateResult UpdateFilters(
            bool ignoreFiltersExpiration,
            int looseTimeout,
            bool ignoreFiltersStatus
        );

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
    }
}
