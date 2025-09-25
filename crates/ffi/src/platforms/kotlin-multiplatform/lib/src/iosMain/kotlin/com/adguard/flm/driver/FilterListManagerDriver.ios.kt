package com.adguard.flm.driver

import com.adguard.flm.native.*
import com.adguard.flm.protobuf.Configuration
import com.adguard.flm.support.FFIMethod
import com.adguard.flm.support.FilterListManagerConstants
import com.adguard.flm.support.RustResponseType
import kotlinx.cinterop.*
import platform.posix.memcpy
import com.adguard.flm.native.RustResponse as NativeRustResponse
import com.adguard.flm.native.RustResponseType as NativeRustResponseType
import com.adguard.flm.logging.FlmLogger
import pbandk.encodeToByteArray

/**
 * iOS implementation of FilterListManagerDriver using CInterop.
 * This class manages the native handle state and provides low-level access to the native library.
 */
@OptIn(ExperimentalForeignApi::class, ExperimentalStdlibApi::class)
internal actual class FilterListManagerDriver
@Throws(RuntimeException::class) actual constructor(configuration: Configuration)
    : AutoCloseable
{
    private val handle: Long



    init {
        val configBytes = configuration.encodeToByteArray()

        val nativeResponse = initializeNative(configBytes)
        val response = createRustResponseOrCloseNative(nativeResponse)
            ?: throw RuntimeException("Rust response hasn't been created")

        handle = response.use {
            if (it.ffiError) {
                throw RuntimeException("Failed to initialize FilterListManager: FFI error")
            }

            if (it.responseType != RustResponseType.FLMHandlePointer) {
                throw RuntimeException("Expected FLMHandlePointer response type, got: ${it.responseType}")
            }

            it.flmHandle
        }
    }

    actual fun call(method: FFIMethod, data: ByteArray): RustResponse {
        @Suppress("DEPRECATION") // Well, when 'byValue' is removed, we will figure it out
        val nativeMethod = com.adguard.flm.native.FFIMethod.Companion.byValue(method.ordinal.toUInt())

        val nativeResponse = data.usePinned { pinned ->
            val dataPtr = if (data.isNotEmpty()) pinned.addressOf(0).reinterpret<UByteVar>() else null

            flm_call_protobuf(
                handle.toCPointer(),
                nativeMethod,
                dataPtr,
                data.size.convert()
            )
        } ?: throw RuntimeException("flm_call_protobuf function returned null")

        return createRustResponseOrCloseNative(nativeResponse)
            ?: throw RuntimeException("Native response hasn't been created")
    }

    actual override fun close() {
        flm_free_handle(handle.toCPointer())
    }



    private fun initializeNative(configBytes: ByteArray): COpaquePointer {
        return configBytes.usePinned { pinned ->
            flm_init_protobuf(
                pinned.addressOf(0).reinterpret(),
                configBytes.size.convert()
            )
        } ?: throw RuntimeException("flm_init_protobuf returned null")
    }



    // Static members section

    actual companion object {

        actual fun getDefaultConfiguration(): RustResponse? {
            val nativeResponse = flm_default_configuration_protobuf()
            if (nativeResponse == null) {
                FlmLogger.error("flm_default_configuration_protobuf returned null")
                return null
            }

            return createRustResponseOrCloseNative(nativeResponse)
        }

        actual fun getConstants(): FilterListManagerConstants {
            memScoped {
                val constants = flm_get_constants().getPointer(this).pointed

                return FilterListManagerConstants(
                    userRulesId = constants.user_rules_id,
                    customGroupId = constants.custom_group_id,
                    specialGroupId = constants.special_group_id,
                    smallestFilterId = constants.smallest_filter_id
                )
            }
        }



        /**
         * Extracts data from native response and creates RustResponse data holder.
         *
         * This function does all the heavy lifting of data extraction from the native
         * RustResponse structure. It replaces the previous approach where RustResponse
         * itself would extract data on-demand through getters.
         *
         * @param nativePointer Pointer to the native RustResponse structure
         * @return RustResponse with all data pre-extracted and ready to use
         */
        private fun createRustResponseOrCloseNative(nativePointer: COpaquePointer): RustResponse? {
            val nullWithClosingNativeRustResponse: () -> RustResponse? = {
                RustResponse(
                    nativePointer,
                    null, false,
                    0, RustResponseType.RustBuffer
                ).close()
                null
            }

            val response = nativePointer.reinterpret<NativeRustResponse>().pointed

            val responseType = when (response.response_type) {
                NativeRustResponseType.RustBuffer -> RustResponseType.RustBuffer
                NativeRustResponseType.FLMHandlePointer -> RustResponseType.FLMHandlePointer
                else -> return nullWithClosingNativeRustResponse()
            }

            val resultDataPointer = response.result_data
            val flmHandle = when (responseType) {
                RustResponseType.FLMHandlePointer -> resultDataPointer?.toLong() ?: 0L
                else -> 0L
            }

            val resultData = if (resultDataPointer == null || response.result_data_len == 0UL) {
                null
            } else {
                val dataPtr = resultDataPointer.reinterpret<ByteVar>()
                val size = response.result_data_len.toInt()

                ByteArray(size).apply {
                    usePinned { pinned ->
                        memcpy(pinned.addressOf(0), dataPtr, size.convert())
                    }
                }
            }

            return RustResponse(
                nativeResponseHandle = nativePointer,
                resultData = resultData,
                ffiError = response.ffi_error,
                flmHandle = flmHandle,
                responseType = responseType
            )
        }
    }
}
