package com.adguard.flm

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.adguard.flm.protobuf.*
import org.junit.Assert.*
import org.junit.Test
import org.junit.runner.RunWith
import java.io.File

/**
 * Instrumented test for KMP FilterListManager, which will execute on an Android device.
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 */
@RunWith(AndroidJUnit4::class)
class FilterListManagerKMPTest {
    private val context = ApplicationProvider.getApplicationContext<Context>()

    private fun createTestConfiguration() = FilterListManager.defaultConfiguration!!.copy(
        appName = "adguard-flm-kmp-instrumented-test",
        shouldIgnoreExpiresForLocalUrls = true,
        version = "1.0",
        workingDirectory = File(context.cacheDir, "flm-kmp-test-${System.currentTimeMillis()}").apply {
            mkdirs()
        }.absolutePath,
        metadataUrl = "https://filters.adtidy.org/android/filters.json",
        metadataLocalesUrl = "https://filters.adtidy.org/android/filters_i18n.json"
    )


    @Test
    fun testMinimalConfiguration() {
        // Test that minimal configuration works
        val minimalConf = Configuration(
            appName = "test",
            version = "1.0",
            workingDirectory = File(context.cacheDir, "flm-minimal-test-${System.currentTimeMillis()}").apply {
                mkdirs()
            }.absolutePath
        )
        FilterListManager.create(minimalConf)!!.apply {
            try {
                // Should not throw exception
                val constants = FilterListManager.constants
                assertNotNull("Constants should be available", constants)
            } finally {
                close()
            }
        }
    }

    @Test
    fun testInvalidMetadataConfiguration() {
        val invalidConf = FilterListManager.defaultConfiguration!!.copy(
            appName = "adguard-flm-kmp-instrumented-test",
            version = "1.0",
            workingDirectory = File(context.cacheDir, "flm-kmp-test-${System.currentTimeMillis()}").apply {
                mkdirs()
            }.absolutePath,
            metadataUrl = "",
            metadataLocalesUrl = ""
        )
        FilterListManager.create(invalidConf)!!.apply {
            try {
                // With invalid configuration, pullMetadata should return null instead of throwing exception
                val result = pullMetadata()
                assertNull("Pull metadata should return null for invalid configuration", result)
            } finally {
                close()
            }
        }
    }

    @Test
    fun testBasic() {
        val constants = FilterListManager.constants
        assertTrue("Smallest filter ID should not be 1", constants.smallestFilterId != 1)

        val conf = createTestConfiguration()
        FilterListManager.create(conf)!!.apply {
            try {
                liftUpDatabase()

                // Verify database is automatically initialized by checking version
                val version = getDatabaseVersion()
                assertNotNull("Database version should not be null after initialization", version)
                assertTrue("Database version should be positive", version!! > 0)
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
                val dbPath = getDatabasePath()!!
                assertNotNull("Database path should not be null", dbPath)
                assertTrue("Database path should contain the working directory", dbPath.contains(conf.workingDirectory ?: ""))

                // Test getting database version
                val version = getDatabaseVersion()
                assertNotNull("Database version should not be null", version)
                assertTrue("Database version should be positive", version!! > 0)
            } finally {
                close()
            }
        }
    }

    @Test
    fun testLocaleOperations() {
        val conf = createTestConfiguration()

        FilterListManager.create(conf)!!.apply {
            try {
                pullMetadata()

                // Test changing locale
                val result = changeLocale("en_US")
                assertTrue("Changing to a valid locale should succeed", result == true)

                // Test changing to an invalid locale (should return false but not throw)
                val invalidResult = changeLocale("xx_YY")
                assertFalse("Changing to an invalid locale should return false", invalidResult == true)
            } finally {
                close()
            }
        }
    }

    @Test
    fun testMetadataOperations() {
        val conf = createTestConfiguration()

        FilterListManager.create(conf)!!.apply {
            try {
                // Test getting all tags
                val tags = getAllTags()
                assertNotNull("Tags list should not be null", tags)

                // Test getting all groups
                val groups = getAllGroups()
                assertNotNull("Groups list should not be null", groups)

                // Test pulling metadata - only if needed for specific tests
                // Commented out to avoid network calls in every test run
                val pullResult = pullMetadata()
                assertNotNull("Pull metadata result should not be null", pullResult)
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
                assertNotNull("Stored filters metadata should not be null", filters)

                // Create a custom filter
                val customFilterRules = "! Test filter\n||example.com^"
                val customFilter = installCustomFilterFromString(
                    downloadUrl = "",
                    lastDownloadTime = System.currentTimeMillis() / 1000,
                    isEnabled = true,
                    isTrusted = true,
                    filterBody = customFilterRules,
                    customTitle = "Test Filter",
                    customDescription = "A test filter for KMP instrumented tests"
                )

                assertNotNull("Custom filter should be created successfully", customFilter)
                assertTrue("Custom filter ID should be valid", customFilter?.id != 0)
                assertEquals("Custom filter title should match", "Test Filter", customFilter?.title)

                // Test getting filter by ID
                val retrievedFilter = getFullFilterListById(customFilter!!.id)
                assertNotNull("Retrieved filter should not be null", retrievedFilter)
                assertEquals("Retrieved filter ID should match", customFilter.id, retrievedFilter?.id)

                // Test updating custom filter metadata
                val updateResult = updateCustomFilterMetadata(
                    id = customFilter.id,
                    title = "Updated Test Filter",
                    isTrusted = false
                )
                assertTrue("Updating custom filter metadata should succeed", updateResult == true)

                val activeRulesResult = getActiveRulesRaw(listOf(customFilter.id))!!
                assertEquals(activeRulesResult.size, 1)
                assertEquals(activeRulesResult[0].filterId, customFilter.id)

                // Test getting rules count
                val rulesCount = getRulesCount(listOf(customFilter.id))!!
                assertNotNull("Rules count should not be null", rulesCount)
                assertFalse("Rules count list should not be empty", rulesCount.isEmpty())

                // Test deleting the custom filter
                val deleteResult = deleteCustomFilterLists(listOf(customFilter.id))
                assertNotNull("Delete result should not be null", deleteResult)
                assertTrue("Deleting custom filter should affect at least one row", deleteResult!! > 0)
            } finally {
                close()
            }
        }
    }
}
