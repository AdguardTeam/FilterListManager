package com.adguard.flm.driver

import com.adguard.flm.native.flm_free_response
import com.adguard.flm.support.RustResponseType
import kotlinx.cinterop.*
import com.adguard.flm.native.RustResponse as NativeRustResponse

/**
 * iOS implementation of RustResponse - pure data holder.
 *
 * This class serves as a simple container for data extracted from native responses.
 * All data extraction from native structures happens in the driver, and this class
 * only stores the pre-processed results and manages the native memory lifecycle.
 *
 * The class follows the same pattern as the Android implementation, where all data
 * is provided ready-to-use in the constructor, ensuring consistent behavior and
 * optimal performance across platforms.
 */
@OptIn(ExperimentalForeignApi::class, ExperimentalStdlibApi::class)
internal actual class RustResponse internal constructor(
    private val nativeResponseHandle: COpaquePointer,
    val resultData: ByteArray?,
    actual val ffiError: Boolean,
    actual val flmHandle: Long,
    actual val responseType: RustResponseType
) : AutoCloseable {

    actual override fun close() {
        flm_free_response(nativeResponseHandle.reinterpret())
    }
}
