package com.adguard.flm.driver

import com.adguard.flm.support.FilterListManagerConstants
import com.adguard.flm.support.FFIMethod
import com.adguard.flm.support.RustResponseType
import com.adguard.flm.protobuf.Configuration
import pbandk.encodeToByteArray

/**
 * Android implementation of FilterListManagerDriver using JNI.
 *
 * This class provides the Android-specific implementation of the FilterListManagerDriver
 * expect class, using JNI (Java Native Interface) to communicate with the native
 * Filter List Manager library written in Rust.
 *
 * The driver manages the lifecycle of a native handle that represents the Filter List Manager
 * instance in native memory. This handle must be properly released using [close] to prevent
 * memory leaks.
 *
 * @constructor Creates a new driver instance and initializes the native Filter List Manager
 * @param configuration The configuration to initialize the native library with
 * @throws RuntimeException if the native library fails to initialize or returns an error
 */
internal actual class FilterListManagerDriver
@Throws(RuntimeException::class) actual constructor(configuration: Configuration)
    : AutoCloseable
{

    actual companion object {

        // Used to load the native library on application startup.
        init {
            System.loadLibrary("filter_list_manager_jni")
        }

        /**
         * Gets the default configuration from the native library.
         *
         * This method calls into the native library to retrieve the default configuration
         * that can be used to initialize a FilterListManager instance.
         *
         * @return RustResponse containing the default configuration, or null if the call fails
         */
        actual external fun getDefaultConfiguration(): RustResponse?

        /**
         * Gets the library constants from the native library.
         *
         * Retrieves important constants used by the Filter List Manager library,
         * such as special filter and group IDs.
         *
         * @return FilterListManagerConstants containing the library constants
         */
        actual external fun getConstants(): FilterListManagerConstants
    }



    private val handle: Long



    init {
        val configBytes = configuration.encodeToByteArray()
        val response = nativeInitProtobuf(configBytes)

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
        return nativeCall(handle, method, data)
    }

    actual override fun close() {
        nativeFreeHandle(handle)
    }




    /**
     * Initializes a new Filter List Manager instance in native memory.
     *
     * This JNI method creates a new Filter List Manager instance using the provided
     * configuration and returns a handle to that instance. The configuration must be
     * serialized as protobuf bytes before calling this method.
     *
     * @param bytes The configuration serialized as protobuf byte array
     * @return RustResponse containing the native handle if successful, or error information if failed
     */
    external fun nativeInitProtobuf(bytes: ByteArray): RustResponse

    /**
     * Releases a Filter List Manager handle and frees associated native memory.
     *
     * This JNI method must be called when a Filter List Manager instance is no longer
     * needed to prevent memory leaks. After calling this method, the handle becomes
     * invalid and should not be used in subsequent calls.
     *
     * @param handle The FLM handle value obtained from a FLMHandlePointer response
     */
    external fun nativeFreeHandle(handle: Long)

    /**
     * Executes a Filter List Manager method on the native instance.
     *
     * This JNI method calls the specified FFI method on the Filter List Manager instance
     * identified by the handle. The input data must be serialized as protobuf bytes.
     * This is the primary method used for all Filter List Manager operations.
     *
     * @param handle The FLM handle value obtained from initialization
     * @param method The FFIMethod enum value specifying which operation to perform
     * @param inputBuffer The input data serialized as protobuf byte array
     * @return RustResponse containing the operation result or error information
     */
    external fun nativeCall(handle: Long, method: FFIMethod, inputBuffer: ByteArray): RustResponse
}
