package com.adguard.flm

import com.adguard.flm.protobuf.ActiveRulesInfo
import com.adguard.flm.protobuf.ActiveRulesInfoRaw
import com.adguard.flm.protobuf.DisabledRulesRaw
import com.adguard.flm.protobuf.FilterGroup
import com.adguard.flm.protobuf.FilterListMetadata
import com.adguard.flm.protobuf.FilterListMetadataWithBody
import com.adguard.flm.protobuf.FilterListRules
import com.adguard.flm.protobuf.FilterListRulesRaw
import com.adguard.flm.protobuf.FilterTag
import com.adguard.flm.protobuf.FullFilterList
import com.adguard.flm.protobuf.PullMetadataResult
import com.adguard.flm.protobuf.RawRequestProxyMode
import com.adguard.flm.protobuf.RulesCountByFilter
import com.adguard.flm.protobuf.StoredFilterMetadata
import com.adguard.flm.protobuf.UpdateResult
/**
 * The FlmAdapter is the main entry point for the Filter List Manager library, providing
 * a comprehensive API for managing filter lists in content-blocking applications.
 *
 * This interface provides functionality for:
 * - Database management (initialization, migration, version control)
 * - Filter list operations (installation, enabling/disabling, updating)
 * - Metadata management (tags, groups, localization)
 * - Custom filter list management (creation, modification, deletion)
 * - Rule management (active rules, disabled rules, rule counts)
 * - Filter list retrieval and search
 *
 * ## Usage
 *
 * 1. Create a configuration:
 * ```kotlin
 * val config = Configuration(
 *     appName = "my-app",
 *     version = "1.0.0",
 *     workingDirectory = context.filesDir.absolutePath,
 * )
 * ```
 *
 * 2. Create the adapter:
 * ```kotlin
 * val adapter = FlmAdapterFactory.create(config)
 * ```
 *
 * 3. Perform operations:
 * ```kotlin
 * adapter.pullMetadata()
 * val groups = adapter.getAllGroups()
 * adapter.enableFilterLists(listOf(1, 2, 3), true)
 * val rules = adapter.getActiveRules()
 * ```
 *
 * 4. Close the adapter when done:
 * ```kotlin
 * adapter.close()
 * ```
 *
 * The FlmAdapter implements [AutoCloseable] and should be properly closed when no longer needed
 * to release native resources.
 */
@OptIn(ExperimentalStdlibApi::class)
interface FlmAdapter : AutoCloseable {

    /**
     * The method "raises" the state of the database to the working state.
     *
     * **If the database doesn't exist:**
     * - Creates database
     * - Rolls up the schema
     * - Rolls migrations
     * - Performs bootstrap.
     */
    fun liftUpDatabase(): Boolean

    /**
     * Gets all tags from DB.
     */
    fun getAllTags(): List<FilterTag>?

    /**
     * Gets all groups from DB.
     */
    fun getAllGroups(): List<FilterGroup>?

    /**
     * Gets absolute path for current database.
     */
    fun getDatabasePath(): String?

    /**
     * Gets version of current database scheme.
     * Can return null if database file exists, but metadata table does not exist.
     */
    fun getDatabaseVersion(): Int?

    /**
     * Changes locale used for filter lists localization.
     * Locale will be used on next `pullMetadata` or `getFilterListById` method call.
     *
     * @param suggestedLocale The locale to use.
     * @return True if the locale was found and changed successfully, false otherwise.
     */
    fun changeLocale(suggestedLocale: String): Boolean?

    /**
     * Makes request to remote server and updates filter tags and groups in DB.
     * Note: Should be used once a week or less frequently.
     *
     * @return Number of added, removed and moved filters.
     */
    fun pullMetadata(): PullMetadataResult?

    /**
     * Toggles `isInstalled` property of filter list.
     *
     * @param ids List of filter IDs.
     * @param isInstalled The new boolean value for the `is_installed` flag.
     * @return SQL's affected rows count.
     */
    fun installFilterLists(ids: List<Int>, isInstalled: Boolean): Long?

    /**
     * Toggles filter lists, using their `filterId`.
     *
     * @param ids List of filter IDs.
     * @param isEnabled Whether this filter list should be enabled.
     * @return SQL's affected rows count.
     */
    fun enableFilterLists(ids: List<Int>, isEnabled: Boolean): Long?

    /**
     * Returns all filter data including its rules by filter ID. Fields `title`, `description` will be
     * localised with selected Locale.
     *
     * @param id Filter ID.
     */
    fun getFullFilterListById(id: Int): FullFilterList?

    /**
     * Returns all stored filters metadata.
     * Fields `title`, `description` will be localised with selected Locale.
     */
    fun getStoredFiltersMetadata(): List<StoredFilterMetadata>?

    /**
     * Returns stored filter metadata by filter ID.
     * Fields `title`, `description` will be localised with selected Locale.
     *
     * @param id Filter ID.
     */
    fun getStoredFilterMetadataById(id: Int): StoredFilterMetadata?

    /**
     * Installs a custom filter list.
     *
     * @param downloadUrl Remote server or a `file://` URL. Can be an empty string.
     * @param isTrusted Whether this filter is considered trusted.
     * @param title Override title. If provided, title from metadata will be ignored.
     * @param description Override description. If provided, description from metadata will be ignored.
     * @return The inserted filter list.
     */
    fun installCustomFilterList(
        downloadUrl: String,
        isTrusted: Boolean,
        title: String? = null,
        description: String? = null,
    ): FullFilterList?

