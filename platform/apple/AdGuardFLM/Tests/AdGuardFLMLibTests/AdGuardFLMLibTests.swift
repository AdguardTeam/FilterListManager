import XCTest
import SwiftProtobuf
import AdGuardFLM

@testable import AdGuardFLMLib

final class AdGuardFLMLibTests: XCTestCase {
    private func spawnConf() throws -> FilterListManager_Configuration {
        var configuration = try makeDefaultConfiguration()

        configuration.locale = "en"
        configuration.metadataURL = "https://filters.adtidy.org/extension/safari/filters.json"
        configuration.metadataLocalesURL = "https://filters.adtidy.org/extension/safari/filters_i18n.json"
        configuration.workingDirectory = "."

        return configuration
    }

    func testAllMethods() throws {
        let conf = try spawnConf()

        let flm = try FLMFacade(configuration: conf)
        try flm.pullMetadata()
        XCTAssertNoThrow(
            try flm.updateFilters(
                ignoreFiltersExpiration: false,
                looseTimeout: 0,
                ignoreFiltersStatus: false
            )
        )

        let filter = try flm.getFullFilterListsById(id: 1)
        NSLog("[Default locale] Filter with id 1 has title \(filter.title)")

        let locale_result = try flm.changeLocale(suggestedLocale: Locale(identifier: "ru_RU"))
        NSLog("Changing locale result: \(locale_result)")

        try flm.liftUpDatabase()

        NSLog("Database successfully lifted")

        let _ = try flm.enableFilterLists(ids: [1, 2, 255], isEnabled: true)
        let _ = try flm.installFilterLists(ids: [1, 2, 255], isInstalled: true)

        NSLog("Lists 1,2,255 successfully installed and enabled")

        let customFilterFromString = try flm.installCustomFilterFromString(
            downloadUrl: "",
            lastDownloadTime: 1000000000,
            isEnabled: true,
            isTrusted: true,
            filterBody: "custom filter string body",
            customTitle: nil,
            customDescription: "Desc"
        )

        NSLog("Custom filter from string body: \(customFilterFromString.rules.rules.joined())")

        var rules1 = FilterListManager_FilterListRules()
        rules1.filterID = customFilterFromString.id
        rules1.rules = ["hello", "world"]

        try flm.saveCustomFilterRules(rules: rules1)

        NSLog("Custom filter rules were saved")

        try flm.saveDisabledRules(id: customFilterFromString.id, disabledRules: ["world"])

        NSLog("Disabled rules were saved")

        let FRAS = try flm.getFilterRulesAsStrings(ids: [customFilterFromString.id])
        NSLog("Rules from getFilterRulesAsStrings for customId: \(FRAS)")

        let testFile = FileManager.default.temporaryDirectory.appending(path: "flmtest_2.txt")
        XCTAssertNoThrow(try flm.saveRulesToFileBlob(id: customFilterFromString.id, filePath: testFile.path()))
        NSLog("Rules were written into \(testFile)")

        let disabledRules = try flm.getDisabledRules(ids: [customFilterFromString.id])
        NSLog("GetDisabledRules returns \(disabledRules)")

        XCTAssertNoThrow(try flm.deleteCustomFilterLists(ids: [customFilterFromString.id]))

        NSLog("Filter \(customFilterFromString.id) was deleted")

        NSLog("Flm get metadata. Groups count: \(try flm.getAllGroups().count) Tags count: \(try flm.getAllTags().count)")

        XCTAssertNoThrow(try flm.forceUpdateFiltersByIds(ids: [1,2], looseTimeout: 0))

        NSLog("Filters 1,2 were force updated")

        let anotherCustomFilter = try flm.installCustomFilterList(
            downloadUrl: "https://filters.adtidy.org/extension/safari/filters/101_optimized.txt",
            isTrusted: true,
            title: "Some filter",
            description: "Some desc"
        )

        NSLog("Installed another custom filter with new id: \(anotherCustomFilter.id)")

        XCTAssertNoThrow(
            try flm.updateCustomFilterMetadata(
                id: anotherCustomFilter.id,
                title: "new title",
                isTrusted: true
            )
        )

        NSLog("Custom filter metadata was updated")

        let filterMetadata = try flm.fetchFiltersListMetadata(url: "https://filters.adtidy.org/extension/safari/filters/101.txt")

        NSLog("Got remote metadata. Homepage: \(filterMetadata.homepage)")

        let filterMetadataWithBody = try flm.fetchFiltersListMetadataWithBody(url: "https://filters.adtidy.org/extension/safari/filters/101.txt")

        NSLog("Got remote metadata with body. Homepage: \(filterMetadataWithBody.metadata.homepage)")

        XCTAssertNoThrow(try flm.deleteCustomFilterLists(ids: [anotherCustomFilter.id]))

        NSLog("Custom filter successfully removed")

        NSLog("Current database path: '\(try flm.getDatabasePath())' and version: '\(try flm.getDatabaseVersion())'")

        XCTAssertNoThrow(try flm.setProxyMode(mode: FilterListManager_RawRequestProxyMode.useCustomProxy, custom_addr: "http://localhost:8080"))

        XCTAssertNoThrow(try flm.setProxyMode(mode: FilterListManager_RawRequestProxyMode.noProxy, custom_addr: nil))

        let constants = flm_get_constants()

        XCTAssert(constants.user_rules_id == Int32.min, "FLM user rules id must be equal to int::min")
        XCTAssert(constants.custom_group_id == Int32.min, "FLM Custom group id must be equal to int::min")
        XCTAssert(constants.special_group_id == 0, "FLM Special group id must be zero")
        XCTAssert(constants.smallest_filter_id == -2_000_000_000, "FLM Custom group id must be two billions")
        XCTAssert(filterMetadataWithBody.metadata.homepage.count > 0, "FLM Metadata homepage must be non-empty")
    }
}

/*
 import Foundation
 import SwiftProtobuf

 func testExceptions() throws {
     let conf = try spawnConf()

     let flm = try FLMFacade(configuration: conf)

     let path = try flm.getDatabasePath()

     if !FileManager.default.fileExists(atPath: path) {
         throw FLMFacadeError.testWillFail
     }

     try FileManager.default.removeItem(atPath: path)

     let _ = try flm.getAllTags()
 }

 */
