# FilterListManager Kotlin Multiplatform Library

A Kotlin Multiplatform library providing unified access to the Filter List Manager functionality across Android and iOS platforms.

## Overview

FilterListManager is a cross-platform library designed for managing content-blocking filter lists in mobile applications. It provides a comprehensive API for downloading, storing, updating, and managing filter lists and their associated rules.

### Key Features

- **Cross-platform**: Single codebase works on Android and iOS
- **Native performance**: Direct integration with Rust-based native library via FFI
- **Comprehensive API**: Full lifecycle management of filter lists
- **Memory safety**: Automatic resource management with proper cleanup
- **Flexible logging**: Customizable logging integration
- **Protobuf communication**: Efficient binary serialization for native calls

## Architecture

The library follows a layered architecture:

```
┌─────────────────────────────────────┐
│          FilterListManager          │  ← High-level API
├─────────────────────────────────────┤
│      Protobuf Message Layer         │  ← Type-safe messaging
├─────────────────────────────────────┤
│     FilterListManagerDriver         │  ← Platform abstraction
├─────────────────────────────────────┤
│    Platform-specific FFI Layer      │  ← JNI (Android) / CInterop (iOS)
├─────────────────────────────────────┤
│         Native Rust Library         │  ← Core implementation
└─────────────────────────────────────┘
```

### Technology Stack

- **Kotlin Multiplatform**: Shared business logic across platforms
- **FFI (Foreign Function Interface)**: Bridge to native Rust library
- **Protobuf**: Efficient binary serialization for cross-language communication
- **JNI**: Android native integration
- **Kotlin/Native CInterop**: iOS native integration

## Installation

### Gradle Setup

Add the library to your `build.gradle.kts`:

```kotlin
kotlin {
    // Your existing platform configurations

    sourceSets {
        val commonMain by getting {
            dependencies {
                implementation("com.adguard.flm:filter-list-manager-kmp:$flm_version")
            }
        }
    }
}
```

### Platform Requirements

**Android:**
- Minimum SDK: 21+
- Native library: `libfilter_list_manager_jni.so` (included in the KMP library)

**iOS:**
- iOS 13.0+
- This library integrates only with Kotlin Multiplatform projects. For iOS, you need to configure your KMP module to generate the iOS framework, which will include the FilterListManager functionality. The actual iOS framework is built by your project's KMP configuration.

## Quick Start

### 1. Basic Setup

```kotlin
import com.adguard.flm.FilterListManager
import com.adguard.flm.logging.FlmLogger

// Set up logging (optional but recommended)
FlmLogger.setCallback { level, message, throwable ->
    when (level) {
        FlmLogLevel.Info -> println("FLM-INFO: $message")
        FlmLogLevel.Warn -> println("FLM-WARN: $message")
        FlmLogLevel.Error -> {
            println("FLM-ERROR: $message")
            throwable?.printStackTrace()
        }
    }
}

// Get default configuration
val config = FilterListManager.defaultConfiguration?.copy {
    appName = "MyApp"
    version = "1.0.0"
    workingDirectory = "/path/to/app/data"
    locale = "en"
    // Configure other options as needed
}

// Create FilterListManager instance
val manager = config?.let { FilterListManager.create(it) }
    ?: throw IllegalStateException("Failed to create FilterListManager")
```

### 2. Initialize Database

```kotlin
// The database is initialized automatically by default
// Manual initialization is only needed if autoLiftUpDatabase=false in config

manager.use { flm ->
    // Database is ready to use
    val version = flm.getDatabaseVersion()
    println("Database version: $version")
}
```

### 3. Download Filter Metadata

```kotlin
manager.use { flm ->
    // Download latest filter metadata from remote server
    val result = flm.pullMetadata()
    result?.let {
        println("Filters added: ${it.filtersAdded}")
        println("Filters removed: ${it.filtersRemoved}")
    }
}
```

## Common Use Cases

### Working with Filter Groups and Tags

```kotlin
manager.use { flm ->
    // Get all available filter groups
    val groups = flm.getAllGroups()
    groups?.forEach { group ->
        println("Group: ${group.name} (${group.displayNumber})")
    }

    // Get all available tags
    val tags = flm.getAllTags()
    tags?.forEach { tag ->
        println("Tag: ${tag.name}")
    }
}
```

### Installing and Managing Filters

