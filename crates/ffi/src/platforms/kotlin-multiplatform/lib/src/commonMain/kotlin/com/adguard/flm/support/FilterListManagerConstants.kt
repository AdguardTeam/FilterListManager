package com.adguard.flm.support

/**
 * Copy for native enum from [FilterListManagerConstants](../../../../../../../../../flm_native_interface.h).
 * These 2 types must be synchronized.
 */
data class FilterListManagerConstants(
    /**
     * Filter ID for *User rules* filter
     */
    val userRulesId: Int,

    /**
     * Group ID for special *custom filters group*
     */
    val customGroupId: Int,

    /**
     * Group ID for *special service filters*
     */
    val specialGroupId: Int,

    /**
     * Smallest possible filter_id. You can safely occupy any filter with an id lower than this number.
     * The library is guaranteed to never create a filter with this id
     */
    val smallestFilterId: Int
)
