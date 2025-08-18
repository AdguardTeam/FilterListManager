package com.adguard.flm.jni

/**
 * Discriminant for RustResponse resultData/flmHandle values.
 * Indicates the type of data contained in the response from the native code.
 */
internal enum class RustResponseType {
    /**
     * Contains a byte buffer with data (e.g., protobuf serialized data)
     */
    RustBuffer,
    
    /**
     * Contains a handle to a Filter List Manager instance
     */
    FLMHandlePointer
}
