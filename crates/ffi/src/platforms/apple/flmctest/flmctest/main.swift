//
//  main.swift
//  flmctest
//
//  Created by Vladimir Bachinskiy on 05.09.2024.
//

import Foundation
import SwiftProtobuf

/// REQUIRED
/// Error container
struct AGOuterError: Error, LocalizedError {
    /// String representation of error
    let message: String
    /// Error type with details
    public let variant: AGOuterErrorVariant

    public var localizedDescription: String {
        self.message
    }

    public var errorDescription: String? {
        self.message
    }

    init(from: FilterListManager_AGOuterError) {
        var error = from;
        self.variant = AGOuterErrorVariant(from: &error)
        self.message = error.message
    }
}

/// REQUIRED
enum AGOuterErrorVariant: Error {
    case CannotOpenDatabase
    case NotADatabase
    case DiskFull
    case EntityNotFound(Int64)
    case PathNotFound(String)
    case PathHasDeniedPermission(String)
    case PathAlreadyExists(String)
    case TimedOut
    case HttpClientNetworkError
    case HttpStrict200Response(UInt32, String)
    case HttpClientBodyRecoveryFailed
    case FilterContentIsLikelyNotAFilter
    case FilterParserError
    case FieldIsEmpty(String)
    case DatabaseBusy
    case Mutex
    case Other
}

extension AGOuterErrorVariant {
    init(from: inout FilterListManager_AGOuterError) {
        guard let thisCase = from.error else {
            from.message = "Error variant is nil";

            self = Self.Other

            return;
        }

        switch thisCase {
        case .cannotOpenDatabase:
            self = Self.CannotOpenDatabase
        case .notADatabase:
            self = Self.NotADatabase
        case .diskFull:
            self = Self.DiskFull
        case .entityNotFound(let container):
            self = Self.EntityNotFound(container.entityID)
        case .pathNotFound(let container):
            self = Self.PathNotFound(container.path)
        case .pathHasDeniedPermission(let container):
            self = Self.PathHasDeniedPermission(container.path)
        case .pathAlreadyExists(let container):
            self = Self.PathAlreadyExists(container.path)
        case .timedOut(_):
            self = Self.TimedOut
        case .httpClientNetworkError(_):
            self = Self.HttpClientNetworkError
        case .httpStrict200Response(let container):
            self = Self.HttpStrict200Response(container.statusCode, container.url)
        case .httpClientBodyRecoveryFailed(_):
            self = Self.HttpClientBodyRecoveryFailed
        case .filterContentIsLikelyNotAFilter(_):
            self = Self.FilterContentIsLikelyNotAFilter
        case .filterParserError(_):
            self = Self.FilterParserError
        case .fieldIsEmpty(let container):
            self = Self.FieldIsEmpty(container.fieldName)
        case .mutex(_):
            self = Self.Mutex
        case .other(_):
            self = Self.Other
        case .databaseBusy(_):
            self = Self.DatabaseBusy
        }
    }
}

/// REQUIRED
/// Main endpoint for getting default  configuration protobuf
func makeDefaultConfiguration() throws -> FilterListManager_Configuration {
    let pointer = flm_default_configuration_protobuf();

    guard let response: UnsafeMutablePointer<RustResponse> = pointer else {
        throw FLMFacadeError.rustResponseAsNullptr
    }

    defer {
        flm_free_response(response);
    }


    let byteData = Data(bytes: response.pointee.result_data, count: response.pointee.result_data_len);

    guard response.pointee.ffi_error == false else {
        let error = try FilterListManager_AGOuterError(serializedBytes: byteData);
        throw AGOuterError(from: error);
    }

    return try FilterListManager_Configuration(serializedBytes: byteData)
}

/// MAY BE REQUIRED
enum FLMFacadeError: Error {
    case objectIsNotInited
    case noDataOnResponse
    case rustResponseAsNullptr
    // TODO: Will be removed
    case testWillFail
}

