package com.adguard.flm.extensions

import com.adguard.flm.driver.RustResponse
import com.adguard.flm.logging.FlmLogger
import com.adguard.flm.protobuf.AGOuterError
import pbandk.Message

/**
 * Platform-specific protobuf message decoding from native responses.
 *
 * This expect function defines the contract for decoding protobuf messages from
 * native library responses. Each platform provides its own actual implementation
 * that handles the platform-specific data format and memory management.
 *
 * The function safely extracts protobuf data from the response and deserializes
 * it into the requested message type, handling any decoding errors gracefully.
 *
 * @param response The RustResponse containing protobuf data to decode
 * @return The decoded protobuf message of type T, or null if decoding fails
 * @receiver The companion object of the protobuf message type to decode
 */
internal expect fun <T : Message> Message.Companion<T>.decodeFromResponse(response: RustResponse): T?

/**
 * Processes protobuf messages with error handling.
 *
 * This extension function provides a standardized way to handle error responses
 * from protobuf messages that contain optional error fields. If the message
 * contains an error, it logs the error details and returns null, otherwise
 * returns the original message for further processing.
 *
 * This pattern is commonly used throughout the FilterListManager API to
 * provide consistent error handling for all operations.
 *
 * ## Usage Example
 * ```kotlin
 * val response = SomeResponse.decodeFromResponse(rustResponse)
 * val result = response?.getOrProcessError(SomeResponse::error)
 * if (result != null) {
 *     // Process successful response
 * }
 * ```
 *
 * @param errorGetter A function that extracts the error field from this message
 * @return This message if no error is present, null if an error was found and logged
 * @receiver The protobuf message to check for errors
 */
internal fun <T : Message> T.getOrProcessError(errorGetter: T.() -> AGOuterError?): T? {
    val error = errorGetter() ?: return this

    FlmLogger.error("The error occurred while processing ${this}, error message: ${error.message}")
    return null
}