```kotlin
manager.use { flm ->
    // Install specific filters
    val filterIds = listOf(1, 2, 3) // EasyList, AdGuard Base, etc.
    flm.installFilterLists(filterIds, isInstalled = true)

    // Enable installed filters
    flm.enableFilterLists(filterIds, isEnabled = true)

    // Get filter metadata
    val filters = flm.getStoredFiltersMetadata()
    filters?.forEach { filter ->
        println("${filter.title}: enabled=${filter.isEnabled}")
    }
}
```

### Getting Active Filter Rules

```kotlin
manager.use { flm ->
    // Get all active filtering rules
    val rules = flm.getActiveRules()
    rules?.forEach { ruleInfo ->
        println("Filter ${ruleInfo.filterId}: ${ruleInfo.rulesCount} rules")
    }

    // Get rules as raw strings for specific filters
    val rawRules = flm.getFilterRulesAsStrings(listOf(1, 2))
    rawRules?.forEach { filterRules ->
        println("Filter ${filterRules.filterId} rules:")
        println(filterRules.rules)
    }
}
```

### Custom Filter Lists

```kotlin
manager.use { flm ->
    // Install custom filter from URL
    val customFilter = flm.installCustomFilterList(
        downloadUrl = "https://example.com/my-custom-filter.txt",
        isTrusted = true,
        title = "My Custom Filter",
        description = "Custom rules for my app"
    )

    // Install custom filter from string content
    val filterFromString = flm.installCustomFilterFromString(
        downloadUrl = "local://my-filter",
        lastDownloadTime = System.currentTimeMillis(),
        isEnabled = true,
        isTrusted = true,
        filterBody = """
            ||example.com^
            ||ads.example.com^
        """.trimIndent(),
        customTitle = "Local Custom Filter"
    )

    // Update custom filter rules
    customFilter?.let { filter ->
        val updatedRules = FilterListRules(
            filterId = filter.id,
            rules = "||new-blocked-domain.com^"
        )
        flm.saveCustomFilterRules(updatedRules)
    }
}
```

### Updating Filters

```kotlin
manager.use { flm ->
    // Update all filters (respecting expiration times)
    val updateResult = flm.updateFilters()
    updateResult?.let { result ->
        println("Updated ${result.updatedFilters.size} filters")
        result.updatedFilters.forEach { update ->
            println("${update.title}: ${update.rulesAdded} rules added")
        }
    }

    // Force update specific filters (ignore expiration)
    val forceResult = flm.forceUpdateFiltersByIds(
        ids = listOf(1, 2, 3),
        looseTimeout = 30000 // 30 seconds timeout
    )
}
```

## Configuration Options

### Basic Configuration

```kotlin
val config = FilterListManager.defaultConfiguration?.copy {
    // Application identification
    appName = "MyApp"
    version = "1.0.0"

    // File system paths
    workingDirectory = context.filesDir.absolutePath

    // Localization
    locale = "en_US"

    // Database behavior
    autoLiftUpDatabase = true  // Automatically initialize database

    // Update behavior
    filtersUpdateServerUrl = "https://filters.adtidy.org/..."
    metadataUpdateServerUrl = "https://filters.adtidy.org/..."

    // Timeouts and limits
    networkTimeout = 30
    maxFiltersToUpdate = 100
}
```

### Advanced Configuration

```kotlin
val config = FilterListManager.defaultConfiguration?.copy {
    // Custom filter update intervals
    filtersUpdateIntervalMs = 3600000 // 1 hour

    // Memory management
    useInMemoryDatabase = false

    // Development options
    verboseLog = true

    // Custom user agent
    userAgent = "MyApp/1.0 FilterListManager"
}
```

## Logging Integration

### Basic Logging

```kotlin
FlmLogger.setCallback { level, message, throwable ->
    val tag = "FilterListManager"
    when (level) {
        FlmLogLevel.Info -> Log.i(tag, message)
        FlmLogLevel.Warn -> Log.w(tag, message)
        FlmLogLevel.Error -> {
            if (throwable != null) {
                Log.e(tag, message, throwable)
            } else {
                Log.e(tag, message)
            }
        }
    }
}
```

### Advanced Logging with Analytics

```kotlin
FlmLogger.setCallback { level, message, throwable ->
    // Log to console/system
    systemLogger.log(level, "FLM", message, throwable)

    // Track important events
    when (level) {
        FlmLogLevel.Error -> {
            // Report errors to crash analytics
            analytics.recordError("FilterListManager", message, throwable)
        }
        FlmLogLevel.Warn -> {
            // Track warnings for monitoring
            analytics.trackEvent("FLM_Warning", mapOf("message" to message))
        }
        else -> { /* Optional: track info events */ }
    }
}
```

