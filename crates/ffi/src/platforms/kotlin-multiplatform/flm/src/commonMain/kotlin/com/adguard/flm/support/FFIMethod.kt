package com.adguard.flm.support

/**
 * Copy for native enum from [FFIMethod](../../../../../../../../../flm_native_interface.h).
 * These 2 enums must be synchronized.
 */
internal enum class FFIMethod {
    InstallCustomFilterList,       // 0
    EnableFilterLists,             // 1
    InstallFilterLists,            // 2
    DeleteCustomFilterLists,       // 3
    GetFullFilterListById,         // 4
    GetStoredFiltersMetadata,      // 5
    GetStoredFilterMetadataById,   // 6
    SaveCustomFilterRules,         // 7
    SaveDisabledRules,             // 8
    UpdateFilters,                 // 9
    ForceUpdateFiltersByIds,       // 10
    UpdateFiltersByIds,            // 11
    FetchFilterListMetadata,       // 12
    FetchFilterListMetadataWithBody, // 13
    LiftUpDatabase,                // 14
    GetAllTags,                    // 15
    GetAllGroups,                  // 16
    ChangeLocale,                  // 17
    PullMetadata,                  // 18
    UpdateCustomFilterMetadata,    // 19
    GetDatabasePath,               // 20
    GetDatabaseVersion,            // 21
    InstallCustomFilterFromString, // 22
    GetActiveRules,                // 23
    GetActiveRulesRaw,             // 24
    GetFilterRulesAsStrings,       // 25
    SaveRulesToFileBlob,           // 26
    GetDisabledRules,              // 27
    SetProxyMode,                  // 28
    GetRulesCount,                 // 29
    VerifyIntegrity,               // 30
    SignAllData,                   // 31
    SignAllDataWithNewKey,         // 32
}
