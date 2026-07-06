package com.adguard.flm.extensions

import com.adguard.flm.driver.RustResponse
import com.adguard.flm.logging.FlmLogger
import com.adguard.flm.protobuf.AGOuterError
import com.squareup.wire.Message
import com.squareup.wire.ProtoAdapter

internal expect fun <T : Message<T, *>> ProtoAdapter<T>.decodeFromResponse(response: RustResponse): T?

internal fun <T : Message<T, *>> T.getOrProcessError(errorGetter: T.() -> AGOuterError?): T? {
    val error = errorGetter() ?: return this

    FlmLogger.error("The error occurred while processing ${this}, error message: ${error.message}")
    return null
}
