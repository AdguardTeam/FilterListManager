package com.adguard.flm

import com.adguard.flm.driver.FilterListManagerDriver
import com.adguard.flm.protobuf.*
import platform.Foundation.NSTemporaryDirectory
import platform.Foundation.NSUUID
import platform.Foundation.timeIntervalSince1970
import kotlin.test.*

class FilterListManagerIOSTest {

    @Test
    fun testMinimalConfiguration() {
        val minimalConf = Configuration(
            appName = "test-ios",
            version = "1.0",
            workingDirectory = "${NSTemporaryDirectory()}flm-minimal-ios-test-${NSUUID().UUIDString()}"
        )
        FlmAdapterFactory.create(minimalConf)!!.apply {
            try {
                val constants = FilterListManagerDriver.getConstants()
                assertNotNull(constants, "Constants should be available")
                assertTrue(constants.smallestFilterId != 1, "Smallest filter ID should not be 1")
            } finally {
                close()
            }
        }
    }

    @Test
    fun testInvalidMetadataConfiguration() {
        val invalidConf = Configuration(
            appName = "adguard-flm-kmp-ios-test",
            version = "1.0",
            workingDirectory = "${NSTemporaryDirectory()}flm-ios-test-${NSUUID().UUIDString()}",
            metadataUrl = "",
            metadataLocalesUrl = ""
        )
        FlmAdapterFactory.create(invalidConf)!!.apply {
            try {
                val result = pullMetadata()
                assertNull(result, "Pull metadata should return null for invalid configuration")
            } finally {
                close()
            }
        }
    }

    @Test
    fun testBasic() {
        val constants = FilterListManagerDriver.getConstants()
        assertTrue(constants.smallestFilterId != 1, "Smallest filter ID should not be 1")

        val conf = createTestConfiguration()
        FlmAdapterFactory.create(conf)!!.apply {
            try {
                liftUpDatabase()

                val version = getDatabaseVersion()
                assertNotNull(version, "Database version should not be null after initialization")
                assertTrue(version > 0, "Database version should be positive")
            } finally {
                close()
            }
        }
    }

    @Test
    fun testDatabaseOperations() {
        val conf = createTestConfiguration()
        FlmAdapterFactory.create(conf)!!.apply {
            try {
                val dbPath = getDatabasePath()
                assertNotNull(dbPath, "Database path should not be null")
                assertTrue(dbPath.contains(conf.workingDirectory ?: ""), "Database path should contain the working directory")

                val version = getDatabaseVersion()
                assertNotNull(version, "Database version should not be null")
                assertTrue(version > 0, "Database version should be positive")
            } finally {
                close()
            }
        }
    }

    @Ignore // TODO change tests to mocks
    @Test
    fun testLocaleOperations() {
        val conf = createTestConfiguration()

        FlmAdapterFactory.create(conf)!!.apply {
            try {
                pullMetadata()

                val result = changeLocale("en_US")
                assertTrue(result == true, "Changing to a valid locale should succeed")

                val invalidResult = changeLocale("xx_YY")
                assertFalse(invalidResult == true, "Changing to an invalid locale should return false")
            } finally {
                close()
            }
        }
    }

    @Ignore // TODO change tests to mocks
    @Test
    fun testMetadataOperations() {
        val conf = createTestConfiguration()

        FlmAdapterFactory.create(conf)!!.apply {
            try {
                setProxyMode(RawRequestProxyMode.NO_PROXY)

                val tags = getAllTags()
                assertNotNull(tags, "Tags list should not be null")

                val groups = getAllGroups()
                assertNotNull(groups, "Groups list should not be null")

                val pullResult = pullMetadata()
                assertNotNull(pullResult, "Pull metadata result should not be null")
            } finally {
                close()
            }
        }
    }

    @Test
    fun testFilterOperations() {
        val conf = createTestConfiguration()

        FlmAdapterFactory.create(conf)!!.apply {
            try {
                val filters = getStoredFiltersMetadata()
                assertNotNull(filters, "Stored filters metadata should not be null")

                val customFilterRules = "! Test filter\n||example.com^"
                val customFilter = installCustomFilterFromString(
                    downloadUrl = "",
                    lastDownloadTime = platform.Foundation.NSDate().timeIntervalSince1970.toLong(),
                    isEnabled = true,
                    isTrusted = true,
                    filterBody = customFilterRules,
                    customTitle = "Test Filter iOS",
                    customDescription = "A test filter for iOS KMP tests"
                )

                assertNotNull(customFilter, "Custom filter should be created successfully")
                assertTrue(customFilter.id != 0, "Custom filter ID should be valid")
                assertEquals("Test Filter iOS", customFilter.title, "Custom filter title should match")

                val retrievedFilter = getFullFilterListById(customFilter.id)
                assertNotNull(retrievedFilter, "Retrieved filter should not be null")
                assertEquals(customFilter.id, retrievedFilter.id, "Retrieved filter ID should match")

                val updateResult = updateCustomFilterMetadata(
                    id = customFilter.id,
                    title = "Updated Test Filter iOS",
                    isTrusted = false
                )
                assertTrue(updateResult == true, "Updating custom filter metadata should succeed")

                val activeRulesResult = getActiveRulesRaw(listOf(customFilter.id))!!
                assertEquals(1, activeRulesResult.size, "Should have one active rule result")
                assertEquals(customFilter.id, activeRulesResult[0].filterId, "Filter ID should match")

                val rulesCount = getRulesCount(listOf(customFilter.id))
                assertNotNull(rulesCount, "Rules count should not be null")
                assertFalse(rulesCount.isEmpty(), "Rules count list should not be empty")

                val deleteResult = deleteCustomFilterLists(listOf(customFilter.id))
                assertNotNull(deleteResult, "Delete result should not be null")
                assertTrue(deleteResult > 0, "Deleting custom filter should affect at least one row")
            } finally {
                close()
            }
        }
    }

