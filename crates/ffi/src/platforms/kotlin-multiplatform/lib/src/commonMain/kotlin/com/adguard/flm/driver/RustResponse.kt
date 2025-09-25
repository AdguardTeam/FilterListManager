package com.adguard.flm.driver

import com.adguard.flm.support.RustResponseType

/**
 * Platform-specific container for native library responses.
 *
 * This expect class defines the contract for responses returned by the native
 * Filter List Manager library. Each platform provides its own actual implementation
 * that handles platform-specific memory management and data representation.
 *
 * RustResponse serves as a bridge between the native Rust code and Kotlin,
 * carrying both the response data and metadata about the operation result.
 * It manages native memory resources and must be properly closed to prevent leaks.
 *
 * ## Response Types
 * The response can contain different types of data based on [responseType]:
 * - [RustResponseType.RustBuffer]: Contains serialized protobuf data
 * - [RustResponseType.FLMHandlePointer]: Contains a handle to a native object
 *
 * ## Memory Management
 * This class manages native memory that must be explicitly released. The response
 * should be used with Kotlin's `use` extension or closed explicitly:
 *
 * ```kotlin
 * response.use { r ->
 *     if (!r.ffiError) {
 *         // Process response data safely
 *     }
 * } // Automatically cleaned up here
 * ```
 *
 * ## Error Handling
 * Always check [ffiError] before processing response data. If true, the response
 * contains error information instead of the expected result.
 */
@OptIn(ExperimentalStdlibApi::class)
internal expect class RustResponse : AutoCloseable {

    /**
     * Indicates whether the native operation encountered an error.
     *
     * When true, the response contains error information and should not be
     * processed as successful result data. The error details can typically
     * be extracted from the response data as a serialized error message.
     */
    val ffiError: Boolean

    /**
     * Native handle value for FLMHandlePointer response types.
     *
     * This property contains a handle (pointer) to a native Filter List Manager
     * instance when [responseType] is [RustResponseType.FLMHandlePointer].
     * For other response types, this value should be ignored.
     *
     * The handle is used for subsequent operations on the native object and
     * must be properly freed when no longer needed.
     */
    val flmHandle: Long

    /**
     * The type of data contained in this response.
     *
     * Determines how the response data should be interpreted:
     * - [RustResponseType.RustBuffer]: Response contains serialized data
     * - [RustResponseType.FLMHandlePointer]: Response contains a native handle
     */
    val responseType: RustResponseType



    /**
     * Releases the native memory associated with this response.
     *
     * This method must be called to free native resources when the response
     * is no longer needed. After calling close(), this response instance
     * becomes invalid and should not be accessed.
     */
    override fun close()
}