## Platform-Specific Considerations

### Android

**Permissions:**
- `INTERNET`: Required for downloading filters
- `WRITE_EXTERNAL_STORAGE`: If using external storage for database

**ProGuard/R8:**
```proguard
# Keep native methods
-keepclasseswithmembers class com.adguard.flm.** {
    native <methods>;
}

# Keep protobuf classes
-keep class com.adguard.flm.protobuf.** { *; }
```

### iOS

**Framework generation:**
Configure your KMP module's `build.gradle.kts` to generate iOS framework:

```kotlin
kotlin {
    listOf(
        iosX64(),
        iosArm64(),
        iosSimulatorArm64()
    ).forEach {
        it.binaries.framework {
            baseName = "YourAppFramework"
            // Your code to include the framework
        }
    }
}
```

The generated framework can then be integrated into your iOS project using Xcode or Swift Package Manager.

## Error Handling

### Defensive Programming

```kotlin
manager.use { flm ->
    try {
        // Always check results for null
        val filters = flm.getStoredFiltersMetadata()
        if (filters != null) {
            // Process filters safely
            processFilters(filters)
        } else {
            // Handle failure case
            handleError("Failed to get filter metadata")
        }
    } catch (e: Exception) {
        // Handle unexpected exceptions
        logger.error("Unexpected error in FilterListManager", e)
    }
}
```

### Common Error Patterns

```kotlin
// Check for initialization errors
val manager = FilterListManager.create(config)
if (manager == null) {
    throw IllegalStateException("FilterListManager initialization failed")
}

// Validate responses before use
manager.use { flm ->
    val result = flm.pullMetadata()
    if (result == null) {
        // Network error, database error, or other failure
        // Check logs for details
        return@use
    }

    // Safe to use result
    println("Update completed successfully")
}
```

## Best Practices

### 1. Resource Management

```kotlin
// Always use 'use' block for automatic cleanup
FilterListManager.create(config)?.use { flm ->
    // Perform operations
} // Automatically closed here

// Or manage lifecycle manually
val manager = FilterListManager.create(config)
try {
    // Use manager
} finally {
    manager?.close()
}
```

### 2. Thread Safety

```kotlin
// FilterListManager is NOT thread-safe
// Use single thread or external synchronization
class FilterManagerWrapper {
    private val manager = FilterListManager.create(config)
    private val mutex = Mutex()

    suspend fun safeOperation() = mutex.withLock {
        manager?.use { flm ->
            // Thread-safe operation
            flm.getStoredFiltersMetadata()
        }
    }
}
```

### 3. Update Strategy

```kotlin
// Update filters periodically, not on every app launch
class FilterUpdateManager {
    private val updateInterval = 24 * 60 * 60 * 1000L // 24 hours

    suspend fun updateIfNeeded() {
        val lastUpdate = getLastUpdateTime()
        if (System.currentTimeMillis() - lastUpdate > updateInterval) {
            manager?.use { flm ->
                val result = flm.updateFilters(
                    ignoreFiltersExpiration = false,
                    looseTimeout = 30000,
                    ignoreFiltersStatus = false
                )
                if (result != null) {
                    saveLastUpdateTime(System.currentTimeMillis())
                }
            }
        }
    }
}
```

## Troubleshooting

### Common Issues

**1. Manager creation fails:**
- Check that native libraries are properly bundled
- Verify working directory exists and is writable
- Check logs for initialization errors

**2. Network operations fail:**
- Ensure internet permission (Android)
- Check network connectivity
- Verify server URLs in configuration

**3. Database corruption:**
- Delete database file and reinitialize
- Check disk space availability
- Verify file system permissions

**4. Memory leaks:**
- Always call `close()` or use `use` blocks
- Don't store long-lived references to responses
- Monitor native memory usage

### Debug Mode

```kotlin
// Enable verbose logging in development
val debugConfig = FilterListManager.defaultConfiguration?.copy {
    verboseLog = true
    networkTimeout = 60 // Longer timeout for debugging
}
```

## Migration Guide

When updating from previous versions:

1. Check changelog for breaking changes
2. Update configuration parameters if needed
3. Test filter list compatibility
4. Verify logging integration still works

## API Reference

For detailed API documentation, refer to the KDoc comments in the source code. Key classes:

- `FilterListManager`: Main API entry point
- `FilterListManagerDriver`: Platform-specific native interface
- `FlmLogger`: Logging system
- `Configuration`: Library configuration
- `RustResponse`: Native response container