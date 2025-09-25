package com.adguard.flm.extensions

import com.adguard.flm.driver.RustResponse
import com.adguard.flm.logging.FlmLogger
import pbandk.Message
import pbandk.decodeFromByteArray

internal actual fun <T : Message> Message.Companion<T>.decodeFromResponse(response: RustResponse): T? {
    return try {
        return decodeFromByteArray(response.resultData ?: ByteArray(0))
    } catch (th: Throwable) {
        FlmLogger.error("The error occurred while decoding ${this} from response ${response.responseType}", th)
        null
    }
}
