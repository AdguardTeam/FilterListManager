package com.adguard.flm.jni

import java.io.Closeable
import java.nio.ByteBuffer

/**
 * Container for rust-formed responses into external world
 * It is Closeable, use Kotlin ".use" or close() explicitly
 */
internal data class RustResponse(
    private val rustResponseHandle: Long,
    // This buffer points directly into native memory, and should not escape ".use" block.
    val resultData: ByteBuffer?,
    val ffiError: Boolean,
    val flmHandle: Long,
    val responseType: RustResponseType
) : Closeable {
    override fun close() {
        NativeInterface.flmFreeResponse(rustResponseHandle)
    }
}