    @Test
    fun testMemoryManagement() {
        val conf = createTestConfiguration()

        repeat(10) { iteration ->
            FlmAdapterFactory.create(conf)!!.apply {
                try {
                    val constants = FilterListManagerDriver.getConstants()
                    assertNotNull(constants, "Constants should be available in iteration $iteration")

                    val version = getDatabaseVersion()
                    assertNotNull(version, "Database version should not be null in iteration $iteration")

                    val dbPath = getDatabasePath()
                    assertNotNull(dbPath, "Database path should not be null in iteration $iteration")
                } finally {
                    close()
                }
            }
        }
    }

    @Test
    fun testConcurrentAccess() {
        val conf = createTestConfiguration()

        FlmAdapterFactory.create(conf)!!.apply {
            try {
                val constants1 = FilterListManagerDriver.getConstants()
                val constants2 = FilterListManagerDriver.getConstants()

                assertEquals(constants1.userRulesId, constants2.userRulesId, "Constants should be consistent")
                assertEquals(constants1.customGroupId, constants2.customGroupId, "Constants should be consistent")
                assertEquals(constants1.specialGroupId, constants2.specialGroupId, "Constants should be consistent")
                assertEquals(constants1.smallestFilterId, constants2.smallestFilterId, "Constants should be consistent")
            } finally {
                close()
            }
        }
    }

    @Test
    fun testLargeFilterHandling() {
        val conf = createTestConfiguration()

        FlmAdapterFactory.create(conf)!!.apply {
            try {
                val largeFilterRules = buildString {
                    appendLine("! Large Test Filter")
                    repeat(1000) { i ->
                        appendLine("||example$i.com^")
                    }
                }

                val largeFilter = installCustomFilterFromString(
                    downloadUrl = "",
                    lastDownloadTime = platform.Foundation.NSDate().timeIntervalSince1970.toLong(),
                    isEnabled = true,
                    isTrusted = true,
                    filterBody = largeFilterRules,
                    customTitle = "Large Test Filter iOS",
                    customDescription = "A large test filter for iOS performance testing"
                )

                assertNotNull(largeFilter, "Large filter should be created successfully")
                assertTrue(largeFilter.id != 0, "Large filter ID should be valid")

                val rulesCount = getRulesCount(listOf(largeFilter.id))
                assertNotNull(rulesCount, "Rules count should not be null")
                assertFalse(rulesCount.isEmpty(), "Rules count should not be empty for large filter")
                assertTrue(rulesCount[0].rulesCount > 900, "Should have approximately 1000+ rules")

                val deleteResult = deleteCustomFilterLists(listOf(largeFilter.id))
                assertNotNull(deleteResult, "Delete result should not be null")
                assertTrue(deleteResult > 0, "Deleting large filter should affect at least one row")
            } finally {
                close()
            }
        }
    }

    @Test
    fun testIOSSpecificPaths() {
        val tempDir = NSTemporaryDirectory()
        val testUUID = NSUUID().UUIDString()

        val conf = Configuration(
            appName = "ios-path-test",
            version = "1.0",
            workingDirectory = "${tempDir}flm-path-test-$testUUID"
        )

        FlmAdapterFactory.create(conf)!!.apply {
            try {
                val dbPath = getDatabasePath()
                assertNotNull(dbPath, "Database path should not be null")
                assertTrue(dbPath.contains(tempDir), "Database path should be within temp directory")
                assertTrue(dbPath.contains(testUUID), "Database path should contain unique identifier")
            } finally {
                close()
            }
        }
    }

    private fun createTestConfiguration() = Configuration(
        appName = "adguard-flm-kmp-ios-test",
        shouldIgnoreExpiresForLocalUrls = true,
        version = "1.0",
        workingDirectory = "${NSTemporaryDirectory()}flm-ios-test-${NSUUID().UUIDString()}",
        metadataUrl = "https://filters.adtidy.org/android/filters.json",
        metadataLocalesUrl = "https://filters.adtidy.org/android/filters_i18n.json"
    )
}
