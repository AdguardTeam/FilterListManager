namespace AdGuard.FilterListManager.RustInterface
{
    internal enum FfiMethod
    {
        InstallCustomFilterList,
        EnableFilterLists,
        InstallFilterLists,
        DeleteCustomFilterLists,
        GetFullFilterListById,
        GetStoredFiltersMetadata,
        GetStoredFilterMetadataById,
        SaveCustomFilterRules,
        SaveDisabledRules,
        UpdateFilters,
        ForceUpdateFiltersByIds,
        FetchFilterListMetadata,
        FetchFilterListMetadataWithBody,
        LiftUpDatabase,
        GetAllTags,
        GetAllGroups,
        ChangeLocale,
        PullMetadata,
        UpdateCustomFilterMetadata,
        GetDatabasePath,
        GetDatabaseVersion,
        InstallCustomFilterFromString,
        GetActiveRules,
        GetFilterRulesAsStrings,
        SaveRulesToFileBlob,
        GetDisabledRules,
        SetProxyMode,
        GetRulesCount,
        // There are ffi methods which not used for working with filters explicitly
        Init,
        SpawnDefaultConfiguration,
    }
}
