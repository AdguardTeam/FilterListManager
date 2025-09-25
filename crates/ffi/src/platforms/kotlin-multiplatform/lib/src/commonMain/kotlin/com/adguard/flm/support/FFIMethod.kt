package com.adguard.flm.support

/**
 * Copy for native enum from [FFIMethod](../../../../../../../../../flm_native_interface.h).
 * These 2 enums must be synchronized.
 */
internal enum class FFIMethod {
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
    GetActiveRulesRaw,
    GetFilterRulesAsStrings,
    SaveRulesToFileBlob,
    GetDisabledRules,
    SetProxyMode,
    GetRulesCount
}