func spawnConf() throws -> FilterListManager_Configuration {
    var configuration = try makeDefaultConfiguration();

    configuration.locale = "en";
    configuration.metadataURL = "https://filters.adtidy.org/extension/safari/filters.json";
    configuration.metadataLocalesURL = "https://filters.adtidy.org/extension/safari/filters_i18n.json";
    configuration.workingDirectory = ".";

    return configuration
}

/// REQUIRED
/// Main FLM  facade
class FLMFacade {
    private let flm_handle: UnsafeMutableRawPointer

    init(configuration: FilterListManager_Configuration) throws {
        var newConfData = try configuration.serializedData();

        let response = try newConfData.withUnsafeMutableBytes { rawBufferPointer in
            let ptr = rawBufferPointer.baseAddress
            let len = rawBufferPointer.count

            guard ptr != nil else {
                throw FLMFacadeError.objectIsNotInited
            }

            let rustResponse = flm_init_protobuf(ptr, len);

            // NOTE: Here we do not need to free RustResponse pointer
            // because it is nil-guarded
            guard let rustResponsePtr: UnsafeMutablePointer<RustResponse> = rustResponse else {
                throw FLMFacadeError.rustResponseAsNullptr
            }

            return rustResponsePtr
        }

        defer {
            flm_free_response(response);
        }

        guard response.pointee.ffi_error == false else {
            let data = Data(bytes: response.pointee.result_data, count: response.pointee.result_data_len);

            let error = try FilterListManager_AGOuterError(serializedBytes: data);

            throw AGOuterError(from: error)
        }

        self.flm_handle = response.pointee.result_data;
    }

    private func callRust(method: FFIMethod, message: Message) throws -> Data {
        var data = try message.serializedData();

        return try data.withUnsafeMutableBytes { argsPointer in
            let ptr = argsPointer.baseAddress
            let ptr_len = argsPointer.count

            let rustResponse = flm_call_protobuf(self.flm_handle, method, ptr, ptr_len);

            guard let rustResponsePtr: UnsafeMutablePointer<RustResponse> = rustResponse else {
                throw FLMFacadeError.rustResponseAsNullptr
            }

            defer {
                flm_free_response(rustResponsePtr);
            }

            guard rustResponsePtr.pointee.ffi_error == false else {
                let data = Data(bytes: rustResponsePtr.pointee.result_data, count: rustResponsePtr.pointee.result_data_len);
                let error = try FilterListManager_AGOuterError(serializedBytes: data);

                throw AGOuterError(from: error)
            }

            return Data(bytes: rustResponsePtr.pointee.result_data, count: rustResponsePtr.pointee.result_data_len);
        }
    }

    func installCustomFilterList(downloadUrl: String, isTrusted: Bool, title: String?, description: String?) throws -> FilterListManager_FullFilterList {
        var message = FilterListManager_InstallCustomFilterListRequest()
        message.downloadURL = downloadUrl;
        message.isTrusted = isTrusted;

        if let name = title {
            message.title = name;
        }
        if let desc = description {
            message.description_p = desc;
        }

        let bytes = try callRust(method: InstallCustomFilterList, message: message);
        let response = try FilterListManager_InstallCustomFilterListResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.filterList
    }

    func enableFilterLists(ids: [Int64], isEnabled: Bool) throws -> Int64 {
        var message = FilterListManager_EnableFilterListsRequest()
        message.ids = ids;
        message.isEnabled = isEnabled;

        let bytes = try callRust(method: EnableFilterLists, message: message);
        let response = try FilterListManager_EnableFilterListsResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.count
    }

    func installFilterLists(ids: [Int64], isInstalled: Bool) throws -> Int64 {
        var message = FilterListManager_InstallFilterListsRequest()
        message.ids = ids;
        message.isInstalled = isInstalled;

        let bytes = try callRust(method: InstallFilterLists, message: message);
        let response = try FilterListManager_InstallFilterListsResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.count
    }

