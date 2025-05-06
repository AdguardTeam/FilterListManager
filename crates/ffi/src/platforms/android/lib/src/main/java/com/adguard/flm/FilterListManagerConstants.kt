package com.adguard.flm

/**
 * Structure used for passing constants through FFI
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
