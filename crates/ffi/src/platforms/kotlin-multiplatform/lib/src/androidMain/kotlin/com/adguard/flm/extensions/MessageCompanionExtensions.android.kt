package com.adguard.flm.extensions

import com.adguard.flm.driver.RustResponse
import com.adguard.flm.logging.FlmLogger
import pbandk.Message
import pbandk.decodeFromByteBuffer

/**
 * Android implementation of protobuf message decoding from RustResponse.
 *
 * This extension function provides the Android-specific implementation for decoding
 * protobuf messages from native library responses. It safely handles the ByteBuffer
 * data from the RustResponse and converts it into the requested protobuf message type.
 *
 * The function includes error handling for corrupted or invalid protobuf data,
 * logging any decoding failures through the FlmLogger system.
 *
 * @param response The RustResponse containing the protobuf data to decode
 * @return The decoded protobuf message of type T, or null if decoding fails
 * @receiver The companion object of the protobuf message type to decode
 */
internal actual fun <T : Message> Message.Companion<T>.decodeFromResponse(response: RustResponse): T? {
    return try {
        decodeFromByteBuffer(response.resultDataBuffer ?: return null)
    } catch (th: Throwable) {
        FlmLogger.error("The error occurred while decoding ${this} from response ${response.responseType}")
        null
    }
}
