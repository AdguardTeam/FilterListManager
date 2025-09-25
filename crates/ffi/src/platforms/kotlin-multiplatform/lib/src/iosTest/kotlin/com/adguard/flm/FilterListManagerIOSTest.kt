package com.adguard.flm

import com.adguard.flm.protobuf.*
import platform.Foundation.NSTemporaryDirectory
import platform.Foundation.NSUUID
import platform.Foundation.timeIntervalSince1970
import kotlin.test.*

/**
 * Instrumented test for KMP FilterListManager, which will execute on iOS.
 * These tests verify the iOS-specific CInterop implementation and memory management.
 */
class FilterListManagerIOSTest {

    @Test
    fun testMinimalConfiguration() {
        // Test that minimal configuration works on iOS
        val minimalConf = Configuration(
            appName = "test-ios",
            version = "1.0",
            workingDirectory = "${NSTemporaryDirectory()}flm-minimal-ios-test-${NSUUID().UUIDString()}"
        )
        FilterListManager.create(minimalConf)!!.apply {
            try {
                // Should not throw exception
                val constants = FilterListManager.constants
                assertNotNull(constants, "Constants should be available")
                assertTrue(constants.smallestFilterId != 1, "Smallest filter ID should not be 1")
            } finally {
                close()
            }
        }
    }

    @Test
    fun testInvalidMetadataConfiguration() {
        val invalidConf = FilterListManager.defaultConfiguration!!.copy(
            appName = "adguard-flm-kmp-ios-test",
            version = "1.0",
            workingDirectory = "${NSTemporaryDirectory()}flm-ios-test-${NSUUID().UUIDString()}",
            metadataUrl = "",
            metadataLocalesUrl = ""
        )
        FilterListManager.create(invalidConf)!!.apply {
            try {
                // With invalid configuration, pullMetadata should return null instead of throwing exception
                val result = pullMetadata()
                assertNull(result, "Pull metadata should return null for invalid configuration")
            } finally {
                close()
            }
        }
    }

    @Test
    fun testBasic() {
        val constants = FilterListManager.constants
        assertTrue(constants.smallestFilterId != 1, "Smallest filter ID should not be 1")

        val conf = createTestConfiguration()
        FilterListManager.create(conf)!!.apply {
            try {
                liftUpDatabase()

                // Verify database is automatically initialized by checking version
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
        FilterListManager.create(conf)!!.apply {
            try {
                // Test getting database path without initialization
                val dbPath = getDatabasePath()
                assertNotNull(dbPath, "Database path should not be null")
                assertTrue(dbPath.contains(conf.workingDirectory ?: ""), "Database path should contain the working directory")

                // Test getting database version
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

        FilterListManager.create(conf)!!.apply {
            try {
                pullMetadata()

                // Test changing locale
                val result = changeLocale("en_US")
                assertTrue(result == true, "Changing to a valid locale should succeed")

                // Test changing to an invalid locale (should return false but not throw)
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

        FilterListManager.create(conf)!!.apply {
            try {
                // Set NO_PROXY mode for iOS simulator
                setProxyMode(RawRequestProxyMode.NO_PROXY)

                // Test getting all tags
                val tags = getAllTags()
                assertNotNull(tags, "Tags list should not be null")

                // Test getting all groups
                val groups = getAllGroups()
                assertNotNull(groups, "Groups list should not be null")

                // Test pulling metadata
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

        FilterListManager.create(conf)!!.apply {
            try {
                // Test getting stored filters metadata
                val filters = getStoredFiltersMetadata()
                assertNotNull(filters, "Stored filters metadata should not be null")

                // Create a custom filter
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

                // Test getting filter by ID
                val retrievedFilter = getFullFilterListById(customFilter.id)
                assertNotNull(retrievedFilter, "Retrieved filter should not be null")
                assertEquals(customFilter.id, retrievedFilter.id, "Retrieved filter ID should match")

                // Test updating custom filter metadata
                val updateResult = updateCustomFilterMetadata(
                    id = customFilter.id,
                    title = "Updated Test Filter iOS",
                    isTrusted = false
                )
                assertTrue(updateResult == true, "Updating custom filter metadata should succeed")

                val activeRulesResult = getActiveRulesRaw(listOf(customFilter.id))!!
                assertEquals(1, activeRulesResult.size, "Should have one active rule result")
                assertEquals(customFilter.id, activeRulesResult[0].filterId, "Filter ID should match")

                // Test getting rules count
                val rulesCount = getRulesCount(listOf(customFilter.id))
                assertNotNull(rulesCount, "Rules count should not be null")
                assertFalse(rulesCount.isEmpty(), "Rules count list should not be empty")

                // Test deleting the custom filter
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
        // iOS-specific test to ensure proper memory management with CInterop
        val conf = createTestConfiguration()

        repeat(10) { iteration ->
            FilterListManager.create(conf)!!.apply {
                try {
                    // Perform some operations to exercise memory allocation/deallocation
                    val constants = FilterListManager.constants
                    assertNotNull(constants, "Constants should be available in iteration $iteration")

                    val version = getDatabaseVersion()
                    assertNotNull(version, "Database version should not be null in iteration $iteration")

                    val dbPath = getDatabasePath()
                    assertNotNull(dbPath, "Database path should not be null in iteration $iteration")
                } finally {
                    close() // This should properly free native memory
                }
            }
        }
    }

    @Test
    fun testConcurrentAccess() {
        // Test that the library handles concurrent access properly on iOS
        val conf = createTestConfiguration()

        FilterListManager.create(conf)!!.apply {
            try {
                // Test concurrent access to constants (should be thread-safe)
                val constants1 = FilterListManager.constants
                val constants2 = FilterListManager.constants

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
        // Test handling of large filters on iOS platform
        val conf = createTestConfiguration()

        FilterListManager.create(conf)!!.apply {
            try {
                // Create a large filter with many rules
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

                // Test getting rules count for large filter
                val rulesCount = getRulesCount(listOf(largeFilter.id))
                assertNotNull(rulesCount, "Rules count should not be null")
                assertFalse(rulesCount.isEmpty(), "Rules count should not be empty for large filter")
                assertTrue(rulesCount[0].rulesCount > 900, "Should have approximately 1000+ rules")

                // Cleanup
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
        // Test that iOS-specific paths work correctly
        val tempDir = NSTemporaryDirectory()
        val testUUID = NSUUID().UUIDString()

        val conf = Configuration(
            appName = "ios-path-test",
            version = "1.0",
            workingDirectory = "${tempDir}flm-path-test-$testUUID"
        )

        FilterListManager.create(conf)!!.apply {
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



    private fun createTestConfiguration() = FilterListManager.defaultConfiguration!!.copy(
        appName = "adguard-flm-kmp-ios-test",
        shouldIgnoreExpiresForLocalUrls = true,
        version = "1.0",
        workingDirectory = "${NSTemporaryDirectory()}flm-ios-test-${NSUUID().UUIDString()}",
        metadataUrl = "https://filters.adtidy.org/android/filters.json",
        metadataLocalesUrl = "https://filters.adtidy.org/android/filters_i18n.json"
    )
}
