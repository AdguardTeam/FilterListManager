package com.adguard.flm.driver

import com.adguard.flm.support.RustResponseType
import java.nio.ByteBuffer

/**
 * Android implementation of RustResponse container for native library responses.
 *
 * This class represents responses from the native Rust library and contains both
 * the response data and metadata about the operation result. It manages native
 * memory resources and must be properly closed to prevent memory leaks.
 *
 * The response can contain different types of data depending on the [responseType]:
 * - [RustResponseType.RustBuffer]: Contains serialized data in [resultDataBuffer]
 * - [RustResponseType.FLMHandlePointer]: Contains a native handle in [flmHandle]
 *
 * **Important**: This class implements [AutoCloseable] and should be used with
 * Kotlin's `use` extension or explicitly closed to release native memory.
 *
 * @constructor Creates a new RustResponse instance (internal use only)
 * @param rustResponseHandle Handle to the native response object that must be freed
 * @param resultDataBuffer ByteBuffer containing response data, may be null
 * @param ffiError True if the native operation encountered an error
 * @param flmHandle Native handle value for FLMHandlePointer responses
 * @param responseType The type of response data contained in this object
 */
internal actual class RustResponse internal constructor(
    private val rustResponseHandle: Long,
    val resultDataBuffer: ByteBuffer?,
    actual val ffiError: Boolean,
    actual val flmHandle: Long,
    actual val responseType: RustResponseType
) : AutoCloseable {

    actual override fun close() {
        nativeFreeResponse(rustResponseHandle)
    }



    /**
     * Releases the native memory associated with this response.
     *
     * This JNI method frees the native memory allocated for this response object.
     * Called automatically by [close] and should not be called directly.
     *
     * @param handle The native response handle to be freed
     */
    external fun nativeFreeResponse(handle: Long)
}
