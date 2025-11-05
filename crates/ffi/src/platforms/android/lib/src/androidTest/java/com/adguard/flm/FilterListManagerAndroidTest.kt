package com.adguard.flm

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.adguard.flm.exceptions.FilterListManagerException
import com.adguard.flm.protobuf.*
import org.junit.Assert.*
import org.junit.Test
import org.junit.runner.RunWith
import java.io.File

/**
 * Instrumented test, which will execute on an Android device.
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 */
@RunWith(AndroidJUnit4::class)
class FilterListManagerAndroidTest {
    private val context = ApplicationProvider.getApplicationContext<Context>()

    private fun createTestConfiguration() = FilterListManager.defaultConfiguration.copy {
        appName = "adguard-flm-instrumented-test"
        shouldIgnoreExpiresForLocalUrls = true
        version = "1.0"
        workingDirectory = File(context.cacheDir, "flm-test-${System.currentTimeMillis()}").apply {
            mkdirs()
        }.absolutePath
        metadataUrl = "https://filters.adtidy.org/android/filters.json"
        metadataLocalesUrl = "https://filters.adtidy.org/android/filters_i18n.json"
    }

    private fun <T : Throwable> assertThrows(
        message: String? = null,
        expected: Class<T>,
        block: () -> Unit
    ): T {
        try {
            block()
        } catch (e: Throwable) {
            if (expected.isInstance(e)) {
                @Suppress("UNCHECKED_CAST")
                return e as T
            }
            throw AssertionError(message ?: "Expected ${expected.name}, but got ${e::class.java.name}", e)
        }
        throw AssertionError(message ?: "Expected ${expected.name}, but no exception was thrown")
    }

    @Test
    fun testInvalidConfiguration() {
        val invalidConf = configuration {
            // Empty configuration
        }
        assertThrows("Should throw exception for invalid configuration", FilterListManagerException::class.java) {
            FilterListManager(invalidConf).close()
        }
    }

    @Test
    fun testInvalidMetadataConfiguration() {
        val invalidConf = FilterListManager.defaultConfiguration.copy {
            appName = "adguard-flm-instrumented-test"
            version = "1.0"
            workingDirectory = File(context.cacheDir, "flm-test-${System.currentTimeMillis()}").apply {
                mkdirs()
            }.absolutePath
            metadataUrl = ""
            metadataLocalesUrl = ""
        }
        assertThrows("Should throw exception for invalid configuration", FilterListManagerException::class.java) {
            FilterListManager(invalidConf).use {
                it.pullMetadata()
            }
        }
    }

    @Test
    fun testBasic() {
        val constants = FilterListManager.constants
        assertTrue("Smallest filter ID should not be 1", constants.smallestFilterId != 1)

        val conf = createTestConfiguration()
        FilterListManager(conf).use { flm ->
            flm.liftUpDatabase()

            // Verify database is automatically initialized by checking version
            val version = flm.getDatabaseVersion()
            assertNotNull("Database version should not be null after initialization", version)
            assertTrue("Database version should be positive", version!! > 0)
        }
    }

    @Test
    fun testDatabaseOperations() {
        val conf = createTestConfiguration()
        FilterListManager(conf).use { flm ->
            // Test getting database path without initialization
            val dbPath = flm.getDatabasePath()
            assertNotNull("Database path should not be null", dbPath)
            assertTrue("Database path should contain the working directory", dbPath.contains(conf.workingDirectory))

            // Test getting database version
            val version = flm.getDatabaseVersion()
            assertNotNull("Database version should not be null", version)
            assertTrue("Database version should be positive", version!! > 0)
        }
    }

    @Test
    fun testLocaleOperations() {
        val conf = createTestConfiguration()

        FilterListManager(conf).use { flm ->
            flm.pullMetadata()

            // Test changing locale
            val result = flm.changeLocale("en_US")
            assertTrue("Changing to a valid locale should succeed", result)

            // Test changing to an invalid locale (should return false but not throw)
            val invalidResult = flm.changeLocale("xx_YY")
            assertFalse("Changing to an invalid locale should return false", invalidResult)
        }
    }

    @Test
    fun testMetadataOperations() {
        val conf = createTestConfiguration()

        FilterListManager(conf).use { flm ->

            // Test getting all tags
            val tags = flm.getAllTags()
            assertNotNull("Tags list should not be null", tags)

            // Test getting all groups
            val groups = flm.getAllGroups()
            assertNotNull("Groups list should not be null", groups)

            // Test pulling metadata - only if needed for specific tests
            // Commented out to avoid network calls in every test run
            val pullResult = flm.pullMetadata()
            assertNotNull("Pull metadata result should not be null", pullResult)
        }
    }

    @Test
    fun testFilterOperations() {
        val conf = createTestConfiguration()

        FilterListManager(conf).use { flm ->

            // Test getting stored filters metadata
            val filters = flm.getStoredFiltersMetadata()
            assertNotNull("Stored filters metadata should not be null", filters)

            // Create a custom filter
            val customFilterRules = "! Test filter\n||example.com^"
            val customFilter = flm.installCustomFilterFromString(
                downloadUrl = "",
                lastDownloadTime = System.currentTimeMillis() / 1000,
                isEnabled = true,
                isTrusted = true,
                filterBody = customFilterRules,
                customTitle = "Test Filter",
                customDescription = "A test filter for instrumented tests"
            )

            assertNotNull("Custom filter should be created successfully", customFilter)
            assertTrue("Custom filter ID should be valid", customFilter.id != 0)
            assertEquals("Custom filter title should match", "Test Filter", customFilter.title)

            // Test getting filter by ID
            val retrievedFilter = flm.getFullFilterListById(customFilter.id)
            assertNotNull("Retrieved filter should not be null", retrievedFilter)
            assertEquals("Retrieved filter ID should match", customFilter.id, retrievedFilter!!.id)

            // Test updating custom filter metadata
            val updateResult = flm.updateCustomFilterMetadata(
                id = customFilter.id,
                title = "Updated Test Filter",
                isTrusted = false
            )
            assertTrue("Updating custom filter metadata should succeed", updateResult)

            val activeRulesResult = flm.getActiveRulesRaw(listOf(customFilter.id))
            assertEquals(activeRulesResult.size, 1)
            assertEquals(activeRulesResult[0].filterId, customFilter.id)

            // Test getting rules count
            val rulesCount = flm.getRulesCount(listOf(customFilter.id))
            assertFalse("Rules count list should not be empty", rulesCount.isEmpty())

            // Test deleting the custom filter
            val deleteResult = flm.deleteCustomFilterLists(listOf(customFilter.id))
            assertTrue("Deleting custom filter should affect at least one row", deleteResult > 0)
        }
    }
}
