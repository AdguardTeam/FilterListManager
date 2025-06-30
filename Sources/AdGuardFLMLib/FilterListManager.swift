import Foundation

import SwiftProtobuf
import AdGuardFLM

/// Main endpoint for getting default configuration protobuf.
public func makeDefaultConfiguration() throws -> FilterListManager_Configuration {
    let pointer = flm_default_configuration_protobuf()

    guard let response: UnsafeMutablePointer<RustResponse> = pointer else {
        throw FLMFacadeError.rustResponseAsNullptr
    }

    defer {
        flm_free_response(response)
    }

    let byteData = Data(bytes: response.pointee.result_data, count: response.pointee.result_data_len)

    guard response.pointee.ffi_error == false else {
        let error = try FilterListManager_AGOuterError(serializedBytes: byteData)
        throw AGOuterError(from: error)
    }

    return try FilterListManager_Configuration(serializedBytes: byteData)
}

/// Gets a structure with all FLM public constant values
public func getFLMConstants() -> FilterListManagerConstants {
    return flm_get_constants()
}

public protocol FLMFacadeProtocol {
    func installCustomFilterList(
        downloadUrl: String,
        isTrusted: Bool,
        title: String?,
        description: String?
    ) throws -> FilterListManager_FullFilterList

    func enableFilterLists(ids: [Int32], isEnabled: Bool) throws -> Int64

    func installFilterLists(ids: [Int32], isInstalled: Bool) throws -> Int64

    func deleteCustomFilterLists(ids: [Int32]) throws -> Int64

    func getFullFilterListById(id: Int32) throws -> FilterListManager_FullFilterList?

    func getStoredFiltersMetadata() throws -> [FilterListManager_StoredFilterMetadata]

    func getStoredFilterMetadataById(id: Int32) throws -> FilterListManager_StoredFilterMetadata?

    func saveCustomFilterRules(rules: FilterListManager_FilterListRules) throws

    func saveDisabledRules(id: Int32, disabledRules: [String]) throws

    func updateFilters(
        ignoreFiltersExpiration: Bool,
        looseTimeout: Int32,
        ignoreFiltersStatus: Bool
    ) throws -> FilterListManager_UpdateResult?

    func forceUpdateFiltersByIds(ids: [Int32], looseTimeout: Int32) throws -> FilterListManager_UpdateResult?

    func fetchFilterListMetadata(url: String) throws -> FilterListManager_FilterListMetadata

    func fetchFilterListMetadataWithBody(url: String) throws -> FilterListManager_FilterListMetadataWithBody

    func liftUpDatabase() throws

    func getAllTags() throws -> [FilterListManager_FilterTag]

    func getAllGroups() throws -> [FilterListManager_FilterGroup]

    func changeLocale(suggestedLocale: Locale) throws -> Bool

    func pullMetadata() throws -> FilterListManager_PullMetadataResult

    func updateCustomFilterMetadata(id: Int32, title: String, isTrusted: Bool) throws -> Bool

    func getDatabasePath() throws -> String

    func getDatabaseVersion() throws -> Int32

    func installCustomFilterFromString(
        downloadUrl: String,
        lastDownloadTime: Int64,
        isEnabled: Bool,
        isTrusted: Bool,
        filterBody: String,
        customTitle: String?,
        customDescription: String?
    ) throws -> FilterListManager_FullFilterList

    func getActiveRules() throws -> [FilterListManager_ActiveRulesInfo]

    func getFilterRulesAsStrings(ids: [Int32]) throws -> [FilterListManager_FilterListRulesRaw]

    func saveRulesToFileBlob(id: Int32, filePath: String) throws

    func getDisabledRules(ids: [Int32]) throws -> [FilterListManager_DisabledRulesRaw]

    func setProxyMode(mode: FilterListManager_RawRequestProxyMode, custom_addr: String?) throws

    func getRulesCount(ids: [Int32]) throws -> [FilterListManager_RulesCountByFilter]
}

/// Main FLM facade.
public class FLMFacade: FLMFacadeProtocol {
    private let flm_handle: OpaquePointer

