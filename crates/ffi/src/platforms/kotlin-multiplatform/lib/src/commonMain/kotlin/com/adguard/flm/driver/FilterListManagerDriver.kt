package com.adguard.flm.driver

import com.adguard.flm.support.FilterListManagerConstants
import com.adguard.flm.support.FFIMethod
import com.adguard.flm.protobuf.Configuration

/**
 * Platform-specific driver for interfacing with the native Filter List Manager library.
 *
 * This expect class defines the contract for platform-specific implementations that provide
 * low-level access to the native Rust library through FFI (Foreign Function Interface).
 * Each platform (Android, iOS) provides its own actual implementation using the appropriate
 * native interop mechanism:
 *
 * - **Android**: Uses JNI (Java Native Interface) to communicate with the native library
 * - **iOS**: Uses Kotlin/Native CInterop to bridge to the native code
 *
 * The driver manages the lifecycle of native resources and ensures proper cleanup through
 * the [AutoCloseable] interface. It serves as the foundation layer that enables the
 * higher-level [FilterListManager] API to work consistently across platforms.
 *
 * ## Thread Safety
 * The driver implementations are **not thread-safe**. Multiple threads should not access
 * the same driver instance concurrently without external synchronization.
 *
 * ## Resource Management
 * This class manages native resources that must be explicitly released. Always use
 * [close] or Kotlin's `use` extension to ensure proper cleanup.
 *
 * @constructor Creates a new driver instance with the specified configuration
 * @param configuration The configuration to initialize the native library with
 * @throws RuntimeException if the native library fails to initialize
 */
@OptIn(ExperimentalStdlibApi::class)
internal expect class FilterListManagerDriver
@Throws(RuntimeException::class) constructor(configuration: Configuration)
    : AutoCloseable
{

    companion object {
        /**
         * Retrieves the default configuration from the native library.
         *
         * This method provides access to the default configuration that can be used
         * to initialize a [FilterListManager] instance. The configuration includes
         * default values for all required settings.
         *
         * @return RustResponse containing the serialized default configuration,
         *         or null if the native call fails
         */
        fun getDefaultConfiguration(): RustResponse?

        /**
         * Retrieves important constants from the native library.
         *
         * These constants define special filter and group IDs that have specific
         * meanings in the Filter List Manager system, such as the user rules filter ID
         * and custom filters group ID.
         *
         * @return FilterListManagerConstants containing the library constants
         */
        fun getConstants(): FilterListManagerConstants
    }



    /**
     * Executes a Filter List Manager operation on the native instance.
     *
     * This method serves as the primary interface for invoking operations on the
     * native Filter List Manager instance. It serializes the method call and input
     * data, sends them to the native library, and returns the response.
     *
     * All high-level FilterListManager operations ultimately go through this method,
     * making it the core bridge between Kotlin and the native Rust implementation.
     *
     * ## Error Handling
     * If the native call fails, the returned [RustResponse] will have [RustResponse.ffiError]
     * set to true, and error details can be extracted from the response data.
     *
     * @param method The FFI method enum value specifying which operation to perform
     * @param data Input data serialized as protobuf byte array
     * @return RustResponse containing the operation result, handle, or error information
     * @throws Exception if the native call cannot be completed due to system-level issues
     */
    fun call(method: FFIMethod, data: ByteArray): RustResponse

    /**
     * Releases all native resources associated with this driver instance.
     *
     * This method must be called when the driver is no longer needed to prevent
     * native memory leaks. After calling close(), this driver instance becomes
     * invalid and should not be used for any further operations.
     *
     * The implementation will free the native Filter List Manager handle and
     * any associated resources. This operation is irreversible.
     *
     * ## Usage
     * It's recommended to use Kotlin's `use` extension function to ensure
     * automatic cleanup:
     * ```kotlin
     * FilterListManagerDriver(config).use { driver ->
     *     // Use driver safely
     * } // Automatically closed here
     * ```
     */
    override fun close()
}