    func deleteCustomFilterLists(ids: [Int64]) throws -> Int64 {
        var message = FilterListManager_DeleteCustomFilterListsRequest()
        message.ids = ids;

        let bytes = try callRust(method: DeleteCustomFilterLists, message: message);
        let response = try FilterListManager_DeleteCustomFilterListsResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.count
    }

    func getFullFilterListsById(id: Int64) throws -> FilterListManager_FullFilterList {
        var message = FilterListManager_GetFullFilterListByIdRequest();
        message.id = id;

        let bytes = try callRust(method: GetFullFilterListById, message: message);
        let response = try FilterListManager_GetFullFilterListByIdResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.filterList
    }

    func getStoredFiltersMetadata() throws -> [FilterListManager_StoredFilterMetadata] {
        let message = FilterListManager_EmptyRequest();

        let bytes = try callRust(method: GetStoredFiltersMetadata, message: message);
        let response = try FilterListManager_GetStoredFiltersMetadataResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.filterLists
    }

    func getStoredFiltersMetadataById(id: Int64) throws -> FilterListManager_StoredFilterMetadata {
        var message = FilterListManager_GetStoredFiltersMetadataByIdRequest();
        message.id = id;

        let bytes = try callRust(method: GetStoredFilterMetadataById, message: message);
        let response = try FilterListManager_GetStoredFilterMetadataByIdResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.filterList
    }

