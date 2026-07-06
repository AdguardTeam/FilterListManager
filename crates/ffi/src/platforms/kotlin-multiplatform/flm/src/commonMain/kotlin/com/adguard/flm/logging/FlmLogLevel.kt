package com.adguard.flm.logging

/**
 * Represents the severity levels for log messages in the Filter List Manager library.
 *
 * FlmLogLevel defines a three-tier logging hierarchy that allows applications to
 * categorize and filter log messages based on their importance and severity.
 * These levels follow industry-standard logging conventions and are designed to
 * provide clear differentiation between routine information, potential issues,
 * and actual errors.
 *
 * ## Usage
 *
 * The log level is passed to the logging callback registered with [FlmLogger.setCallback],
 * allowing the host application to handle different severity levels appropriately:
 *
 * ```kotlin
 * FlmLogger.setCallback { level, message ->
 *     when (level) {
 *         FlmLogLevel.Info -> {
 *             // Log to info channel or ignore in production
 *             debugLogger.info(message)
 *         }
 *         FlmLogLevel.Warn -> {
 *             // Log warnings, might want to track these
 *             analyticsTracker.trackWarning(message)
 *             logger.warning(message)
 *         }
 *         FlmLogLevel.Error -> {
 *             // Always log errors, possibly report to crash analytics
 *             crashlytics.log(message)
 *             logger.error(message)
 *         }
 *     }
 * }
 * ```
 *
 * ## Level Filtering Example
 *
 * Applications can implement level-based filtering to control verbosity:
 *
 * ```kotlin
 * enum class AppLogLevel { DEBUG, INFO, WARNING, ERROR }
 *
 * val minLogLevel = if (BuildConfig.DEBUG) AppLogLevel.DEBUG else AppLogLevel.WARNING
 *
 * FlmLogger.setCallback { level, message ->
 *     val appLevel = when (level) {
 *         FlmLogLevel.Info -> AppLogLevel.INFO
 *         FlmLogLevel.Warn -> AppLogLevel.WARNING
 *         FlmLogLevel.Error -> AppLogLevel.ERROR
 *     }
 *
 *     if (appLevel >= minLogLevel) {
 *         logToConsole(level, message)
 *     }
 * }
 * ```
 *
 * @see FlmLogger for the logging system that uses these levels
 */
enum class FlmLogLevel {
    /**
     * Informational messages that highlight the progress of the application at a coarse-grained level.
     *
     * Info level messages are used for general informational purposes and typically include:
     * - Successful initialization of components
     * - Successful completion of significant operations
     * - State transitions in the filter list lifecycle
     * - Configuration changes
     * - Network requests and responses (successful cases)
     *
     * These messages are useful during development and debugging but may be filtered out
     * in production environments to reduce log verbosity.
     *
     * Example messages:
     * - "Filter list database initialized successfully"
     * - "Filter list 'EasyList' updated: 1234 rules loaded"
     * - "Starting synchronization of filter lists"
     */
    Info,

    /**
     * Warning messages that indicate potentially harmful situations or unexpected conditions.
     *
     * Warn level messages represent situations where the library can continue functioning
     * but something unexpected occurred that might require attention. These include:
     * - Deprecated feature usage
     * - Missing optional configurations
     * - Recoverable errors that were handled gracefully
     * - Performance degradation warnings
     * - Network timeouts that will be retried
     * - Data inconsistencies that can be auto-corrected
     *
     * Warnings should be reviewed as they might indicate configuration issues or
     * environmental problems that could escalate to errors if left unaddressed.
     *
     * Example messages:
     * - "Filter list update failed, will retry in 5 minutes"
     * - "Database migration detected inconsistent data, auto-correcting"
     * - "Network request timeout, falling back to cached data"
     * - "Filter list 'CustomList' has no rules, might be corrupted"
     */
    Warn,

    /**
     * Error messages that indicate serious problems that likely caused a failure.
     *
     * Error level messages represent failures that prevent normal operation of the library
     * or specific features. These require immediate attention and include:
     * - Unrecoverable exceptions
     * - Database corruption or access failures
     * - Critical network failures
     * - Invalid or corrupted filter list data
     * - Security or permission violations
     * - Resource exhaustion (memory, disk space)
     *
     * Errors typically indicate that manual intervention or application restart might
     * be necessary to recover. In production, these should always be logged and might
     * trigger alerts or crash reporting.
     *
     * Example messages:
     * - "Failed to open database: Permission denied"
     * - "Filter list parsing failed: Invalid syntax at line 42"
     * - "Out of memory while processing large filter list"
     * - "Critical: Database corruption detected, cannot recover"
     */
    Error,
}
