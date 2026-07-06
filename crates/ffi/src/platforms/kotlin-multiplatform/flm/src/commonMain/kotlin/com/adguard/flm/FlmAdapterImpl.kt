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
import com.adguard.flm.protobuf.UpdateFiltersByIdsRequest
import com.adguard.flm.protobuf.UpdateFiltersByIdsResponse
import com.adguard.flm.protobuf.UpdateFiltersRequest
import com.adguard.flm.protobuf.UpdateFiltersResponse
import com.adguard.flm.protobuf.UpdateResult
import com.adguard.flm.protobuf.SignAllDataWithNewKeyRequest
import com.adguard.flm.support.FFIMethod
import com.adguard.flm.support.RustResponseType
import com.squareup.wire.Message

@OptIn(ExperimentalStdlibApi::class)
internal class FlmAdapterImpl(
    private val driver: FilterListManagerDriver
) : FlmAdapter {

    override fun close() {
        driver.close()
    }

    override fun liftUpDatabase(): Boolean {
        val request = EmptyRequest()

        return call(FFIMethod.LiftUpDatabase, request) { result ->
            EmptyResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(EmptyResponse::error)
        } != null
    }

    override fun getAllTags(): List<FilterTag>? {
        val request = EmptyRequest()

        return call(FFIMethod.GetAllTags, request) { result ->
            GetAllTagsResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(GetAllTagsResponse::error)
                ?.tags
        }
    }

    override fun getAllGroups(): List<FilterGroup>? {
        val request = EmptyRequest()

        return call(FFIMethod.GetAllGroups, request) { result ->
            GetAllGroupsResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(GetAllGroupsResponse::error)
                ?.groups
        }
    }

    override fun getDatabasePath(): String? {
        val request = EmptyRequest()

        return call(FFIMethod.GetDatabasePath, request) { result ->
            GetDatabasePathResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(GetDatabasePathResponse::error)
                ?.path
        }
    }

    override fun getDatabaseVersion(): Int? {
        val request = EmptyRequest()

        return call(FFIMethod.GetDatabaseVersion, request) { result ->
            GetDatabaseVersionResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(GetDatabaseVersionResponse::error)
                ?.version
        }
    }

    override fun changeLocale(suggestedLocale: String): Boolean? {
        val request = ChangeLocaleRequest(suggested_locale = suggestedLocale)

        return call(FFIMethod.ChangeLocale, request) { result ->
            ChangeLocaleResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(ChangeLocaleResponse::error)
                ?.success
        }
    }

    override fun pullMetadata(): PullMetadataResult? {
        val request = EmptyRequest()

        return call(FFIMethod.PullMetadata, request) { result ->
            PullMetadataResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(PullMetadataResponse::error)
                ?.result
        }
    }

    override fun installFilterLists(ids: List<Int>, isInstalled: Boolean): Long? {
        val request = InstallFilterListsRequest(ids = ids, is_installed = isInstalled)

        return call(FFIMethod.InstallFilterLists, request) { result ->
            InstallFilterListsResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(InstallFilterListsResponse::error)
                ?.count
        }
    }

    override fun enableFilterLists(ids: List<Int>, isEnabled: Boolean): Long? {
        val request = EnableFilterListsRequest(ids = ids, is_enabled = isEnabled)

        return call(FFIMethod.EnableFilterLists, request) { result ->
            EnableFilterListsResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(EnableFilterListsResponse::error)
                ?.count
        }
    }

    override fun getFullFilterListById(id: Int): FullFilterList? {
        val request = GetFullFilterListByIdRequest(id = id)

        return call(FFIMethod.GetFullFilterListById, request) { result ->
            GetFullFilterListByIdResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(GetFullFilterListByIdResponse::error)
                ?.filter_list
        }
    }

    override fun getStoredFiltersMetadata(): List<StoredFilterMetadata>? {
        val request = EmptyRequest()

        return call(FFIMethod.GetStoredFiltersMetadata, request) { result ->
            GetStoredFiltersMetadataResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(GetStoredFiltersMetadataResponse::error)
                ?.filter_lists
        }
    }

    override fun getStoredFilterMetadataById(id: Int): StoredFilterMetadata? {
        val request = GetStoredFilterMetadataByIdRequest(id = id)

        return call(FFIMethod.GetStoredFilterMetadataById, request) { result ->
            GetStoredFilterMetadataByIdResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(GetStoredFilterMetadataByIdResponse::error)
                ?.filter_list
        }
    }

    override fun installCustomFilterList(
        downloadUrl: String,
        isTrusted: Boolean,
        title: String?,
        description: String?,
    ): FullFilterList? {
        val request = InstallCustomFilterListRequest(
            download_url = downloadUrl,
            is_trusted = isTrusted,
            title = title,
            description = description
        )

        return call(FFIMethod.InstallCustomFilterList, request) { result ->
            InstallCustomFilterListResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(InstallCustomFilterListResponse::error)
                ?.filter_list
        }
    }

    override fun updateFilters(
        ignoreFiltersExpiration: Boolean,
        looseTimeout: Int,
        ignoreFiltersStatus: Boolean,
    ): UpdateResult? {
        val request = UpdateFiltersRequest(
            ignore_filters_expiration = ignoreFiltersExpiration,
            loose_timeout = looseTimeout,
            ignore_filters_status = ignoreFiltersStatus
        )

        return call(FFIMethod.UpdateFilters, request) { result ->
            UpdateFiltersResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(UpdateFiltersResponse::error)
                ?.result
        }
    }

    override fun forceUpdateFiltersByIds(
        ids: List<Int>,
        looseTimeout: Int,
    ): UpdateResult? {
        val request = ForceUpdateFiltersByIdsRequest(
            ids = ids,
            loose_timeout = looseTimeout
        )

        return call(FFIMethod.ForceUpdateFiltersByIds, request) { result ->
            ForceUpdateFiltersByIdsResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(ForceUpdateFiltersByIdsResponse::error)
                ?.result
        }
    }

    override fun updateFiltersByIds(
        ids: List<Int>,
        ignoreFiltersExpiration: Boolean,
        looseTimeout: Int,
        ignoreFiltersStatus: Boolean,
    ): UpdateResult? {
        val request = UpdateFiltersByIdsRequest(
            ids = ids,
            ignore_filters_expiration = ignoreFiltersExpiration,
            loose_timeout = looseTimeout,
            ignore_filters_status = ignoreFiltersStatus
        )

        return call(FFIMethod.UpdateFiltersByIds, request) { result ->
            UpdateFiltersByIdsResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(UpdateFiltersByIdsResponse::error)
                ?.result
        }
    }

    override fun fetchFilterListMetadata(url: String): FilterListMetadata? {
        val request = FetchFilterListMetadataRequest(
            url = url
        )

        return call(FFIMethod.FetchFilterListMetadata, request) { result ->
            FetchFilterListMetadataResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(FetchFilterListMetadataResponse::error)
                ?.metadata
        }
    }

    override fun fetchFilterListMetadataWithBody(url: String): FilterListMetadataWithBody? {
        val request = FetchFilterListMetadataWithBodyRequest(
            url = url
        )

        return call(FFIMethod.FetchFilterListMetadataWithBody, request) { result ->
            FetchFilterListMetadataWithBodyResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(FetchFilterListMetadataWithBodyResponse::error)
                ?.metadata
        }
    }

    override fun deleteCustomFilterLists(ids: List<Int>): Long? {
        val request = DeleteCustomFilterListsRequest(
            ids = ids
        )

        return call(FFIMethod.DeleteCustomFilterLists, request) { result ->
            DeleteCustomFilterListsResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(DeleteCustomFilterListsResponse::error)
                ?.count
        }
    }

    override fun saveCustomFilterRules(rules: FilterListRules): Boolean {
        val request = SaveCustomFilterRulesRequest(
            rules = rules
        )

        return call(FFIMethod.SaveCustomFilterRules, request) { result ->
            EmptyResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(EmptyResponse::error) != null
        } != null
    }

    override fun saveDisabledRules(id: Int, disabledRules: List<String>): Boolean {
        val request = SaveDisabledRulesRequest(
            filter_id = id,
            disabled_rules = disabledRules
        )

        return call(FFIMethod.SaveDisabledRules, request) { result ->
            EmptyResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(EmptyResponse::error) != null
        } != null
    }

    override fun updateCustomFilterMetadata(id: Int, title: String, isTrusted: Boolean): Boolean? {
        val request = UpdateCustomFilterMetadataRequest(
            filter_id = id,
            title = title,
            is_trusted = isTrusted
        )

        return call(FFIMethod.UpdateCustomFilterMetadata, request) { result ->
            UpdateCustomFilterMetadataResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(UpdateCustomFilterMetadataResponse::error)
                ?.success
        }
    }

    override fun installCustomFilterFromString(
        downloadUrl: String,
        lastDownloadTime: Long,
        isEnabled: Boolean,
        isTrusted: Boolean,
        filterBody: String,
        customTitle: String?,
        customDescription: String?,
    ): FullFilterList? {
        val request = InstallCustomFilterFromStringRequest(
            download_url = downloadUrl,
            last_download_time = lastDownloadTime,
            is_enabled = isEnabled,
            is_trusted = isTrusted,
            filter_body = filterBody,
            custom_title = customTitle,
            custom_description = customDescription
        )

        return call(FFIMethod.InstallCustomFilterFromString, request) { result ->
            InstallCustomFilterFromStringResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(InstallCustomFilterFromStringResponse::error)
                ?.filter_list
        }
    }

    override fun getActiveRules(): List<ActiveRulesInfo>? {
        val request = EmptyRequest()

        return call(FFIMethod.GetActiveRules, request) { result ->
            GetActiveRulesResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(GetActiveRulesResponse::error)
                ?.rules
        }
    }

    override fun getActiveRulesRaw(ids: List<Int>): List<ActiveRulesInfoRaw>? {
        val request = GetActiveRulesRawRequest(
            filter_by = ids
        )

        return call(FFIMethod.GetActiveRulesRaw, request) { result ->
            GetActiveRulesRawResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(GetActiveRulesRawResponse::error)
                ?.rules
        }
    }

    override fun getFilterRulesAsStrings(ids: List<Int>): List<FilterListRulesRaw>? {
        val request = GetFilterRulesAsStringsRequest(
            ids = ids
        )

        return call(FFIMethod.GetFilterRulesAsStrings, request) { result ->
            GetFilterRulesAsStringsResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(GetFilterRulesAsStringsResponse::error)
                ?.rules_list
        }
    }

    override fun saveRulesToFileBlob(id: Int, filePath: String): Boolean {
        val request = SaveRulesToFileBlobRequest(
            filter_id = id,
            file_path = filePath
        )

        return call(FFIMethod.SaveRulesToFileBlob, request) { result ->
            EmptyResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(EmptyResponse::error) != null
        } != null
    }

    override fun getDisabledRules(ids: List<Int>): List<DisabledRulesRaw>? {
        val request = GetDisabledRulesRequest(
            ids = ids
        )

        return call(FFIMethod.GetDisabledRules, request) { result ->
            GetDisabledRulesResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(GetDisabledRulesResponse::error)
                ?.rules_raw
        }
    }

    override fun setProxyMode(mode: RawRequestProxyMode, customAddr: String?): Boolean {
        val request = SetProxyModeRequest(
            mode = mode,
            custom_proxy_addr = customAddr ?: ""
        )

        return call(FFIMethod.SetProxyMode, request) { result ->
            EmptyResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(EmptyResponse::error) != null
        } != null
    }

    override fun getRulesCount(ids: List<Int>): List<RulesCountByFilter>? {
        val request = GetRulesCountRequest(
            ids = ids
        )

        return call(FFIMethod.GetRulesCount, request) { result ->
            GetRulesCountResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(GetRulesCountResponse::error)
                ?.rules_count_by_filter
        }
    }

    override fun verifyIntegrity(): Boolean {
        val request = EmptyRequest()

        return call(FFIMethod.VerifyIntegrity, request) { result ->
            EmptyResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(EmptyResponse::error) != null
        } != null
    }

    override fun signAllData(): Boolean {
        val request = EmptyRequest()

        return call(FFIMethod.SignAllData, request) { result ->
            EmptyResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(EmptyResponse::error) != null
        } != null
    }

    override fun signAllDataWithNewKey(integrityKey: String): Boolean {
        val request = SignAllDataWithNewKeyRequest(
            integrity_key = integrityKey
        )

        return call(FFIMethod.SignAllDataWithNewKey, request) { result ->
            EmptyResponse.ADAPTER.decodeFromResponse(result)
                ?.getOrProcessError(EmptyResponse::error) != null
        } != null
    }

    private fun <T> call(
        method: FFIMethod,
        request: Message<*, *>,
        processResponse: (response: RustResponse) -> T
    ): T? {
        var response: RustResponse? = null
        try {
            response = driver.call(method, request.encode())
            if (response.responseType != RustResponseType.RustBuffer) {
                FlmLogger.error("Can't process native response for the '$method' method, response type is ${response.responseType}")
                return null
            }
            if (response.ffiError) {
                val errorMessage = AGOuterError.ADAPTER.decodeFromResponse(response)?.message
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
