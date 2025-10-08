namespace AdGuard.FilterListManager.RustInterface
{
    /// <summary>
    /// Represents RUST method. See <see cref="IFilterListManager"/> for more information.
    /// </summary>
    public enum FfiMethod
    {
        /// <summary>
        /// Installs a custom filter list specified by metadata
        /// </summary>
        InstallCustomFilterList,
        
        /// <summary>
        /// Enables or disables specified filter lists
        /// </summary>
        EnableFilterLists,
        
        /// <summary>
        /// Installs filter lists by their identifiers
        /// </summary>
        InstallFilterLists,
        
        /// <summary>
        /// Deletes custom filter lists by their identifiers
        /// </summary>
        DeleteCustomFilterLists,
        
        /// <summary>
        /// Retrieves complete filter list information by its identifier
        /// </summary>
        GetFullFilterListById,
        
        /// <summary>
        /// Gets metadata for all stored filters
        /// </summary>
        GetStoredFiltersMetadata,
        
        /// <summary>
        /// Gets metadata for a specific filter by its identifier
        /// </summary>
        GetStoredFilterMetadataById,
        
        /// <summary>
        /// Saves custom filter rules to a specified filter
        /// </summary>
        SaveCustomFilterRules,
        
        /// <summary>
        /// Saves rules that should be disabled
        /// </summary>
        SaveDisabledRules,
        
        /// <summary>
        /// Updates all installed filters
        /// </summary>
        UpdateFilters,
        
        /// <summary>
        /// Forces update of specific filters by their identifiers
        /// </summary>
        ForceUpdateFiltersByIds,
        
        /// <summary>
        /// This method works almost the same as `updateFilters`
        /// But also, you MUST pass the list of `FilterId`
        /// Empty list will cause an empty `UpdateResult` if database exists.
        /// This returns null if db is empty
        /// </summary>
        UpdateFiltersByIds,
        
        /// <summary>
        /// Fetches metadata for a filter list
        /// </summary>
        FetchFilterListMetadata,
        
        /// <summary>
        /// Fetches both metadata and content for a filter list
        /// </summary>
        FetchFilterListMetadataWithBody,
        
        /// <summary>
        /// Upgrades the database to the latest version
        /// </summary>
        LiftUpDatabase,
        
        /// <summary>
        /// Gets all available filter tags
        /// </summary>
        GetAllTags,
        
        /// <summary>
        /// Gets all available filter groups
        /// </summary>
        GetAllGroups,
        
        /// <summary>
        /// Changes the locale used for filters metadata
        /// </summary>
        ChangeLocale,
        
        /// <summary>
        /// Pulls updated metadata from remote sources
        /// </summary>
        PullMetadata,
        
        /// <summary>
        /// Updates metadata for a custom filter
        /// </summary>
        UpdateCustomFilterMetadata,
        
        /// <summary>
        /// Gets the path to the filters database
        /// </summary>
        GetDatabasePath,
        
        /// <summary>
        /// Gets the current version of the filters database
        /// </summary>
        GetDatabaseVersion,
        
        /// <summary>
        /// Installs a custom filter from a string containing rules
        /// </summary>
        InstallCustomFilterFromString,
        
        /// <summary>
        /// Gets all currently active filtering rules
        /// </summary>
        GetActiveRules,
        
        /// <summary>
        /// Gets a list of [`ActiveRulesInfoRaw`] from filters with `filter.is_enabled=true` flag.
        /// `filter_by` - If empty, returns all active rules, otherwise returns intersection between `filter_by` and all active rules
        /// </summary>
        GetActiveRulesRaw,
        
        /// <summary>
        /// Gets filter rules as strings for a specific filter
        /// </summary>
        GetFilterRulesAsStrings,
        
        /// <summary>
        /// Saves rules to a file blob
        /// </summary>
        SaveRulesToFileBlob,
        
        /// <summary>
        /// Gets rules that are currently disabled
        /// </summary>
        GetDisabledRules,
        
        /// <summary>
        /// Sets the proxy mode for filter updates
        /// </summary>
        SetProxyMode,
        
        /// <summary>
        /// Gets the count of rules in filters
        /// </summary>
        GetRulesCount,
        
        // There are ffi methods that not used for working with filters explicitly
        
        /// <summary>
        /// Initializes the filter list manager
        /// </summary>
        Init,
        
        /// <summary>
        /// Creates a default configuration for the filter list manager
        /// </summary>
        SpawnDefaultConfiguration,
    }
}
