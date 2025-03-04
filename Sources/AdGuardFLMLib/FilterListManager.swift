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

    func getFullFilterListsById(id: Int32) throws -> FilterListManager_FullFilterList

    func getStoredFiltersMetadata() throws -> [FilterListManager_StoredFilterMetadata]

    func getStoredFiltersMetadataById(id: Int32) throws -> FilterListManager_StoredFilterMetadata

    func saveCustomFilterRules(rules: FilterListManager_FilterListRules) throws

    func saveDisabledRules(id: Int32, disabledRules: [String]) throws

    func updateFilters(
        ignoreFiltersExpiration: Bool,
        looseTimeout: Int32,
        ignoreFiltersStatus: Bool
    ) throws -> FilterListManager_UpdateResult

    func forceUpdateFiltersByIds(ids: [Int32], looseTimeout: Int32) throws -> FilterListManager_UpdateResult

    func fetchFiltersListMetadata(url: String) throws -> FilterListManager_FilterListMetadata

    func fetchFiltersListMetadataWithBody(url: String) throws -> FilterListManager_FilterListMetadataWithBody

    func liftUpDatabase() throws

    func getAllTags() throws -> [FilterListManager_FilterTag]

    func getAllGroups() throws -> [FilterListManager_FilterGroup]

    func changeLocale(suggestedLocale: Locale) throws -> Bool

    func pullMetadata() throws

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

    func getFilterRulesAsStrings(ids: [Int32]) throws -> [FilterListManager_FilterListRulesRaw]

    func saveRulesToFileBlob(id: Int32, filePath: String) throws

    func getDisabledRules(ids: [Int32]) throws -> [FilterListManager_DisabledRulesRaw]

    func setProxyMode(mode: FilterListManager_RawRequestProxyMode, custom_addr: String?) throws
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

        guard response.pointee.ffi_error == false else {
            let data = Data(bytes: response.pointee.result_data, count: response.pointee.result_data_len)

            let error = try FilterListManager_AGOuterError(serializedBytes: data)

            throw AGOuterError(from: error)
        }

        self.flm_handle = OpaquePointer(response.pointee.result_data)
    }

    private func callRust(method: FFIMethod, message: Message) throws -> Data {
        var data = try message.serializedData()

        return try data.withUnsafeMutableBytes { argsPointer in
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

        let bytes = try callRust(method: InstallCustomFilterList, message: message)
        let response = try FilterListManager_InstallCustomFilterListResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.filterList
    }

    public func enableFilterLists(ids: [Int32], isEnabled: Bool) throws -> Int64 {
        var message = FilterListManager_EnableFilterListsRequest()
        message.ids = ids
        message.isEnabled = isEnabled

        let bytes = try callRust(method: EnableFilterLists, message: message)
        let response = try FilterListManager_EnableFilterListsResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.count
    }

    public func installFilterLists(ids: [Int32], isInstalled: Bool) throws -> Int64 {
        var message = FilterListManager_InstallFilterListsRequest()
        message.ids = ids
        message.isInstalled = isInstalled

        let bytes = try callRust(method: InstallFilterLists, message: message)
        let response = try FilterListManager_InstallFilterListsResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.count
    }

    public func deleteCustomFilterLists(ids: [Int32]) throws -> Int64 {
        var message = FilterListManager_DeleteCustomFilterListsRequest()
        message.ids = ids

        let bytes = try callRust(method: DeleteCustomFilterLists, message: message)
        let response = try FilterListManager_DeleteCustomFilterListsResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.count
    }

    public func getFullFilterListsById(id: Int32) throws -> FilterListManager_FullFilterList {
        var message = FilterListManager_GetFullFilterListByIdRequest()
        message.id = id

        let bytes = try callRust(method: GetFullFilterListById, message: message)
        let response = try FilterListManager_GetFullFilterListByIdResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.filterList
    }

    public func getStoredFiltersMetadata() throws -> [FilterListManager_StoredFilterMetadata] {
        let message = FilterListManager_EmptyRequest()

        let bytes = try callRust(method: GetStoredFiltersMetadata, message: message)
        let response = try FilterListManager_GetStoredFiltersMetadataResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.filterLists
    }

    public func getStoredFiltersMetadataById(id: Int32) throws -> FilterListManager_StoredFilterMetadata {
        var message = FilterListManager_GetStoredFiltersMetadataByIdRequest()
        message.id = id

        let bytes = try callRust(method: GetStoredFilterMetadataById, message: message)
        let response = try FilterListManager_GetStoredFilterMetadataByIdResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.filterList
    }

    public func saveCustomFilterRules(rules: FilterListManager_FilterListRules) throws {
        var message = FilterListManager_SaveCustomFilterRulesRequest()
        message.rules = rules

        let bytes = try callRust(method: SaveCustomFilterRules, message: message)
        let response = try FilterListManager_EmptyResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }
    }

    public func saveDisabledRules(id: Int32, disabledRules: [String]) throws {
        var message = FilterListManager_SaveDisabledRulesRequest()
        message.disabledRules = disabledRules
        message.filterID = id

        let bytes = try callRust(method: SaveDisabledRules, message: message)
        let response = try FilterListManager_EmptyResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }
    }

    public func updateFilters(ignoreFiltersExpiration: Bool, looseTimeout: Int32, ignoreFiltersStatus: Bool) throws -> FilterListManager_UpdateResult {
        var message = FilterListManager_UpdateFiltersRequest()
        message.ignoreFiltersExpiration = ignoreFiltersExpiration
        message.ignoreFiltersStatus = ignoreFiltersStatus
        message.looseTimeout = looseTimeout

        let bytes = try callRust(method: UpdateFilters, message: message)
        let response = try FilterListManager_UpdateFiltersResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.result
    }

    public func forceUpdateFiltersByIds(ids: [Int32], looseTimeout: Int32) throws -> FilterListManager_UpdateResult {
        var message = FilterListManager_ForceUpdateFiltersByIdsRequest()
        message.ids = ids
        message.looseTimeout = looseTimeout

        let bytes = try callRust(method: ForceUpdateFiltersByIds, message: message)
        let response = try FilterListManager_ForceUpdateFiltersByIdsResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.result
    }

    public func fetchFiltersListMetadata(url: String) throws -> FilterListManager_FilterListMetadata {
        var message = FilterListManager_FetchFilterListMetadataRequest()
        message.url = url

        let bytes = try callRust(method: FetchFilterListMetadata, message: message)
        let response = try FilterListManager_FetchFilterListMetadataResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.metadata
    }

    public func fetchFiltersListMetadataWithBody(url: String) throws -> FilterListManager_FilterListMetadataWithBody {
        var message = FilterListManager_FetchFilterListMetadataWithBodyRequest()
        message.url = url

        let bytes = try callRust(method: FetchFilterListMetadataWithBody, message: message)
        let response = try FilterListManager_FetchFilterListMetadataWithBodyResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.metadata
    }

    public func liftUpDatabase() throws {
        let message = FilterListManager_EmptyRequest()

        let bytes = try callRust(method: LiftUpDatabase, message: message)
        let response = try FilterListManager_EmptyResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }
    }

    public func getAllTags() throws -> [FilterListManager_FilterTag] {
        let message = FilterListManager_EmptyRequest()

        let bytes = try callRust(method: GetAllTags, message: message)
        let response = try FilterListManager_GetAllTagsResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.tags
    }

    public func getAllGroups() throws -> [FilterListManager_FilterGroup] {
        let message = FilterListManager_EmptyRequest()

        let bytes = try callRust(method: GetAllGroups, message: message)
        let response = try FilterListManager_GetAllGroupsResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.groups
    }

    public func changeLocale(suggestedLocale: Locale) throws -> Bool {
        var message = FilterListManager_ChangeLocaleRequest()
        message.suggestedLocale = suggestedLocale.identifier

        let bytes = try callRust(method: ChangeLocale, message: message)
        let response = try FilterListManager_ChangeLocaleResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.success
    }

    public func pullMetadata() throws {
        let message = FilterListManager_EmptyRequest()

        let bytes = try callRust(method: PullMetadata, message: message)
        let response = try FilterListManager_EmptyResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }
    }

    public func updateCustomFilterMetadata(id: Int32, title: String, isTrusted: Bool) throws -> Bool {
        var message = FilterListManager_UpdateCustomFilterMetadataRequest()
        message.filterID = id
        message.title = title
        message.isTrusted = isTrusted

        let bytes = try callRust(method: UpdateCustomFilterMetadata, message: message)
        let response = try FilterListManager_UpdateCustomFilterMetadataResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.success
    }

    public func getDatabasePath() throws -> String {
        let message = FilterListManager_EmptyRequest()

        let bytes = try callRust(method: GetDatabasePath, message: message)

        let response = try FilterListManager_GetDatabasePathResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.path
    }

    public func getDatabaseVersion() throws -> Int32 {
        let message = FilterListManager_EmptyRequest()

        let bytes = try callRust(method: GetDatabaseVersion, message: message)
        let response = try FilterListManager_GetDatabaseVersionResponse(serializedBytes: bytes)

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

        let bytes = try callRust(method: InstallCustomFilterFromString, message: message)
        let response = try FilterListManager_InstallCustomFilterFromStringResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.filterList
    }

    public func getFilterRulesAsStrings(ids: [Int32]) throws -> [FilterListManager_FilterListRulesRaw] {
        var message = FilterListManager_GetFilterRulesAsStringsRequest()
        message.ids = ids

        let bytes = try callRust(method: GetFilterRulesAsStrings, message: message)
        let response = try FilterListManager_GetFilterRulesAsStringsResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.rulesList
    }

    public func saveRulesToFileBlob(id: Int32, filePath: String) throws {
        var message = FilterListManager_SaveRulesToFileBlobRequest()
        message.filterID = id
        message.filePath = filePath

        let bytes = try callRust(method: SaveRulesToFileBlob, message: message)
        let response = try FilterListManager_EmptyResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }
    }

    public func getDisabledRules(ids: [Int32]) throws -> [FilterListManager_DisabledRulesRaw] {
        var message = FilterListManager_GetDisabledRulesRequest()
        message.ids = ids

        let bytes = try callRust(method: GetDisabledRules, message: message)
        let response = try FilterListManager_GetDisabledRulesResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.rulesRaw
    }

    public func setProxyMode(mode: FilterListManager_RawRequestProxyMode, custom_addr: String?) throws {
        var message = FilterListManager_SetProxyModeRequest()
        message.mode = mode
        message.customProxyAddr = custom_addr ?? ""

        let bytes = try callRust(method: SetProxyMode, message: message)
        let response = try FilterListManager_EmptyResponse(serializedBytes: bytes)

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }
    }

    deinit {
        flm_free_handle(self.flm_handle)
    }
}
