package com.adguard.flm.extensions

import com.adguard.flm.driver.RustResponse
import com.adguard.flm.logging.FlmLogger
import com.squareup.wire.Message
import com.squareup.wire.ProtoAdapter

internal actual fun <T : Message<T, *>> ProtoAdapter<T>.decodeFromResponse(response: RustResponse): T? {
    return try {
        decode(response.resultData ?: return null)
    } catch (th: Throwable) {
        FlmLogger.error("The error occurred while decoding from response ${response.responseType}", th)
        null
    }
}