    public init(configuration: FilterListManager_Configuration) throws {
        var newConfData = try configuration.serializedData()

        let response = try newConfData.withUnsafeMutableBytes { rawBufferPointer in
            let ptr = rawBufferPointer.baseAddress
            let len = rawBufferPointer.count

            guard ptr != nil else {
                throw FLMFacadeError.objectIsNotInited
            }

            let rustResponse = flm_init_protobuf(ptr, len)

            // NOTE: Here we do not need to free RustResponse pointer
            // because it is nil-guarded
            guard let rustResponsePtr: UnsafeMutablePointer<RustResponse> = rustResponse else {
                throw FLMFacadeError.rustResponseAsNullptr
            }

            return rustResponsePtr
        }

        defer {
            flm_free_response(response)
        }

        guard response.pointee.ffi_error == false
                && response.pointee.response_type == FLMHandlePointer.rawValue else {
            let data = Data(bytes: response.pointee.result_data, count: response.pointee.result_data_len)

            let error = try FilterListManager_AGOuterError(serializedBytes: data)

            throw AGOuterError(from: error)
        }

        self.flm_handle = OpaquePointer(response.pointee.result_data)
    }

    private func callRust<T: SwiftProtobuf.Message>(method: FFIMethod, message: Message) throws -> T {
        var data = try message.serializedData()

        let bytes = try data.withUnsafeMutableBytes { argsPointer in
            let ptr = argsPointer.baseAddress
            let ptr_len = argsPointer.count

            let rustResponse = flm_call_protobuf(self.flm_handle, method, ptr, ptr_len)

            guard let rustResponsePtr: UnsafeMutablePointer<RustResponse> = rustResponse else {
                throw FLMFacadeError.rustResponseAsNullptr
            }

            defer {
                flm_free_response(rustResponsePtr)
            }

            guard rustResponsePtr.pointee.ffi_error == false else {
                let data = Data(bytes: rustResponsePtr.pointee.result_data, count: rustResponsePtr.pointee.result_data_len)
                let error = try FilterListManager_AGOuterError(serializedBytes: data)

                throw AGOuterError(from: error)
            }

            return Data(bytes: rustResponsePtr.pointee.result_data, count: rustResponsePtr.pointee.result_data_len)
        }

        return try T(serializedBytes: bytes)
    }

