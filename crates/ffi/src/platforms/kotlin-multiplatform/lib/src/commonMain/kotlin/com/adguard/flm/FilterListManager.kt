package com.adguard.flm

import com.adguard.flm.driver.FilterListManagerDriver
import com.adguard.flm.driver.RustResponse
import com.adguard.flm.extensions.decodeFromResponse
import com.adguard.flm.extensions.getOrProcessError
import com.adguard.flm.logging.FlmLogger
import com.adguard.flm.protobuf.AGOuterError
import com.adguard.flm.protobuf.ActiveRulesInfo
import com.adguard.flm.protobuf.ActiveRulesInfoRaw
import com.adguard.flm.protobuf.ChangeLocaleRequest
import com.adguard.flm.protobuf.ChangeLocaleResponse
import com.adguard.flm.protobuf.Configuration
import com.adguard.flm.protobuf.DeleteCustomFilterListsRequest
import com.adguard.flm.protobuf.DeleteCustomFilterListsResponse
import com.adguard.flm.protobuf.DisabledRulesRaw
import com.adguard.flm.protobuf.EmptyRequest
import com.adguard.flm.protobuf.EmptyResponse
import com.adguard.flm.protobuf.EnableFilterListsRequest
import com.adguard.flm.protobuf.EnableFilterListsResponse
import com.adguard.flm.protobuf.FetchFilterListMetadataRequest
import com.adguard.flm.protobuf.FetchFilterListMetadataResponse
import com.adguard.flm.protobuf.FetchFilterListMetadataWithBodyRequest
import com.adguard.flm.protobuf.FetchFilterListMetadataWithBodyResponse
import com.adguard.flm.protobuf.FilterGroup
import com.adguard.flm.protobuf.FilterListMetadata
import com.adguard.flm.protobuf.FilterListMetadataWithBody
import com.adguard.flm.protobuf.FilterListRules
import com.adguard.flm.protobuf.FilterListRulesRaw
import com.adguard.flm.protobuf.FilterTag
import com.adguard.flm.protobuf.ForceUpdateFiltersByIdsRequest
import com.adguard.flm.protobuf.ForceUpdateFiltersByIdsResponse
import com.adguard.flm.protobuf.FullFilterList
import com.adguard.flm.protobuf.GetActiveRulesRawRequest
import com.adguard.flm.protobuf.GetActiveRulesRawResponse
import com.adguard.flm.protobuf.GetActiveRulesResponse
import com.adguard.flm.protobuf.GetAllGroupsResponse
import com.adguard.flm.protobuf.GetAllTagsResponse
import com.adguard.flm.protobuf.GetDatabasePathResponse
import com.adguard.flm.protobuf.GetDatabaseVersionResponse
import com.adguard.flm.protobuf.GetDisabledRulesRequest
import com.adguard.flm.protobuf.GetDisabledRulesResponse
import com.adguard.flm.protobuf.GetFilterRulesAsStringsRequest
import com.adguard.flm.protobuf.GetFilterRulesAsStringsResponse
import com.adguard.flm.protobuf.GetFullFilterListByIdRequest
import com.adguard.flm.protobuf.GetFullFilterListByIdResponse
import com.adguard.flm.protobuf.GetRulesCountRequest
import com.adguard.flm.protobuf.GetRulesCountResponse
import com.adguard.flm.protobuf.GetStoredFilterMetadataByIdRequest
import com.adguard.flm.protobuf.GetStoredFilterMetadataByIdResponse
import com.adguard.flm.protobuf.GetStoredFiltersMetadataResponse
import com.adguard.flm.protobuf.InstallCustomFilterFromStringRequest
import com.adguard.flm.protobuf.InstallCustomFilterFromStringResponse
import com.adguard.flm.protobuf.InstallCustomFilterListRequest
import com.adguard.flm.protobuf.InstallCustomFilterListResponse
import com.adguard.flm.protobuf.InstallFilterListsRequest
import com.adguard.flm.protobuf.InstallFilterListsResponse
import com.adguard.flm.protobuf.PullMetadataResponse
import com.adguard.flm.protobuf.PullMetadataResult
import com.adguard.flm.protobuf.RawRequestProxyMode
import com.adguard.flm.protobuf.RulesCountByFilter
import com.adguard.flm.protobuf.SaveCustomFilterRulesRequest
import com.adguard.flm.protobuf.SaveDisabledRulesRequest
import com.adguard.flm.protobuf.SaveRulesToFileBlobRequest
import com.adguard.flm.protobuf.SetProxyModeRequest
import com.adguard.flm.protobuf.StoredFilterMetadata
import com.adguard.flm.protobuf.UpdateCustomFilterMetadataRequest
import com.adguard.flm.protobuf.UpdateCustomFilterMetadataResponse
import com.adguard.flm.protobuf.UpdateFiltersRequest
import com.adguard.flm.protobuf.UpdateFiltersResponse
import com.adguard.flm.protobuf.UpdateResult
import com.adguard.flm.support.FFIMethod
import com.adguard.flm.support.FilterListManagerConstants
import com.adguard.flm.support.RustResponseType
import pbandk.Message
import pbandk.encodeToByteArray

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
 */
