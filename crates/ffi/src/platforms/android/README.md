# Filter List Manager for Android

This directory contains the Android bindings for the Filter List Manager library.
It provides a JNI interface to the Rust-based Filter List Manager core.

## Prerequisites

Before building the Android library, ensure you have the following prerequisites installed:

1. Android Studio or command-line tools with NDK installed
2. Rust 1.85 with Android targets added
    ```bash
    rustup target add aarch64-linux-android
    rustup target add armv7-linux-androideabi
    rustup target add i686-linux-android
    rustup target add x86_64-linux-android
    cargo install cargo-ndk
    ```
3. This project requires at least Java 11 for build

## Building the Library

To build the Android library, run the following command in this directory:

```bash
./gradlew assembleRelease
```

The built AAR file will be available in the `lib/build/outputs/aar/` directory.

## Publishing the Library

To publish the library to your local Maven repository, run:

```bash
./gradlew publish
```

This will make the library available for use in your local projects through Maven coordinates.

## Using the Library

### Adding the Library to Your Project

#### Option 1: Using Maven coordinates (recommended)

Add the following to your app's `build.gradle`:

```gradle
dependencies {
    implementation "com.adguard.flm:adguard-flm:2.0.0"
}
```

This approach automatically handles all transitive dependencies.

#### Option 2: Using the AAR file directly

1. Add the AAR file to your project's `libs` directory
2. Add the following to your app's `build.gradle`:
   ```gradle
   dependencies {
       implementation fileTree(dir: 'libs', include: ['*.aar'])

       // Required dependencies for the Filter List Manager when AAR is included by file name
       implementation "com.google.protobuf:protobuf-javalite:3.21.12"
       implementation "com.google.protobuf:protobuf-kotlin-lite:3.21.12"
   }
   ```

**Note:** When including an AAR file directly, the transitive dependencies are not automatically
added to your project. You must manually add all the required dependencies as shown above.

### Create and setup configuration for library facade

```kotlin
// Every instance of FilterListManager must have its own configuration
val conf = FilterListManager.defaultConfiguration.copy {
    // Application-specific configuration
    appName = "my-flm-app"
    version = "1.0"
    workingDirectory = context.cacheDir.absolutePath

    // Sets urls for filters indices
    metadataUrl = "https://filters.adtidy.org/extension/safari/filters.json"
    metadataLocalesUrl = "https://filters.adtidy.org/extension/safari/filters_i18n.json"

    // Sets locale. Will be used for returning localized strings for filters,
    // groups, and tags, where applicable
    locale = "pt_PT"

    // Sets type of filters lists
    // By default, FilterListType.STANDARD will be selected
    filterListType = FilterListType.DNS
}

// Creates facade instance
val flm = FilterListManager(conf)

// Working with instance
...

// Instance should be closed after usage
flm.close()
```

### How to create and fill up standard filters database

```kotlin
// Creates and configures the database. Populates the database with information
// from the filter indexes (filters metadata), the paths to which are specified
// in the configuration.
// In addition, this method applies migrations that have not yet been applied.
// See the liftUpDatabase method for details on "lifting" a database.
flm.pullMetadata()

// Then, downloads the contents of the filters
flm.updateFilters(ignoreFiltersExpiration = true, looseTimeout = 0, ignoreFiltersStatus = true)
```

> [!NOTE]
> By default, the application operates with a database located in the directory specified in the configuration's
> `workingDirectory`, and the database file name is generated based on the format `agflm_{configuration.filterListType}`.
> For standard filters, the file path will be `$workingDirectory/agflm_standard.db`.

### Database scheme updates

Database schema updates (migrations) are possible using the `flm.liftUpDatabase()` method.
The method "raises" the state of the database to the working state.

**If the database doesn't exist:**
- Creates database
- Rolls up the schema
- Rolls migrations
- Performs bootstrap.

**If the database is an empty file:**
- Rolls the schema
- Rolls migrations
- Performs bootstrap.

... and so on.

#### Usage notes

First connection to database almost always "lift" the database.
So you need this only in special cases, like old database backups.

### Operations with custom filters

The library categorizes all filters into three types:

1. **Index Filters** - Filters created by parsing the index (registry).
2. **Custom Filters** - Filters added (and edited) by the user using the library's methods.
3. **Special Filters** - Custom filters preconfigured by the library's scripts.

```kotlin
// Installs a custom filter
val customFilter = flm.installCustomFilterList(
    downloadUrl = "https://example.com/custom_filter.txt",
    isTrusted = true,  // The filter list is marked as trusted
    title = "Custom title",  // Optional
    description = "Custom description"  // Optional
)

// Edit metadata
flm.updateCustomFilterMetadata(
    id = customFilter.id,
    title = "new title",
    isTrusted = false  // The filter list is marked as not trusted
)

// Turn on this filter
flm.enableFilterLists(ids = listOf(customFilter.id), isEnabled = true)

// Remove this filter
flm.deleteCustomFilterLists(ids = listOf(customFilter.id))
```

### Installing a custom filter from a string instead of downloading it

```kotlin
val stringContents = """
! Checksum: ecbiyIyplBZKLeNzi64pGA
...
! JS API START
#%#var AG_onLoad=function(func){if(document.readyState==="complete"||document.readyState==="interactive")func();else
...
""".trimIndent()

flm.installCustomFilterFromString(
    downloadUrl = "",  // download url
    lastDownloadTime = 1719505304L,  // lastDownloadTime value. Explanation: Can we update filter? Answer: (filter.lastDownloadTime + filter.expires < now())
    isEnabled = true,  // Enabled
    isTrusted = true,  // Trusted
    filterBody = stringContents,  // Filter body
    customTitle = null,  // Filter title - Optional
    customDescription = null   // Filter description - Optional
)
```

### Operations with custom filters rules

```kotlin
// Saves the structure containing the filter rules
flm.saveCustomFilterRules(/* FilterListRules */ rulesForNewLocalCustomFilter)

// You can save only disabled rules for the filter list
flm.saveDisabledRules(id = filter.id, disabledRules = /* List<String> */ disabledRulesList)
```

### Get operations

```kotlin
// Retrieves all filters metadata from the database **with** their rules
// Returns List<FullFilterList>
flm.getFullFilterLists()

// Retrieves a filter metadata by its ID from the database **with** its rules
// Returns Optional<FullFilterList>
flm.getFullFilterListById(id)

// Retrieves all enabled filters as ActiveRulesInfo
flm.getActiveRules()

// Retrieves all filters metadata from the database **without** their rules
// Returns List<StoredFilterMetadata>
flm.getStoredFiltersMetadata()

// Retrieves a filter metadata by its ID from the database **without** its rules
// Returns Optional<StoredFilterMetadata>
flm.getStoredFilterMetadataById(id)
```

### Getting constants

```kotlin
val constants = FilterListManager.constants
println("User rules ID: ${constants.userRulesId}")
println("Custom group ID: ${constants.customGroupId}")
println("Special group ID: ${constants.specialGroupId}")
println("Smallest filter ID: ${constants.smallestFilterId}")
```

### Error Handling

The library uses exceptions to handle errors:

```kotlin
import com.adguard.flm.protobuf.configuration
import com.adguard.flm.exceptions.FilterListManagerException

try {
    // Create an invalid configuration
    val invalidConf = configuration {
        // Empty configuration without required fields
    }

    // This will throw an exception
    FilterListManager(invalidConf).close()
} catch (e: FilterListManagerException) {
    // Handle the exception
    println("Error: ${e.message}")
}
```

## Testing

The library includes instrumented tests that can be run on an Android device or emulator:

```bash
./gradlew connectedAndroidTest
```

See `lib/src/androidTest/java/com/adguard/flm/FilterListManagerAndroidTest.kt` for example test
cases.

## Architecture

The Android library consists of:

1. **JNI Layer** - C++ code that bridges between Java and Rust
2. **Kotlin API** - High-level API for using the library in Android applications
3. **Protobuf Definitions** - For type-safe configuration and communication

The library uses JNI to call the Rust functions from Java/Kotlin, and the Rust code is compiled for
all supported Android architectures (arm64-v8a, armeabi-v7a, x86, x86_64).
