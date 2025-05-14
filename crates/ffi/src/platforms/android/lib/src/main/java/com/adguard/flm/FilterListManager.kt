package com.adguard.flm

import com.adguard.flm.exceptions.FilterListManagerException
import com.adguard.flm.jni.FFIMethod
import com.adguard.flm.jni.NativeInterface
import com.adguard.flm.jni.RustResponse
import com.adguard.flm.jni.RustResponseType
import com.adguard.flm.protobuf.*
import com.adguard.flm.protobuf.OuterError.AGOuterError
import com.google.protobuf.MessageLite
import java.io.Closeable
import kotlin.Throws

/**
 * The FilterListManager is the main entry point for the Filter List Manager library, providing
 * a comprehensive API for managing filter lists in content-blocking applications.
 *
 * This class provides functionality for:
 * - Database management (initialization, migration, version control)
 * - Filter list operations (installation, enabling/disabling, updating)
 * - Metadata management (tags, groups, localization)
 * - Custom filter list management (creation, modification, deletion)
 * - Rule management (active rules, disabled rules, rule counts)
 * - Filter list retrieval and search
 *
 * The FilterListManager maintains an internal database of filter lists, their metadata, and rules.
 * It handles all aspects of filter list lifecycle including downloading, parsing, storage, and retrieval.
 *
 * ## Usage
 *
 * 1. Create a configuration:
 * ```kotlin
 * val config = FilterListManager.defaultConfiguration.copy {
 *     appName = "my-app"
 *     version = "1.0.0"
 *     workingDirectory = context.filesDir.absolutePath
 *     // Set other configuration options as needed
 * }
 * ```
 *
 * 2. Initialize the manager:
 * ```kotlin
 * val manager = FilterListManager(config)
 * ```
 *
 * 3. Manually initialize database (only if autoLiftUpDatabase=false):
 * ```kotlin
 * manager.liftUpDatabase() // Not needed by default as it's automatic
 * ```
 *
 * 4. Perform operations:
 * ```kotlin
 * // Update filter metadata
 * manager.pullMetadata()
 *
 * // Get all available filter groups
 * val groups = manager.getAllGroups()
 *
 * // Enable specific filters
 * manager.enableFilterLists(listOf(1, 2, 3), true)
 *
 * // Get active filtering rules
 * val rules = manager.getActiveRules()
 * ```
 *
 * 5. Close the manager when done:
 * ```kotlin
 * manager.close()
 * ```
 *
 * The FilterListManager implements [Closeable] and should be properly closed when no longer needed
 * to release native resources.
 *
 * See README.md for more information.
 *
 * @throws FilterListManagerException if there's an error during initialization or operation
 */
