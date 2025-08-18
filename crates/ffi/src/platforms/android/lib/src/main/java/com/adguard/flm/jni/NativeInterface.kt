package com.adguard.flm.jni

import com.adguard.flm.FilterListManagerConstants

/**
 * Native interface for Filter List Manager
 * JNI bindings for functions defined in flm_native_interface.h
 */
internal class NativeInterface {

    companion object {
        /**
         * Makes default Configuration object as protobuf in RustResponse
         *
         * @return RustResponse containing the default configuration
         */
        @JvmStatic
        external fun flmDefaultConfigurationProtobuf(): RustResponse

        /**
         * Makes an FLM object and returns a RustResponse
         *
         * @param bytes Protobuf bytes
         * @return RustResponse containing the FLM handle
         */
        @JvmStatic
        external fun flmInitProtobuf(bytes: ByteArray): RustResponse

        /**
         * Drops FLMHandle
         * This should be called for FLMHandlePointer responses
         * when the handle is no longer needed.
         *
         * @param handle FLM handle value from FLMHandlePointer
         */
        @JvmStatic
        external fun flmFreeHandle(handle: Long)

        /**
         * Frees resources associated with a RustResponse
         * This should be called when a RustResponse is no longer needed.
         *
         * @param handle RustResponse handle value to be freed
         */
        @JvmStatic
        external fun flmFreeResponse(handle: Long)

        /**
         * Getter for the set of FilterListManager constants
         *
         * @return FilterListManagerConstants
         */
        @JvmStatic
        external fun flmGetConstants(): FilterListManagerConstants

        /**
         * Calls FLM method described as FFIMethod for object behind FLMHandle
         *
         * @param handle FLM handle value from FLMHandlePointer
         * @param method FFIMethod enum value
         * @param inputBuffer Input protobuf buffer
         * @return RustResponse containing the result
         */
        @JvmStatic
        external fun flmCallProtobuf(handle: Long, method: FFIMethod, inputBuffer: ByteArray): RustResponse

        // Used to load the native library on application startup.
        init {
            System.loadLibrary("filter_list_manager_ffi")
            System.loadLibrary("filter_list_manager_jni")
        }
    }
}