    func saveCustomFilterRules(rules: FilterListManager_FilterListRules) throws {
        var message = FilterListManager_SaveCustomFilterRulesRequest();
        message.rules = rules;

        let bytes = try callRust(method: SaveCustomFilterRules, message: message);
        let response = try FilterListManager_EmptyResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }
    }

    func saveDisabledRules(id: Int64, disabledRules: [String]) throws {
        var message = FilterListManager_SaveDisabledRulesRequest();
        message.disabledRules = disabledRules;
        message.filterID = id;

        let bytes = try callRust(method: SaveDisabledRules, message: message);
        let response = try FilterListManager_EmptyResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }
    }

    func updateFilters(ignoreFiltersExpiration: Bool, looseTimeout: Int32, ignoreFiltersStatus: Bool) throws -> FilterListManager_UpdateResult {
        var message = FilterListManager_UpdateFiltersRequest()
        message.ignoreFiltersExpiration = ignoreFiltersExpiration;
        message.ignoreFiltersStatus = ignoreFiltersStatus;
        message.looseTimeout = looseTimeout;

        let bytes = try callRust(method: UpdateFilters, message: message);
        let response = try FilterListManager_UpdateFiltersResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.result
    }

    func forceUpdateFiltersByIds(ids: [Int64], looseTimeout: Int32) throws -> FilterListManager_UpdateResult {
        var message = FilterListManager_ForceUpdateFiltersByIdsRequest();
        message.ids = ids;
        message.looseTimeout = looseTimeout;

        let bytes = try callRust(method: ForceUpdateFiltersByIds, message: message);
        let response = try FilterListManager_ForceUpdateFiltersByIdsResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.result
    }

    func fetchFiltersListMetadata(url: String) throws -> FilterListManager_FilterListMetadata {
        var message = FilterListManager_FetchFilterListMetadataRequest();
        message.url = url;

        let bytes = try callRust(method: FetchFilterListMetadata, message: message);
        let response = try FilterListManager_FetchFilterListMetadataResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.metadata
    }

    func liftUpDatabase() throws {
        let message = FilterListManager_EmptyRequest();

        let bytes = try callRust(method: LiftUpDatabase, message: message)
        let response = try FilterListManager_EmptyResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }
    }

    func getAllTags() throws -> [FilterListManager_FilterTag] {
        let message = FilterListManager_EmptyRequest();

        let bytes = try callRust(method: GetAllTags, message: message);
        let response = try FilterListManager_GetAllTagsResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.tags
    }

    func getAllGroups() throws -> [FilterListManager_FilterGroup] {
        let message = FilterListManager_EmptyRequest();

        let bytes = try callRust(method: GetAllGroups, message: message);
        let response = try FilterListManager_GetAllGroupsResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.groups
    }

    func changeLocale(suggestedLocale: Locale) throws -> Bool {
        var message = FilterListManager_ChangeLocaleRequest();
        message.suggestedLocale = suggestedLocale.identifier;

        let bytes = try callRust(method: ChangeLocale, message: message);
        let response = try FilterListManager_ChangeLocaleResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.success
    }

    func pullMetadata() throws {
        let message = FilterListManager_EmptyRequest();

        let bytes = try callRust(method: PullMetadata, message: message);
        let response = try FilterListManager_EmptyResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }
    }

    func updateCustomFilterMetadata(id: Int64, title: String, isTrusted: Bool) throws -> Bool {
        var message = FilterListManager_UpdateCustomFilterMetadataRequest();
        message.filterID = id;
        message.title = title;
        message.isTrusted = isTrusted;

        let bytes = try callRust(method: UpdateCustomFilterMetadata, message: message);
        let response = try FilterListManager_UpdateCustomFilterMetadataResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.success
    }

    func getDatabasePath() throws -> String {
        let message = FilterListManager_EmptyRequest();

        let bytes = try callRust(method: GetDatabasePath, message: message);

        let response = try FilterListManager_GetDatabasePathResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.path;
    }

    func getDatabaseVersion() throws -> Int32 {
        let message = FilterListManager_EmptyRequest();

        let bytes = try callRust(method: GetDatabaseVersion, message: message);
        let response = try FilterListManager_GetDatabaseVersionResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.version;
    }

    func installCustomFilterFromString(
        downloadUrl: String,
        lastDownloadTime: Int64, 
        isEnabled: Bool,
        isTrusted: Bool,
        filterBody: String,
        customTitle: String?,
        customDescription: String?
    ) throws -> FilterListManager_FullFilterList {
        var message = FilterListManager_InstallCustomFilterFromStringRequest();
        message.downloadURL = downloadUrl;
        message.lastDownloadTime = lastDownloadTime;
        message.isEnabled = isEnabled;
        message.isTrusted = isTrusted;
        message.filterBody = filterBody;
        if let title = customTitle {
            message.customTitle = title;
        }
        if let description = customDescription {
            message.customDescription = description;
        }

        let bytes = try callRust(method: InstallCustomFilterFromString, message: message);
        let response = try FilterListManager_InstallCustomFilterFromStringResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.filterList;
    }

    func getFilterRulesAsStrings(ids: [Int64]) throws -> [FilterListManager_FilterListRulesRaw] {
        var message = FilterListManager_GetFilterRulesAsStringsRequest();
        message.ids = ids;

        let bytes = try callRust(method: GetFilterRulesAsStrings, message: message);
        let response = try FilterListManager_GetFilterRulesAsStringsResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.rulesList;
    }

    func saveRulesToFileBlob(id: Int64, filePath: String) throws {
        var message = FilterListManager_SaveRulesToFileBlobRequest();
        message.filterID = id;
        message.filePath = filePath;

        let bytes = try callRust(method: SaveRulesToFileBlob, message: message);
        let response = try FilterListManager_EmptyResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }
    }

    func getDisabledRules(ids: [Int64]) throws -> [FilterListManager_DisabledRulesRaw] {
        var message = FilterListManager_GetDisabledRulesRequest();
        message.ids = ids;

        let bytes = try callRust(method: GetDisabledRules, message: message);
        let response = try FilterListManager_GetDisabledRulesResponse(serializedBytes: bytes);

        guard response.hasError == false else {
            throw AGOuterError(from: response.error)
        }

        return response.rulesRaw;
    }

    deinit {
        flm_free_handle(self.flm_handle)
    }
}

func testExceptions() throws {
    let conf = try spawnConf();

    let flm = try FLMFacade(configuration: conf);

    let path = try flm.getDatabasePath();

    if !FileManager.default.fileExists(atPath: path) {
        throw FLMFacadeError.testWillFail
    }

    try FileManager.default.removeItem(atPath: path);

    let _ = try flm.getAllTags();
}