class FilterListManager
@Throws(FilterListManagerException::class)
constructor(configuration: Configuration)
    : Closeable {

    companion object {
        val defaultConfiguration: Configuration
            @Throws(FilterListManagerException::class)
            get() = NativeInterface
                .flmDefaultConfigurationProtobuf()
                .use { result ->
                    when {
                        result.responseType != RustResponseType.RustBuffer ->
                            throw responseTypeRuntimeException(RustResponseType.RustBuffer, result.responseType)

                        result.ffiError ->
                            throw FilterListManagerException(AGOuterError.parseFrom(result.resultData))

                        else -> Configuration.parseFrom(result.resultData)
                    }
                }

        val constants: FilterListManagerConstants
            get() = NativeInterface.flmGetConstants()

        private fun responseTypeRuntimeException(expected: RustResponseType, actual: RustResponseType) =
            RuntimeException("Expected responseType is $expected but $actual received")
    }

    private val handle: Long = NativeInterface
        .flmInitProtobuf(configuration.toByteArray())
        .use {
            when {
                it.responseType != RustResponseType.FLMHandlePointer -> when {
                    it.ffiError ->
                        throw FilterListManagerException(AGOuterError.parseFrom(it.resultData))

                    else ->
                        throw responseTypeRuntimeException(RustResponseType.FLMHandlePointer, it.responseType)
                }

                else -> it.flmHandle
            }
        }

    override fun close() {
        NativeInterface.flmFreeHandle(handle)
    }

    /**
     * The method "raises" the state of the database to the working state.
     *
     * **If the database doesn't exist:**
     * - Creates database
     * - Rolls up the schema
     * - Rolls migrations
     * - Performs bootstrap.
     *
     * ... and so on.
     *
     * @throws FilterListManagerException If there's an error during the database initialization process.
     */
    @Throws(FilterListManagerException::class)
    fun liftUpDatabase() {
        val request = emptyRequest {}

        call(FFIMethod.LiftUpDatabase, request).use { result ->
            EmptyResponse.parseFrom(result.resultData).let { response ->
                if (response.hasError()) {
                    throw FilterListManagerException(response.error)
                }
            }
        }
    }

    /**
     * Gets all tags from DB.
     *
     * @return List of filter tags.
     * @throws FilterListManagerException If there's an error retrieving data from the database.
     */
    @Throws(FilterListManagerException::class)
    fun getAllTags(): List<FilterTag> {
        val request = emptyRequest {}

        return call(FFIMethod.GetAllTags, request).use { result ->
            GetAllTagsResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    else -> response.tagsList
                }
            }
        }
    }

    /**
     * Gets all groups from DB.
     *
     * @return List of filter groups.
     * @throws FilterListManagerException If there's an error retrieving data from the database.
     */
    @Throws(FilterListManagerException::class)
    fun getAllGroups(): List<FilterGroup> {
        val request = emptyRequest {}

        return call(FFIMethod.GetAllGroups, request).use { result ->
            GetAllGroupsResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    else -> response.groupsList
                }
            }
        }
    }

    /**
     * Gets absolute path for current database.
     *
     * @return The absolute path to the database file.
     * @throws FilterListManagerException If there's an error retrieving the path.
     */
    @Throws(FilterListManagerException::class)
    fun getDatabasePath(): String {
        val request = emptyRequest {}

        return call(FFIMethod.GetDatabasePath, request).use { result ->
            GetDatabasePathResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    else -> response.path
                }
            }
        }
    }

    /**
     * Gets version of current database scheme.
     *
     * # Special case
     *
     * Can return null if database file exists, but metadata table does not exist.
     *
     * @return Database version or null
     * @throws FilterListManagerException If there's an error reading the database version from the metadata table.
     */
    @Throws(FilterListManagerException::class)
    fun getDatabaseVersion(): Int? {
        val request = emptyRequest {}

        return call(FFIMethod.GetDatabaseVersion, request).use { result ->
            GetDatabaseVersionResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    response.hasVersion() -> response.version
                    else -> null
                }
            }
        }
    }

    /**
     * Changes locale used for filter lists localization.
     *
     * Locale will be used on next `pullMetadata` or `getFilterListById` method call.
     *
     * Will search `suggestedLocale` in database. If it cannot find exact
     * locale, like `en_GB`, it will try to find language code - `en`. Locales
     * with "-", like `en-GB`, will be normalized to internal format - `en_GB`.
     *
     * @param suggestedLocale The locale to use.
     * @return True if the locale was found and changed successfully, false otherwise.
     * @throws FilterListManagerException If there's an error setting the locale.
     */
    @Throws(FilterListManagerException::class)
    fun changeLocale(suggestedLocale: String): Boolean {
        val request = changeLocaleRequest {
            this.suggestedLocale = suggestedLocale
        }

        return call(FFIMethod.ChangeLocale, request).use { result ->
            ChangeLocaleResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    else -> response.success
                }
            }
        }
    }

    /**
     * Makes request to remote server and updates filter tags and groups in DB.
     * The method is used for creating a database and downloading filters.
     * If the database exists, it attempts to bring it to a state compatible
     * with the current indexes. Also, migrations update will be processed in this method, too.
     * Additionally, the method checks the downloaded indexes for consistency.
     *
     * This method follows the algorithm below:
     *
     * 1. Downloads the filters index (registry).
     * 2. Checks the index consistency:
     *     a. Take filters from the index.
     *     b. For each filter check that `filter.group_id` > 0 and the group is present in the index.
     *     c. For each filter check that tag is present in the index.
     *     d. `filter.name` (title) is not empty.
     *     e. `filter.download_url` must be unique.
     *     f. Everything else is not a critical issue.
     * 3. Opens the database.
     *    a. Check if the database is empty (by the presence of the `filter` table).
     *    b. If empty, apply the schema, save data from indexes, and finish.
     *    c. Otherwise, proceed to the next step.
     * 4. Select all filters from the database and iterate:
     *    (Comparison relies on `filter.id`)
     *    a. If custom filter (`group_id` < 1), continue.
     *    b. If reserved filter (`filter_id` < 1), continue.
     *    c. If filter is enabled but not in the new index, move it to the custom group and change its ID.
     *    d. If the filter is disabled or not installed, delete it.
     *    e. Update the filter with values from the index: `display_number`, `title`, `description`, `homepage`, `expires`, `download_url`, `subscription_url`, `last_update_time`.
     *    f. Mark the filter in the index as processed.
     * 5. Remove old groups/tags/locales.
     * 6. Fill in new groups/tags/locales.
     * 7. Fill in updated filters along with raw filters from the index.
     *
     * Note: Should be used once a week or less frequently.
     * This method uses the locale previously set via [changeLocale].
     *
     * @throws FilterListManagerException If it fails to update filter tags and groups in DB, or if it fails to download the metadata file.
     */
    @Throws(FilterListManagerException::class)
    fun pullMetadata() {
        val request = emptyRequest {}

        call(FFIMethod.PullMetadata, request).use { result ->
            EmptyResponse.parseFrom(result.resultData).let { response ->
                if (response.hasError()) {
                    throw FilterListManagerException(response.error)
                }
            }
        }
    }

    /**
     * Toggles `isInstalled` property of filter list.
     *
     * @param ids List of filter IDs.
     * @param isInstalled The new boolean value for the `is_installed` flag.
     * @return SQL's affected rows count.
     * @throws FilterListManagerException If there's an error updating the database.
     */
    @Throws(FilterListManagerException::class)
    fun installFilterLists(ids: List<Int>, isInstalled: Boolean): Long {
        val request = installFilterListsRequest {
            this.ids.addAll(ids)
            this.isInstalled = isInstalled
        }

        return call(FFIMethod.InstallFilterLists, request).use { result ->
            InstallFilterListsResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    else -> response.count
                }
            }
        }
    }

    /**
     * Toggles filter lists, using their `filterId`.
     *
     * @param ids List of filter IDs.
     * @param isEnabled Whether this filter list should be enabled.
     * @return SQL's affected rows count.
     * @throws FilterListManagerException If there's an error updating the database.
     */
    @Throws(FilterListManagerException::class)
    fun enableFilterLists(ids: List<Int>, isEnabled: Boolean): Long {
        val request = enableFilterListsRequest {
            this.ids.addAll(ids)
            this.isEnabled = isEnabled
        }

        return call(FFIMethod.EnableFilterLists, request).use { result ->
            EnableFilterListsResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    else -> response.count
                }
            }
        }
    }

    /**
     * Returns all filter data including its rules by filter ID. Fields `title`, `description` will be
     * localised with selected Locale.
     *
     * @param id Filter ID.
     * @return Full filter list or null if not found.
     * @throws FilterListManagerException If there's an error retrieving data from the database.
     */
    @Throws(FilterListManagerException::class)
    fun getFullFilterListById(id: Int): FullFilterList? {
        val request = getFullFilterListByIdRequest {
            this.id = id
        }

        return call(FFIMethod.GetFullFilterListById, request).use { result ->
            GetFullFilterListByIdResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    response.hasFilterList() -> response.filterList
                    else -> null
                }
            }
        }
    }

    /**
     * Returns all stored filters metadata. This is the lightweight counterpart of `getFullFilterLists()`.
     * Fields `title`, `description` will be localised with selected Locale.
     *
     * @return List of filter metadata.
     * @throws FilterListManagerException if there's an error in the Rust code.
     */
    @Throws(FilterListManagerException::class)
    fun getStoredFiltersMetadata(): List<StoredFilterMetadata> {
        val request = emptyRequest {}

        return call(FFIMethod.GetStoredFiltersMetadata, request).use { result ->
            GetStoredFiltersMetadataResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    else -> response.filterListsList
                }
            }
        }
    }

    /**
     * Returns stored filter metadata by filter ID. This is the lightweight counterpart of `getFullFilterListById(id)`.
     * Fields `title`, `description` will be localised with selected Locale.
     *
     * @param id Filter ID.
     * @return Filter metadata or null if not found.
     * @throws FilterListManagerException If there's an error retrieving data from the database.
     */
    @Throws(FilterListManagerException::class)
    fun getStoredFilterMetadataById(id: Int): StoredFilterMetadata? {
        val request = getStoredFilterMetadataByIdRequest {
            this.id = id
        }

        return call(FFIMethod.GetStoredFilterMetadataById, request).use { result ->
            GetStoredFilterMetadataByIdResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    response.hasFilterList() -> response.filterList
                    else -> null
                }
            }
        }
    }

    /**
     * Installs a custom filter list.
     *
     * @param downloadUrl Remote server or a `file://` URL. Can be an empty string.
     *                    In that case filter will be local (e.g. won't be updated).
     * @param isTrusted Whether this filter is considered trusted.
     * @param title Override title. If provided, title from metadata will be ignored.
     * @param description Override description. If provided, description from metadata will be ignored.
     * @return The inserted filter list.
     * @throws FilterListManagerException If there's an error during the installation process (e.g., network error, database error).
     */
    @Throws(FilterListManagerException::class)
    fun installCustomFilterList(
        downloadUrl: String,
        isTrusted: Boolean,
        title: String? = null,
        description: String? = null
    ): FullFilterList {
        val request = installCustomFilterListRequest {
            this.downloadUrl = downloadUrl
            this.isTrusted = isTrusted
            title?.let { this.title = it }
            description?.let { this.description = it }
        }

        return call(FFIMethod.InstallCustomFilterList, request).use { result ->
            InstallCustomFilterListResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    else -> response.filterList
                }
            }
        }
    }

    /**
     * Updates filters in multiple steps:
     * 1. Searches for filters ready for update
     * 2. Fetches them
     * 3. Saves lastDownloadTime and updates metadata
     * 4. Collects updated filters
     *
     * @param ignoreFiltersExpiration Whether to ignore filter's expire information
     * @param looseTimeout Not a strict timeout, checked after processing each filter. If the total time exceeds this value,
     *                     filters processing will stop, and the number of unprocessed filters will be set in result value.
     *                     Pass 0 to disable timeout.
     * @param ignoreFiltersStatus Whether to include disabled filters
     * @return An [UpdateResult] object with update information, or null if DB is empty
     * @throws FilterListManagerException If records cannot be retrieved from the database or a common error is encountered
     *
     * Note: Should be used once an hour or less frequently.
     */
    @Throws(FilterListManagerException::class)
    fun updateFilters(
        ignoreFiltersExpiration: Boolean = false,
        looseTimeout: Int = 0,
        ignoreFiltersStatus: Boolean = false
    ): UpdateResult? {
        val request = updateFiltersRequest {
            this.ignoreFiltersExpiration = ignoreFiltersExpiration
            this.looseTimeout = looseTimeout
            this.ignoreFiltersStatus = ignoreFiltersStatus
        }

        return call(FFIMethod.UpdateFilters, request).use { result ->
            UpdateFiltersResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    response.hasResult() -> response.result
                    else -> null
                }
            }
        }
    }

    /**
     * Tries to update the specified list of filter IDs. The logic is the same as in the updateFilters method
     * with exceptions:
     * - Returns null if DB result set is empty
     * - Always ignores filters' expires and isEnabled parameters
     *
     * @param ids List of filter IDs to update
     * @param looseTimeout Not a strict timeout, checked after processing each filter. See updateFilters method
     *                     for more details.
     * @return An [UpdateResult] object with update information, or null if DB result set is empty
     * @throws FilterListManagerException If a critical error occurs during the update process
     *
     * Note: Should be used once an hour or less frequently.
     */
    @Throws(FilterListManagerException::class)
    fun forceUpdateFiltersByIds(
        ids: List<Int>,
        looseTimeout: Int = 0
    ): UpdateResult? {
        val request = forceUpdateFiltersByIdsRequest {
            this.ids.addAll(ids)
            this.looseTimeout = looseTimeout
        }

        return call(FFIMethod.ForceUpdateFiltersByIds, request).use { result ->
            ForceUpdateFiltersByIdsResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    response.hasResult() -> response.result
                    else -> null
                }
            }
        }
    }

    /**
     * Fetches filter list by url and returns its raw metadata.
     *
     * @param url Remote server or a `file://` URL.
     * @return Filter list metadata.
     * @throws FilterListManagerException If there's an error fetching or parsing the metadata (e.g., network error, invalid format).
     */
    @Throws(FilterListManagerException::class)
    fun fetchFilterListMetadata(url: String): FilterListMetadata {
        val request = fetchFilterListMetadataRequest {
            this.url = url
        }

        return call(FFIMethod.FetchFilterListMetadata, request).use { result ->
            FetchFilterListMetadataResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    else -> response.metadata
                }
            }
        }
    }

    /**
     * Fetches filter list by url and returns its raw metadata and body.
     *
     * @param url Remote server or a `file://` URL.
     * @return Filter list metadata and body.
     * @throws FilterListManagerException If there's an error fetching or parsing the data (e.g., network error, invalid format).
     */
    @Throws(FilterListManagerException::class)
    fun fetchFilterListMetadataWithBody(url: String): FilterListMetadataWithBody {
        val request = fetchFilterListMetadataWithBodyRequest {
            this.url = url
        }

        return call(FFIMethod.FetchFilterListMetadataWithBody, request).use { result ->
            FetchFilterListMetadataWithBodyResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    else -> response.metadata
                }
            }
        }
    }

    /**
     * Deletes custom filter lists, using their filterId.
     *
     * @param ids List of filter IDs.
     * @return SQL's affected rows count.
     * @throws FilterListManagerException If there's an error deleting the filters from the database.
     */
    @Throws(FilterListManagerException::class)
    fun deleteCustomFilterLists(ids: List<Int>): Long {
        val request = deleteCustomFilterListsRequest {
            this.ids.addAll(ids)
        }

        return call(FFIMethod.DeleteCustomFilterLists, request).use { result ->
            DeleteCustomFilterListsResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    else -> response.count
                }
            }
        }
    }

    /**
     * Saves custom filter list rules. Note that the filter's timeUpdated will be updated too.
     *
     * @param rules Filter list rules to save (contains filter ID and rules string).
     * @throws FilterListManagerException If the specified filter ID is not found in the database, or it is not from a custom filter.
     */
    @Throws(FilterListManagerException::class)
    fun saveCustomFilterRules(rules: FilterListRules) {
        val request = saveCustomFilterRulesRequest {
            this.rules = rules
        }

        call(FFIMethod.SaveCustomFilterRules, request).use { result ->
            EmptyResponse.parseFrom(result.resultData).let { response ->
                if (response.hasError()) {
                    throw FilterListManagerException(response.error)
                }
            }
        }
    }

    /**
     * Saves a set of disabled rules for a specific filter list.
     *
     * @param id Filter ID.
     * @param disabledRules List of disabled rules as strings.
     * @throws FilterListManagerException If rulesList entity does not exist for passed filterId. This happens because if you want to keep
     *                          disabled filters, you should already have a rulesList entity.
     */
    @Throws(FilterListManagerException::class)
    fun saveDisabledRules(id: Int, disabledRules: List<String>) {
        val request = saveDisabledRulesRequest {
            this.filterId = id
            this.disabledRules.addAll(disabledRules)
        }

        call(FFIMethod.SaveDisabledRules, request).use { result ->
            EmptyResponse.parseFrom(result.resultData).let { response ->
                if (response.hasError()) {
                    throw FilterListManagerException(response.error)
                }
            }
        }
    }

    /**
     * Updates custom filter data.
     *
     * @param id Custom filter ID.
     * @param title New title for the filter. Cannot be empty.
     * @param isTrusted New `is_trusted` status for filter.
     * @return True if the update was successful.
     * @throws FilterListManagerException If the manager couldn't find a filter by `id`, if `id` does not belong to a custom filter,
     *                          or if the provided `title` is empty.
     */
    @Throws(FilterListManagerException::class)
    fun updateCustomFilterMetadata(id: Int, title: String, isTrusted: Boolean): Boolean {
        val request = updateCustomFilterMetadataRequest {
            this.filterId = id
            this.title = title
            this.isTrusted = isTrusted
        }

        return call(FFIMethod.UpdateCustomFilterMetadata, request).use { result ->
            UpdateCustomFilterMetadataResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    else -> response.success
                }
            }
        }
    }

    /**
     * Installs custom filter from string.
     *
     * @param downloadUrl Download url for filter. String will be placed *as is*.
     *                    See [installCustomFilterList] for the format.
     * @param lastDownloadTime Set filter's lastDownloadTime value, which will be added to filter's expires
     *                         and compared to now() at updateFilters method.
     * @param isEnabled True if the filter is enabled.
     * @param isTrusted True if the filter is trusted.
     * @param filterBody Filter contents.
     * @param customTitle Filter may have customized title. See [installCustomFilterList].
     * @param customDescription Filter may have customized description. See [installCustomFilterList].
     * @return The [FullFilterList] object representing the newly installed filter list.
     * @throws FilterListManagerException If `lastDownloadTime` has an unsupported format or if there's another error during installation.
     */
    @Throws(FilterListManagerException::class)
    fun installCustomFilterFromString(
        downloadUrl: String,
        lastDownloadTime: Long,
        isEnabled: Boolean,
        isTrusted: Boolean,
        filterBody: String,
        customTitle: String? = null,
        customDescription: String? = null
    ): FullFilterList {
        val request = installCustomFilterFromStringRequest {
            this.downloadUrl = downloadUrl
            this.lastDownloadTime = lastDownloadTime
            this.isEnabled = isEnabled
            this.isTrusted = isTrusted
            this.filterBody = filterBody
            customTitle?.let { this.customTitle = it }
            customDescription?.let { this.customDescription = it }
        }

        return call(FFIMethod.InstallCustomFilterFromString, request).use { result ->
            InstallCustomFilterFromStringResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    else -> response.filterList
                }
            }
        }
    }

    /**
     * Gets a list of [ActiveRulesInfo] from filters with isEnabled=true flag.
     *
     * @return List of active rules info.
     * @throws FilterListManagerException if there's an error in the Rust code.
     */
    @Throws(FilterListManagerException::class)
    fun getActiveRules(): List<ActiveRulesInfo> {
        val request = emptyRequest {}

        return call(FFIMethod.GetActiveRules, request).use { result ->
            GetActiveRulesResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    else -> response.rulesList
                }
            }
        }
    }

    /**
     * Gets a list of [FilterListRulesRaw] structures containing `rules` and `disabledRules` as strings,
     * directly from database fields.
     *
     * This method acts in the same way as the `IN` database operator. Only found entities will be returned.
     *
     * @param ids List of filter IDs.
     * @return List of filter rules raw data for the found filters.
     * @throws FilterListManagerException if there's an error in the Rust code.
     */
    @Throws(FilterListManagerException::class)
    fun getFilterRulesAsStrings(ids: List<Int>): List<FilterListRulesRaw> {
        val request = getFilterRulesAsStringsRequest {
            this.ids.addAll(ids)
        }

        return call(FFIMethod.GetFilterRulesAsStrings, request).use { result ->
            GetFilterRulesAsStringsResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    else -> response.rulesListList
                }
            }
        }
    }

    /**
     * Reads the rule list for a specific filter in chunks, applying exceptions from the disabledRules list on the fly,
     * and saves the result to the specified file path. The default size of the read buffer is 1 megabyte,
     * but this size can be exceeded if a longer string appears in the list of filter rules.
     *
     * The main purpose of this method is to reduce RAM consumption when reading large size filters.
     *
     * @param id Filter ID.
     * @param filePath Path to the file where the rules should be saved.
     * @throws FilterListManagerException If rule list is not found for the given ID.
     */
    @Throws(FilterListManagerException::class)
    fun saveRulesToFileBlob(id: Int, filePath: String) {
        val request = saveRulesToFileBlobRequest {
            this.filterId = id
            this.filePath = filePath
        }

        call(FFIMethod.SaveRulesToFileBlob, request).use { result ->
            EmptyResponse.parseFrom(result.resultData).let { response ->
                if (response.hasError()) {
                    throw FilterListManagerException(response.error)
                }
            }
        }
    }

    /**
     * Returns lists of disabled rules by list of filter IDs.
     *
     * @param ids List of filter IDs.
     * @return List of disabled rules for the specified filters.
     * @throws FilterListManagerException if there's an error in the Rust code.
     */
    @Throws(FilterListManagerException::class)
    fun getDisabledRules(ids: List<Int>): List<DisabledRulesRaw> {
        val request = getDisabledRulesRequest {
            this.ids.addAll(ids)
        }

        return call(FFIMethod.GetDisabledRules, request).use { result ->
            GetDisabledRulesResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    else -> response.rulesRawList
                }
            }
        }
    }

    /**
     * Sets a new proxy mode. Value will be applied on next method call.
     *
     * @param mode The [RawRequestProxyMode] to set.
     * @param customAddr Optional custom proxy address, used if mode is [RawRequestProxyMode.USE_CUSTOM_PROXY].
     * @throws FilterListManagerException if there's an error in the Rust code.
     */
    @Throws(FilterListManagerException::class)
    fun setProxyMode(mode: RawRequestProxyMode, customAddr: String? = null) {
        val request = setProxyModeRequest {
            this.mode = mode
            customAddr?.let { this.customProxyAddr = customAddr }
        }

        call(FFIMethod.SetProxyMode, request).use { result ->
            EmptyResponse.parseFrom(result.resultData).let { response ->
                if (response.hasError()) {
                    throw FilterListManagerException(response.error)
                }
            }
        }
    }

    /**
     * Returns lists of rules count by list of filter IDs.
     *
     * @param ids List of filter IDs.
     * @return List of rules count by filter for the specified filters.
     * @throws FilterListManagerException if there's an error in the Rust code.
     */
    @Throws(FilterListManagerException::class)
    fun getRulesCount(ids: List<Int>): List<RulesCountByFilter> {
        val request = getRulesCountRequest {
            this.ids.addAll(ids)
        }

        return call(FFIMethod.GetRulesCount, request).use { result ->
            GetRulesCountResponse.parseFrom(result.resultData).let { response ->
                when {
                    response.hasError() -> throw FilterListManagerException(response.error)
                    else -> response.rulesCountByFilterList
                }
            }
        }
    }

    /**
     * Wraps flmCallProtobuf with responseType handling and FFI error checking.
     * @return Closeable RustResponse with result data.
     */
    @Throws(FilterListManagerException::class)
    private fun call(
        method: FFIMethod,
        request: MessageLite
    ): RustResponse {
        return NativeInterface.flmCallProtobuf(handle, method, request.toByteArray()).let { result ->
            when {
                result.responseType != RustResponseType.RustBuffer -> result.use {
                    throw responseTypeRuntimeException(RustResponseType.RustBuffer, result.responseType)
                }

                result.ffiError -> result.use {
                    throw FilterListManagerException(AGOuterError.parseFrom(result.resultData))
                }

                else -> result
            }
        }
    }
}