@OptIn(ExperimentalStdlibApi::class)
class FilterListManager private constructor(
    private val driver: FilterListManagerDriver
) : AutoCloseable {

    companion object {

        // Proxy members section

        val defaultConfiguration: Configuration?; get() {
            return FilterListManagerDriver.getDefaultConfiguration()?.use { response ->
                if (response.responseType != RustResponseType.RustBuffer) {
                    FlmLogger.error("The error occurred while collecting default configuration: native part returned ${response.responseType} instead of Rust buffer")
                    return@use null
                }

                if (response.ffiError) {
                    val error = AGOuterError.decodeFromResponse(response) ?: return@use null

                    FlmLogger.error("The error occurred while collecting default configuration: ${error.message}")
                    return@use null
                }

                Configuration.decodeFromResponse(response)
            }
        }

        val constants: FilterListManagerConstants; get() = FilterListManagerDriver.getConstants()



        fun create(configuration: Configuration): FilterListManager? {
            try {
                val driver = FilterListManagerDriver(configuration)
                return FilterListManager(driver)
            } catch (th: Throwable) {
                FlmLogger.error("The error coccurred while creating Filter List manager", th)
            }
            return null
        }
    }



    override fun close() {
        driver.close()
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
     */
    fun liftUpDatabase(): Boolean {
        val request = EmptyRequest()

        return call(FFIMethod.LiftUpDatabase, request) { result ->
            EmptyResponse.decodeFromResponse(result)
                ?.getOrProcessError(EmptyResponse::error)
        } != null
    }

    /**
     * Gets all tags from DB.
     *
     * @return List of filter tags.
     */
    fun getAllTags(): List<FilterTag>? {
        val request = EmptyRequest()

        return call(FFIMethod.GetAllTags, request) { result ->
            GetAllTagsResponse.decodeFromResponse(result)
                ?.getOrProcessError(GetAllTagsResponse::error)
                ?.tags
        }
    }



    /**
     * Gets all groups from DB.
     *
     * @return List of filter groups.
     */
    fun getAllGroups(): List<FilterGroup>? {
        val request = EmptyRequest()

        return call(FFIMethod.GetAllGroups, request) { result ->
            GetAllGroupsResponse.decodeFromResponse(result)
                ?.getOrProcessError(GetAllGroupsResponse::error)
                ?.groups
        }
    }

    /**
     * Gets absolute path for current database.
     *
     * @return The absolute path to the database file.
     */
    fun getDatabasePath(): String? {
        val request = EmptyRequest()

        return call(FFIMethod.GetDatabasePath, request) { result ->
            GetDatabasePathResponse.decodeFromResponse(result)
                ?.getOrProcessError(GetDatabasePathResponse::error)
                ?.path
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
     */
    fun getDatabaseVersion(): Int? {
        val request = EmptyRequest()

        return call(FFIMethod.GetDatabaseVersion, request) { result ->
            GetDatabaseVersionResponse.decodeFromResponse(result)
                ?.getOrProcessError(GetDatabaseVersionResponse::error)
                ?.version
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
     */
    fun changeLocale(suggestedLocale: String): Boolean? {
        val request = ChangeLocaleRequest(suggestedLocale = suggestedLocale)

        return call(FFIMethod.ChangeLocale, request) { result ->
            ChangeLocaleResponse.decodeFromResponse(result)
                ?.getOrProcessError(ChangeLocaleResponse::error)
                ?.success
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
     * @return Number of added, removed and moved filters.
     */
    fun pullMetadata(): PullMetadataResult? {
        val request = EmptyRequest()

        return call(FFIMethod.PullMetadata, request) { result ->
            PullMetadataResponse.decodeFromResponse(result)
                ?.getOrProcessError(PullMetadataResponse::error)
                ?.result
        }
    }

    /**
     * Toggles `isInstalled` property of filter list.
     *
     * @param ids List of filter IDs.
     * @param isInstalled The new boolean value for the `is_installed` flag.
     * @return SQL's affected rows count.
     */
    fun installFilterLists(ids: List<Int>, isInstalled: Boolean): Long? {
        val request = InstallFilterListsRequest(ids = ids, isInstalled = isInstalled)

        return call(FFIMethod.InstallFilterLists, request) { result ->
            InstallFilterListsResponse.decodeFromResponse(result)
                ?.getOrProcessError(InstallFilterListsResponse::error)
                ?.count
        }
    }

    /**
     * Toggles filter lists, using their `filterId`.
     *
     * @param ids List of filter IDs.
     * @param isEnabled Whether this filter list should be enabled.
     * @return SQL's affected rows count.
     */
    fun enableFilterLists(ids: List<Int>, isEnabled: Boolean): Long? {
        val request = EnableFilterListsRequest(ids = ids, isEnabled = isEnabled)

        return call(FFIMethod.EnableFilterLists, request) { result ->
            EnableFilterListsResponse.decodeFromResponse(result)
                ?.getOrProcessError(EnableFilterListsResponse::error)
                ?.count
        }
    }

    /**
     * Returns all filter data including its rules by filter ID. Fields `title`, `description` will be
     * localised with selected Locale.
     *
     * @param id Filter ID.
     * @return Full filter list or null if not found.
     */
    fun getFullFilterListById(id: Int): FullFilterList? {
        val request = GetFullFilterListByIdRequest(id = id)

        return call(FFIMethod.GetFullFilterListById, request) { result ->
            GetFullFilterListByIdResponse.decodeFromResponse(result)
                ?.getOrProcessError(GetFullFilterListByIdResponse::error)
                ?.filterList
        }
    }

    /**
     * Returns all stored filters metadata. This is the lightweight counterpart of `getFullFilterLists()`.
     * Fields `title`, `description` will be localised with selected Locale.
     *
     * @return List of filter metadata.
     */
    fun getStoredFiltersMetadata(): List<StoredFilterMetadata>? {
        val request = EmptyRequest()

        return call(FFIMethod.GetStoredFiltersMetadata, request) { result ->
            GetStoredFiltersMetadataResponse.decodeFromResponse(result)
                ?.getOrProcessError(GetStoredFiltersMetadataResponse::error)
                ?.filterLists
        }
    }

    /**
     * Returns stored filter metadata by filter ID. This is the lightweight counterpart of `getFullFilterListById(id)`.
     * Fields `title`, `description` will be localised with selected Locale.
     *
     * @param id Filter ID.
     * @return Filter metadata or null if not found.
     */
    fun getStoredFilterMetadataById(id: Int): StoredFilterMetadata? {
        val request = GetStoredFilterMetadataByIdRequest(id = id)

        return call(FFIMethod.GetStoredFilterMetadataById, request) { result ->
            GetStoredFilterMetadataByIdResponse.decodeFromResponse(result)
                ?.getOrProcessError(GetStoredFilterMetadataByIdResponse::error)
                ?.filterList
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
     */
    fun installCustomFilterList(
        downloadUrl: String,
        isTrusted: Boolean,
        title: String? = null,
        description: String? = null
    ): FullFilterList? {
        val request = InstallCustomFilterListRequest(
            downloadUrl = downloadUrl,
            isTrusted = isTrusted,
            title = title,
            description = description
        )

        return call(FFIMethod.InstallCustomFilterList, request) { result ->
            InstallCustomFilterListResponse.decodeFromResponse(result)
                ?.getOrProcessError(InstallCustomFilterListResponse::error)
                ?.filterList
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
     *
     * Note: Should be used once an hour or less frequently.
     */
    fun updateFilters(
        ignoreFiltersExpiration: Boolean = false,
        looseTimeout: Int = 0,
        ignoreFiltersStatus: Boolean = false
    ): UpdateResult? {
        val request = UpdateFiltersRequest(
            ignoreFiltersExpiration = ignoreFiltersExpiration,
            looseTimeout = looseTimeout,
            ignoreFiltersStatus = ignoreFiltersStatus
        )

        return call(FFIMethod.UpdateFilters, request) { result ->
            UpdateFiltersResponse.decodeFromResponse(result)
                ?.getOrProcessError(UpdateFiltersResponse::error)
                ?.result
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
     *
     * Note: Should be used once an hour or less frequently.
     */
    fun forceUpdateFiltersByIds(
        ids: List<Int>,
        looseTimeout: Int = 0
    ): UpdateResult? {
        val request = ForceUpdateFiltersByIdsRequest(
            ids = ids,
            looseTimeout = looseTimeout
        )

        return call(FFIMethod.ForceUpdateFiltersByIds, request) { result ->
            ForceUpdateFiltersByIdsResponse.decodeFromResponse(result)
                ?.getOrProcessError(ForceUpdateFiltersByIdsResponse::error)
                ?.result
        }
    }

    /**
     * Fetches filter list by url and returns its raw metadata.
     *
     * @param url Remote server or a `file://` URL.
     * @return Filter list metadata.
     */
    fun fetchFilterListMetadata(url: String): FilterListMetadata? {
        val request = FetchFilterListMetadataRequest(
            url = url
        )

        return call(FFIMethod.FetchFilterListMetadata, request) { result ->
            FetchFilterListMetadataResponse.decodeFromResponse(result)
                ?.getOrProcessError(FetchFilterListMetadataResponse::error)
                ?.metadata
        }
    }

    /**
     * Fetches filter list by url and returns its raw metadata and body.
     *
     * @param url Remote server or a `file://` URL.
     * @return Filter list metadata and body.
     */
    fun fetchFilterListMetadataWithBody(url: String): FilterListMetadataWithBody? {
        val request = FetchFilterListMetadataWithBodyRequest(
            url = url
        )

        return call(FFIMethod.FetchFilterListMetadataWithBody, request) { result ->
            FetchFilterListMetadataWithBodyResponse.decodeFromResponse(result)
                ?.getOrProcessError(FetchFilterListMetadataWithBodyResponse::error)
                ?.metadata
        }
    }

    /**
     * Deletes custom filter lists, using their filterId.
     *
     * @param ids List of filter IDs.
     * @return SQL's affected rows count.
     */
    fun deleteCustomFilterLists(ids: List<Int>): Long? {
        val request = DeleteCustomFilterListsRequest(
            ids = ids
        )

        return call(FFIMethod.DeleteCustomFilterLists, request) { result ->
            DeleteCustomFilterListsResponse.decodeFromResponse(result)
                ?.getOrProcessError(DeleteCustomFilterListsResponse::error)
                ?.count
        }
    }

    /**
     * Saves custom filter list rules. Note that the filter's timeUpdated will be updated too.
     *
     * @param rules Filter list rules to save (contains filter ID and rules string).
     */
    fun saveCustomFilterRules(rules: FilterListRules): Boolean {
        val request = SaveCustomFilterRulesRequest(
            rules = rules
        )

        return call(FFIMethod.SaveCustomFilterRules, request) { result ->
            EmptyResponse.decodeFromResponse(result)
                ?.getOrProcessError(EmptyResponse::error) != null
        } != null
    }

    /**
     * Saves a set of disabled rules for a specific filter list.
     *
     * @param id Filter ID.
     * @param disabledRules List of disabled rules as strings.
     */
    fun saveDisabledRules(id: Int, disabledRules: List<String>): Boolean {
        val request = SaveDisabledRulesRequest(
            filterId = id,
            disabledRules = disabledRules
        )

        return call(FFIMethod.SaveDisabledRules, request) { result ->
            EmptyResponse.decodeFromResponse(result)
                ?.getOrProcessError(EmptyResponse::error) != null
        } != null
    }

    /**
     * Updates custom filter data.
     *
     * @param id Custom filter ID.
     * @param title New title for the filter. Cannot be empty.
     * @param isTrusted New `is_trusted` status for filter.
     * @return True if the update was successful.
     */
    fun updateCustomFilterMetadata(id: Int, title: String, isTrusted: Boolean): Boolean? {
        val request = UpdateCustomFilterMetadataRequest(
            filterId = id,
            title = title,
            isTrusted = isTrusted
        )

        return call(FFIMethod.UpdateCustomFilterMetadata, request) { result ->
            UpdateCustomFilterMetadataResponse.decodeFromResponse(result)
                ?.getOrProcessError(UpdateCustomFilterMetadataResponse::error)
                ?.success
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
     */
    fun installCustomFilterFromString(
        downloadUrl: String,
        lastDownloadTime: Long,
        isEnabled: Boolean,
        isTrusted: Boolean,
        filterBody: String,
        customTitle: String? = null,
        customDescription: String? = null
    ): FullFilterList? {
        val request = InstallCustomFilterFromStringRequest(
            downloadUrl = downloadUrl,
            lastDownloadTime = lastDownloadTime,
            isEnabled = isEnabled,
            isTrusted = isTrusted,
            filterBody = filterBody,
            customTitle = customTitle,
            customDescription = customDescription
        )

        return call(FFIMethod.InstallCustomFilterFromString, request) { result ->
            InstallCustomFilterFromStringResponse.decodeFromResponse(result)
                ?.getOrProcessError(InstallCustomFilterFromStringResponse::error)
                ?.filterList
        }
    }

    /**
     * Gets a list of [ActiveRulesInfo] from filters with isEnabled=true flag.
     *
     * @return List of active rules info.
     */
    fun getActiveRules(): List<ActiveRulesInfo>? {
        val request = EmptyRequest()

        return call(FFIMethod.GetActiveRules, request) { result ->
            GetActiveRulesResponse.decodeFromResponse(result)
                ?.getOrProcessError(GetActiveRulesResponse::error)
                ?.rules
        }
    }

    /**
     * Gets a list of [ActiveRulesInfoRaw] from filters with `filter.is_enabled=true` flag.
     *
     * @param ids If empty, returns all active rules, otherwise returns intersection between `filter_by` and all active rules
     * @return List of raw active rules info.
     */
    fun getActiveRulesRaw(ids: List<Int>): List<ActiveRulesInfoRaw>? {
        val request = GetActiveRulesRawRequest(
            filterBy = ids
        )

        return call(FFIMethod.GetActiveRulesRaw, request) { result ->
            GetActiveRulesRawResponse.decodeFromResponse(result)
                ?.getOrProcessError(GetActiveRulesRawResponse::error)
                ?.rules
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
     */
    fun getFilterRulesAsStrings(ids: List<Int>): List<FilterListRulesRaw>? {
        val request = GetFilterRulesAsStringsRequest(
            ids = ids
        )

        return call(FFIMethod.GetFilterRulesAsStrings, request) { result ->
            GetFilterRulesAsStringsResponse.decodeFromResponse(result)
                ?.getOrProcessError(GetFilterRulesAsStringsResponse::error)
                ?.rulesList
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
     */
    fun saveRulesToFileBlob(id: Int, filePath: String): Boolean {
        val request = SaveRulesToFileBlobRequest(
            filterId = id,
            filePath = filePath
        )

        return call(FFIMethod.SaveRulesToFileBlob, request) { result ->
            EmptyResponse.decodeFromResponse(result)
                ?.getOrProcessError(EmptyResponse::error) != null
        } != null
    }

    /**
     * Returns lists of disabled rules by list of filter IDs.
     *
     * @param ids List of filter IDs.
     * @return List of disabled rules for the specified filters.
     */
    fun getDisabledRules(ids: List<Int>): List<DisabledRulesRaw>? {
        val request = GetDisabledRulesRequest(
            ids = ids
        )

        return call(FFIMethod.GetDisabledRules, request) { result ->
            GetDisabledRulesResponse.decodeFromResponse(result)
                ?.getOrProcessError(GetDisabledRulesResponse::error)
                ?.rulesRaw
        }
    }

    /**
     * Sets a new proxy mode. Value will be applied on next method call.
     *
     * @param mode The [RawRequestProxyMode] to set.
     * @param customAddr Optional custom proxy address, used if mode is [RawRequestProxyMode.USE_CUSTOM_PROXY].
     */
    fun setProxyMode(mode: RawRequestProxyMode, customAddr: String? = null): Boolean {
        val request = SetProxyModeRequest(
            mode = mode,
            customProxyAddr = customAddr ?: ""
        )

        return call(FFIMethod.SetProxyMode, request) { result ->
            EmptyResponse.decodeFromResponse(result)
                ?.getOrProcessError(EmptyResponse::error) != null
        } != null
    }

    /**
     * Returns lists of rules count by list of filter IDs.
     *
     * @param ids List of filter IDs.
     * @return List of rules count by filter for the specified filters.
     */
    fun getRulesCount(ids: List<Int>): List<RulesCountByFilter>? {
        val request = GetRulesCountRequest(
            ids = ids
        )

        return call(FFIMethod.GetRulesCount, request) { result ->
            GetRulesCountResponse.decodeFromResponse(result)
                ?.getOrProcessError(GetRulesCountResponse::error)
                ?.rulesCountByFilter
        }
    }

    private fun <T> call(
        method: FFIMethod,
        request: Message,
        processResponse: (response: RustResponse) -> T
    ): T? {
        var response: RustResponse? = null
        try {
            response = driver.call(method, request.encodeToByteArray())
            if (response.responseType != RustResponseType.RustBuffer) {
                FlmLogger.error("Can't process native response for the '$method' method, response type is ${response.responseType}")
                return null
            }
            if (response.ffiError) {
                val errorMessage = AGOuterError.decodeFromResponse(response)?.message
                FlmLogger.error("Can't process native response for the '$method' method, error message: $errorMessage")
                return null
            }

            return processResponse(response)
        } catch (th: Throwable) {
            FlmLogger.error("The error occurred while calling native method ' ${method}'", th)
            return null
        } finally {
            response?.close()
        }
    }
}