func testAllMethods() throws {
    let conf = try spawnConf();

    let flm = try FLMFacade(configuration: conf);
    try flm.pullMetadata();
    let _ = try flm.updateFilters(ignoreFiltersExpiration: false, looseTimeout: 0, ignoreFiltersStatus: false);

    let filter = try flm.getFullFilterListsById(id: 1);
    print("[Default locale] Filter with id 1 has title \(filter.title)");

    let locale_result = try flm.changeLocale(suggestedLocale: Locale(identifier: "ru_RU"));
    print("Changing locale result: \(locale_result)");

    try flm.liftUpDatabase();

    print("Database successfully lifted");

    let _ = try flm.enableFilterLists(ids: [1, 2, 255], isEnabled: true);
    let _ = try flm.installFilterLists(ids: [1, 2, 255], isInstalled: true);

    print("Lists 1,2,255 successfully installed and enabled");

    let customFilterFromString = try flm.installCustomFilterFromString(
        downloadUrl: "",
        lastDownloadTime: 1000000000,
        isEnabled: true,
        isTrusted: true,
        filterBody: "custom filter string body",
        customTitle: nil,
        customDescription: "Desc"
    );

    print("Custom filter from string body: \(customFilterFromString.rules.rules.joined())");

    var rules1 = FilterListManager_FilterListRules();
    rules1.filterID = customFilterFromString.id;
    rules1.rules = ["hello", "world"];

    try flm.saveCustomFilterRules(rules: rules1);

    print("Custom filter rules were saved");

    try flm.saveDisabledRules(id: customFilterFromString.id, disabledRules: ["world"]);

    print("Disabled rules were saved");

    let FRAS = try flm.getFilterRulesAsStrings(ids: [customFilterFromString.id]);
    print("Rules from getFilterRulesAsStrings for customId: \(FRAS)");

    // TODO: This test always failed with no file or directory
    //let testFile = FileManager.default.temporaryDirectory.appending(path: "flmtest_2.txt");
    //let _ = try flm.saveRulesToFileBlob(id: customFilterFromString.id, filePath: testFile.absoluteString);
    //print("Rules were written into \(testFile)");

    let disabledRules = try flm.getDisabledRules(ids: [customFilterFromString.id]);
    print("GetDisabledRules returns \(disabledRules)");

    let _ = try flm.deleteCustomFilterLists(ids: [customFilterFromString.id]);

    print("Filter \(customFilterFromString.id) was deleted");

    print("Flm get metadata. Groups count: \(try flm.getAllGroups().count); Tags count: \(try flm.getAllTags().count);");

    let _ = try flm.forceUpdateFiltersByIds(ids: [1,2], looseTimeout: 0);

    print("Filters 1,2 were force updated");

    let anotherCustomFilter = try flm.installCustomFilterList(
        downloadUrl: "https://filters.adtidy.org/extension/safari/filters/101_optimized.txt",
        isTrusted: true,
        title: "Some filter",
        description: "Some desc"
    );

    print("Installed another custom filter with new id: \(anotherCustomFilter.id)");

    let _ = try flm.updateCustomFilterMetadata(id: anotherCustomFilter.id, title: "new title", isTrusted: true);

    print("Custom filter metadata was updated");

    let filterMetadata = try flm.fetchFiltersListMetadata(url: "https://filters.adtidy.org/extension/safari/filters/101.txt");

    print("Got remote metadata. Homepage: \(filterMetadata.homepage)");

    let _ = try flm.deleteCustomFilterLists(ids: [anotherCustomFilter.id]);

    print("Custom filter successfully removed");

    print("Current database path: '\(try flm.getDatabasePath())' and version: '\(try flm.getDatabaseVersion())'");
}

do {
    try testAllMethods();
} catch {
    print("Type: \(type(of: error))");
    print("Error description iz: \(error.localizedDescription)");
}

