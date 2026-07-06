package com.adguard.flm

import com.adguard.flm.driver.FilterListManagerDriver
import com.adguard.flm.logging.FlmLogger
import com.adguard.flm.protobuf.Configuration

/** Factory for creating [FlmAdapter] instances backed by a native Filter List Manager driver. */
object FlmAdapterFactory {

    /** Creates an [FlmAdapter] for the given [configuration], or returns `null` if initialization fails. */
    fun create(configuration: Configuration): FlmAdapter? {
        try {
            val driver = FilterListManagerDriver(configuration)
            return FlmAdapterImpl(driver)
        } catch (th: Throwable) {
            FlmLogger.error("The error occurred while creating FlmAdapter", th)
        }
        return null
    }
}