    public func installCustomFilterList(downloadUrl: String, isTrusted: Bool, title: String?, description: String?) throws -> FilterListManager_FullFilterList {
        var message = FilterListManager_InstallCustomFilterListRequest()
        message.downloadURL = downloadUrl
        message.isTrusted = isTrusted

        if let name = title {
            message.title = name
        }
        if let desc = description {
            message.description_p = desc
        }

        let response: FilterListManager_InstallCustomFilterListResponse = try callRust(method: InstallCustomFilterList, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.filterList
    }

    public func enableFilterLists(ids: [Int32], isEnabled: Bool) throws -> Int64 {
        var message = FilterListManager_EnableFilterListsRequest()
        message.ids = ids
        message.isEnabled = isEnabled

        let response: FilterListManager_EnableFilterListsResponse = try callRust(method: EnableFilterLists, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.count
    }

    public func installFilterLists(ids: [Int32], isInstalled: Bool) throws -> Int64 {
        var message = FilterListManager_InstallFilterListsRequest()
        message.ids = ids
        message.isInstalled = isInstalled

        let response: FilterListManager_InstallFilterListsResponse = try callRust(method: InstallFilterLists, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.count
    }

    public func deleteCustomFilterLists(ids: [Int32]) throws -> Int64 {
        var message = FilterListManager_DeleteCustomFilterListsRequest()
        message.ids = ids

        let response: FilterListManager_DeleteCustomFilterListsResponse = try callRust(method: DeleteCustomFilterLists, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.count
    }

    public func getFullFilterListById(id: Int32) throws -> FilterListManager_FullFilterList? {
        var message = FilterListManager_GetFullFilterListByIdRequest()
        message.id = id

        let response: FilterListManager_GetFullFilterListByIdResponse = try callRust(method: GetFullFilterListById, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.hasFilterList ? response.filterList : nil
    }

    public func getStoredFiltersMetadata() throws -> [FilterListManager_StoredFilterMetadata] {
        let message = FilterListManager_EmptyRequest()

        let response: FilterListManager_GetStoredFiltersMetadataResponse = try callRust(method: GetStoredFiltersMetadata, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.filterLists
    }

    public func getStoredFilterMetadataById(id: Int32) throws -> FilterListManager_StoredFilterMetadata? {
        var message = FilterListManager_GetStoredFilterMetadataByIdRequest()
        message.id = id

        let response: FilterListManager_GetStoredFilterMetadataByIdResponse = try callRust(method: GetStoredFilterMetadataById, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.hasFilterList ? response.filterList : nil
    }

    public func saveCustomFilterRules(rules: FilterListManager_FilterListRules) throws {
        var message = FilterListManager_SaveCustomFilterRulesRequest()
        message.rules = rules

        let response: FilterListManager_EmptyResponse = try callRust(method: SaveCustomFilterRules, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }
    }

    public func saveDisabledRules(id: Int32, disabledRules: [String]) throws {
        var message = FilterListManager_SaveDisabledRulesRequest()
        message.disabledRules = disabledRules
        message.filterID = id

        let response: FilterListManager_EmptyResponse = try callRust(method: SaveDisabledRules, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }
    }

    public func updateFilters(ignoreFiltersExpiration: Bool, looseTimeout: Int32, ignoreFiltersStatus: Bool) throws -> FilterListManager_UpdateResult? {
        var message = FilterListManager_UpdateFiltersRequest()
        message.ignoreFiltersExpiration = ignoreFiltersExpiration
        message.ignoreFiltersStatus = ignoreFiltersStatus
        message.looseTimeout = looseTimeout

        let response: FilterListManager_UpdateFiltersResponse = try callRust(method: UpdateFilters, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.hasResult ? response.result : nil
    }

    public func forceUpdateFiltersByIds(ids: [Int32], looseTimeout: Int32) throws -> FilterListManager_UpdateResult? {
        var message = FilterListManager_ForceUpdateFiltersByIdsRequest()
        message.ids = ids
        message.looseTimeout = looseTimeout

        let response: FilterListManager_ForceUpdateFiltersByIdsResponse = try callRust(method: ForceUpdateFiltersByIds, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.hasResult ? response.result : nil
    }

    public func fetchFilterListMetadata(url: String) throws -> FilterListManager_FilterListMetadata {
        var message = FilterListManager_FetchFilterListMetadataRequest()
        message.url = url

        let response: FilterListManager_FetchFilterListMetadataResponse = try callRust(method: FetchFilterListMetadata, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.metadata
    }

    public func fetchFilterListMetadataWithBody(url: String) throws -> FilterListManager_FilterListMetadataWithBody {
        var message = FilterListManager_FetchFilterListMetadataWithBodyRequest()
        message.url = url

        let response: FilterListManager_FetchFilterListMetadataWithBodyResponse = try callRust(method: FetchFilterListMetadataWithBody, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.metadata
    }

    public func liftUpDatabase() throws {
        let message = FilterListManager_EmptyRequest()

        let response: FilterListManager_EmptyResponse = try callRust(method: LiftUpDatabase, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }
    }

    public func getAllTags() throws -> [FilterListManager_FilterTag] {
        let message = FilterListManager_EmptyRequest()

        let response: FilterListManager_GetAllTagsResponse = try callRust(method: GetAllTags, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.tags
    }

    public func getAllGroups() throws -> [FilterListManager_FilterGroup] {
        let message = FilterListManager_EmptyRequest()

        let response: FilterListManager_GetAllGroupsResponse = try callRust(method: GetAllGroups, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.groups
    }

    public func changeLocale(suggestedLocale: Locale) throws -> Bool {
        var message = FilterListManager_ChangeLocaleRequest()
        message.suggestedLocale = suggestedLocale.identifier

        let response: FilterListManager_ChangeLocaleResponse = try callRust(method: ChangeLocale, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.success
    }

    public func pullMetadata() throws -> FilterListManager_PullMetadataResult {
        let message = FilterListManager_EmptyRequest()

        let response: FilterListManager_PullMetadataResponse = try callRust(method: PullMetadata, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.result
    }

    public func updateCustomFilterMetadata(id: Int32, title: String, isTrusted: Bool) throws -> Bool {
        var message = FilterListManager_UpdateCustomFilterMetadataRequest()
        message.filterID = id
        message.title = title
        message.isTrusted = isTrusted

        let response: FilterListManager_UpdateCustomFilterMetadataResponse = try callRust(method: UpdateCustomFilterMetadata, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.success
    }

    public func getDatabasePath() throws -> String {
        let message = FilterListManager_EmptyRequest()

        let response: FilterListManager_GetDatabasePathResponse = try callRust(method: GetDatabasePath, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.path
    }

    public func getDatabaseVersion() throws -> Int32 {
        let message = FilterListManager_EmptyRequest()

        let response: FilterListManager_GetDatabaseVersionResponse = try callRust(method: GetDatabaseVersion, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.version
    }

    public func installCustomFilterFromString(
        downloadUrl: String,
        lastDownloadTime: Int64,
        isEnabled: Bool,
        isTrusted: Bool,
        filterBody: String,
        customTitle: String?,
        customDescription: String?
    ) throws -> FilterListManager_FullFilterList {
        var message = FilterListManager_InstallCustomFilterFromStringRequest()
        message.downloadURL = downloadUrl
        message.lastDownloadTime = lastDownloadTime
        message.isEnabled = isEnabled
        message.isTrusted = isTrusted
        message.filterBody = filterBody
        if let title = customTitle {
            message.customTitle = title
        }
        if let description = customDescription {
            message.customDescription = description
        }

        let response: FilterListManager_InstallCustomFilterFromStringResponse = try callRust(method: InstallCustomFilterFromString, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.filterList
    }

    public func getActiveRules() throws -> [FilterListManager_ActiveRulesInfo] {
        let message = FilterListManager_EmptyRequest()

        let response: FilterListManager_GetActiveRulesResponse = try callRust(method: GetActiveRules, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.rules
    }

    public func getFilterRulesAsStrings(ids: [Int32]) throws -> [FilterListManager_FilterListRulesRaw] {
        var message = FilterListManager_GetFilterRulesAsStringsRequest()
        message.ids = ids

        let response: FilterListManager_GetFilterRulesAsStringsResponse = try callRust(method: GetFilterRulesAsStrings, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.rulesList
    }

    public func saveRulesToFileBlob(id: Int32, filePath: String) throws {
        var message = FilterListManager_SaveRulesToFileBlobRequest()
        message.filterID = id
        message.filePath = filePath

        let response: FilterListManager_EmptyResponse = try callRust(method: SaveRulesToFileBlob, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }
    }

    public func getDisabledRules(ids: [Int32]) throws -> [FilterListManager_DisabledRulesRaw] {
        var message = FilterListManager_GetDisabledRulesRequest()
        message.ids = ids

        let response: FilterListManager_GetDisabledRulesResponse = try callRust(method: GetDisabledRules, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.rulesRaw
    }

    public func setProxyMode(mode: FilterListManager_RawRequestProxyMode, custom_addr: String?) throws {
        var message = FilterListManager_SetProxyModeRequest()
        message.mode = mode
        message.customProxyAddr = custom_addr ?? ""

        let response: FilterListManager_EmptyResponse = try callRust(method: SetProxyMode, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }
    }

    public func getRulesCount(ids: [Int32]) throws -> [FilterListManager_RulesCountByFilter] {
        var message = FilterListManager_GetRulesCountRequest()
        message.ids = ids

        let response: FilterListManager_GetRulesCountResponse = try callRust(method: GetRulesCount, message: message)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.rulesCountByFilter
    }

    deinit {
        flm_free_handle(self.flm_handle)
    }
}
