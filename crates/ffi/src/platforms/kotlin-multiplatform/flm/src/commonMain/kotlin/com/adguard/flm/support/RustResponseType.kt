package com.adguard.flm.support

/**
 * Copy for native enum from [RustResponseType](../../../../../../../../../flm_native_interface.h).
 * These 2 enums must be synchronized.
 */
enum class RustResponseType {
    /**
     * Contains a byte buffer with data (e.g., protobuf serialized data)
     */
    RustBuffer,

    /**
     * Contains a handle to a Filter List Manager instance
     */
    FLMHandlePointer
}