    /**
     * Updates filters.
     * Note: Should be used once an hour or less frequently.
     *
     * @param ignoreFiltersExpiration Whether to ignore filter's expire information.
     * @param looseTimeout Not a strict timeout. Pass 0 to disable.
     * @param ignoreFiltersStatus Whether to include disabled filters.
     */
    fun updateFilters(
        ignoreFiltersExpiration: Boolean = false,
        looseTimeout: Int = 0,
        ignoreFiltersStatus: Boolean = false,
    ): UpdateResult?

    /**
     * Tries to update the specified list of filter IDs.
     * Always ignores filters' expires and isEnabled parameters.
     */
    fun forceUpdateFiltersByIds(
        ids: List<Int>,
        looseTimeout: Int = 0,
    ): UpdateResult?

    /**
     * Updates specified filter lists by their IDs.
     */
    fun updateFiltersByIds(
        ids: List<Int>,
        ignoreFiltersExpiration: Boolean = false,
        looseTimeout: Int = 0,
        ignoreFiltersStatus: Boolean = false,
    ): UpdateResult?

    /**
     * Fetches filter list by url and returns its raw metadata.
     *
     * @param url Remote server or a `file://` URL.
     */
    fun fetchFilterListMetadata(url: String): FilterListMetadata?

    /**
     * Fetches filter list by url and returns its raw metadata and body.
     *
     * @param url Remote server or a `file://` URL.
     */
    fun fetchFilterListMetadataWithBody(url: String): FilterListMetadataWithBody?

    /**
     * Deletes custom filter lists, using their filterId.
     *
     * @param ids List of filter IDs.
     * @return SQL's affected rows count.
     */
    fun deleteCustomFilterLists(ids: List<Int>): Long?

    /**
     * Saves custom filter list rules. Note that the filter's timeUpdated will be updated too.
     *
     * @param rules Filter list rules to save.
     */
    fun saveCustomFilterRules(rules: FilterListRules): Boolean

    /**
     * Saves a set of disabled rules for a specific filter list.
     *
     * @param id Filter ID.
     * @param disabledRules List of disabled rules as strings.
     */
    fun saveDisabledRules(id: Int, disabledRules: List<String>): Boolean

    /**
     * Updates custom filter data.
     *
     * @param id Custom filter ID.
     * @param title New title for the filter. Cannot be empty.
     * @param isTrusted New `is_trusted` status for filter.
     */
    fun updateCustomFilterMetadata(id: Int, title: String, isTrusted: Boolean): Boolean?

    /**
     * Installs custom filter from string.
     *
     * @param downloadUrl Download url for filter.
     * @param lastDownloadTime Set filter's lastDownloadTime value.
     * @param isEnabled True if the filter is enabled.
     * @param isTrusted True if the filter is trusted.
     * @param filterBody Filter contents.
     * @param customTitle Filter may have customized title.
     * @param customDescription Filter may have customized description.
     */
    fun installCustomFilterFromString(
        downloadUrl: String,
        lastDownloadTime: Long,
        isEnabled: Boolean,
        isTrusted: Boolean,
        filterBody: String,
        customTitle: String? = null,
        customDescription: String? = null,
    ): FullFilterList?

    /**
     * Gets a list of [ActiveRulesInfo] from filters with isEnabled=true flag.
     */
    fun getActiveRules(): List<ActiveRulesInfo>?

    /**
     * Gets a list of [ActiveRulesInfoRaw] from filters with `filter.is_enabled=true` flag.
     *
     * @param ids If empty, returns all active rules, otherwise returns intersection.
     */
    fun getActiveRulesRaw(ids: List<Int>): List<ActiveRulesInfoRaw>?

    /**
     * Gets a list of [FilterListRulesRaw] structures containing `rules` and `disabledRules` as strings.
     * Only found entities will be returned.
     *
     * @param ids List of filter IDs.
     */
    fun getFilterRulesAsStrings(ids: List<Int>): List<FilterListRulesRaw>?

    /**
     * Saves the rule list for a specific filter to a file, applying disabled rules on the fly.
     *
     * @param id Filter ID.
     * @param filePath Path to the output file.
     */
    fun saveRulesToFileBlob(id: Int, filePath: String): Boolean

    /**
     * Returns lists of disabled rules by list of filter IDs.
     *
     * @param ids List of filter IDs.
     */
    fun getDisabledRules(ids: List<Int>): List<DisabledRulesRaw>?

    /**
     * Sets a new proxy mode. Value will be applied on next method call.
     *
     * @param mode The [RawRequestProxyMode] to set.
     * @param customAddr Optional custom proxy address.
     */
    fun setProxyMode(mode: RawRequestProxyMode, customAddr: String? = null): Boolean

    /**
     * Returns lists of rules count by list of filter IDs.
     *
     * @param ids List of filter IDs.
     */
    fun getRulesCount(ids: List<Int>): List<RulesCountByFilter>?

    /**
     * Verifies the integrity of all stored filter data.
     * Requires integrity_key to be set in Configuration.
     */
    fun verifyIntegrity(): Boolean

    /**
     * Signs all stored filter data using the integrity_key from Configuration.
     */
    fun signAllData(): Boolean

    /**
     * Signs all stored filter data using a new integrity key and updates the stored key.
     *
     * @param integrityKey The new key to use for signing.
     */
    fun signAllDataWithNewKey(integrityKey: String): Boolean
}
