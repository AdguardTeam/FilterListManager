#include <jni.h>
#include "../../../../../flm_native_interface.h"

static jobject rust_response_to_jobject(
    JNIEnv *env,
    RustResponse *response
) {
    // Find the FilterListManagerDriver class
    jclass driverClass = env->FindClass("com/adguard/flm/driver/FilterListManagerDriver");
    if (driverClass == nullptr) {
        return nullptr; // Error is set by FindClass
    }

    // Find the RustResponse class
    jclass responseClass = env->FindClass("com/adguard/flm/driver/RustResponse");
    if (responseClass == nullptr) {
        return nullptr; // Error is set by FindClass
    }

    // Find the RustResponseType enum class
    jclass responseTypeClass = env->FindClass("com/adguard/flm/support/RustResponseType");
    if (responseTypeClass == nullptr) {
        return nullptr; // Error is set by FindClass
    }

    // Get the enum values for RustResponseType
    jfieldID rustBufferField = env->GetStaticFieldID(responseTypeClass, "RustBuffer",
                                                     "Lcom/adguard/flm/support/RustResponseType;");
    if (rustBufferField == nullptr) {
        return nullptr; // Error is set by GetStaticFieldID
    }

    jfieldID flmHandlePointerField = env->GetStaticFieldID(responseTypeClass, "FLMHandlePointer",
                                                           "Lcom/adguard/flm/support/RustResponseType;");
    if (flmHandlePointerField == nullptr) {
        return nullptr; // Error is set by GetStaticFieldID
    }

    // Process response type
    jobject responseTypeValue;
    jobject resultBuffer = nullptr;
    jlong flmHandle = 0;
    switch (response->response_type) {
        case RustResponseType::RustBuffer:
            resultBuffer = env->NewDirectByteBuffer(response->result_data, response->result_data_len);
            responseTypeValue = env->GetStaticObjectField(responseTypeClass, rustBufferField);
            break;
        case RustResponseType::FLMHandlePointer:
            flmHandle = reinterpret_cast<jlong>(response->result_data);
            responseTypeValue = env->GetStaticObjectField(responseTypeClass, flmHandlePointerField);
            break;
        default:
            env->ThrowNew(env->FindClass("java/lang/RuntimeException"),
                          "Invalid response_type in native RustResponse");
            return nullptr;
    }

    // Find constructor for RustResponse
    jmethodID constructor = env->GetMethodID(responseClass, "<init>",
                                             "(JLjava/nio/ByteBuffer;ZJLcom/adguard/flm/support/RustResponseType;)V");
    if (constructor == nullptr) {
        return nullptr; // Error is set by GetMethodID
    }

    // Create RustResponse object
    // Parameters: rustResponseHandle, result, ffiError, flmHandle, responseType
    jlong rustResponseHandle = reinterpret_cast<jlong>(response);
    return env->NewObject(responseClass, constructor,
                          rustResponseHandle,
                          resultBuffer,
                          response->ffi_error ? JNI_TRUE : JNI_FALSE,
                          flmHandle,
                          responseTypeValue);
}

extern "C"
JNIEXPORT jobject JNICALL
Java_com_adguard_flm_driver_FilterListManagerDriver_00024Companion_getDefaultConfiguration(
    JNIEnv *env,
    jobject /*thiz*/
) {
    // Call the C function to get default configuration
    RustResponse *response = flm_default_configuration_protobuf();
    if (response == nullptr) {
        env->ThrowNew(env->FindClass("java/lang/RuntimeException"),
                      "flm_default_configuration_protobuf() cannot return null");
        return nullptr;
    }

    // Convert the response to a Java object
    return rust_response_to_jobject(env, response);
}

extern "C"
JNIEXPORT jobject JNICALL
Java_com_adguard_flm_driver_FilterListManagerDriver_nativeInitProtobuf(
    JNIEnv *env,
    jobject /*thiz*/,
    jbyteArray bytes
) {
    // Get the byte array elements
    jsize length = env->GetArrayLength(bytes);
    jbyte *elements = env->GetByteArrayElements(bytes, nullptr);

    // Call the C function to initialize FLM
    RustResponse *response = flm_init_protobuf(reinterpret_cast<const uint8_t *>(elements), length);

    // Release the byte array elements
    env->ReleaseByteArrayElements(bytes, elements, JNI_ABORT);

    if (response == nullptr) {
        env->ThrowNew(env->FindClass("java/lang/RuntimeException"), "flm_init_protobuf() cannot return null");
        return nullptr;
    }

    // Convert the response to a Java object
    return rust_response_to_jobject(env, response);
}

extern "C"
JNIEXPORT void JNICALL
Java_com_adguard_flm_driver_FilterListManagerDriver_nativeFreeHandle(
    JNIEnv *env,
    jobject /*thiz*/,
    jlong handle
) {
    // Call the C function to free the handle
    flm_free_handle(reinterpret_cast<FLMHandle *>(handle));
}

extern "C"
JNIEXPORT jobject JNICALL
Java_com_adguard_flm_driver_FilterListManagerDriver_00024Companion_getConstants(
    JNIEnv *env,
    jobject /*thiz*/
) {
    // Call the C function to get constants
    FilterListManagerConstants constants = flm_get_constants();

    // Find the FilterListManagerConstants class
    jclass constantsClass = env->FindClass(
            "com/adguard/flm/support/FilterListManagerConstants");
    if (constantsClass == nullptr) {
        return nullptr; // Error is set by FindClass
    }

    // Find constructor for FilterListManagerConstants
    jmethodID constructor = env->GetMethodID(constantsClass, "<init>", "(IIII)V");
    if (constructor == nullptr) {
        return nullptr; // Error is set by GetMethodID
    }

    // Create FilterListManagerConstants object
    return env->NewObject(constantsClass, constructor,
                          constants.user_rules_id,
                          constants.custom_group_id,
                          constants.special_group_id,
                          constants.smallest_filter_id);
}

extern "C"
JNIEXPORT jobject JNICALL
Java_com_adguard_flm_driver_FilterListManagerDriver_nativeCall(
    JNIEnv *env,
    jobject /*thiz*/,
    jlong handle,
    jobject method,
    jbyteArray inputBuffer
) {
    // Get the FFIMethod enum value
    jclass methodClass = env->GetObjectClass(method);
    jmethodID ordinalMethod = env->GetMethodID(methodClass, "ordinal", "()I");
    jint methodOrdinal = env->CallIntMethod(method, ordinalMethod);

    // Convert Java enum to C enum
    FFIMethod cMethod = static_cast<FFIMethod>(methodOrdinal);

    // Get the byte array elements
    jsize length = env->GetArrayLength(inputBuffer);
    jbyte *elements = env->GetByteArrayElements(inputBuffer, nullptr);

    // Call the C function
    RustResponse *response = flm_call_protobuf(
            reinterpret_cast<FLMHandle *>(handle),
            cMethod,
            reinterpret_cast<uint8_t *>(elements),
            length);

    // Release the byte array elements
    env->ReleaseByteArrayElements(inputBuffer, elements, JNI_ABORT);

    if (response == nullptr) {
        env->ThrowNew(env->FindClass("java/lang/RuntimeException"), "flm_call_protobuf() cannot return null");
        return nullptr;
    }

    // Convert the response to a Java object
    return rust_response_to_jobject(env, response);
}



extern "C"
JNIEXPORT void JNICALL
Java_com_adguard_flm_driver_RustResponse_nativeFreeResponse(
    JNIEnv *env,
    jobject /*thiz*/,
    jlong handle
) {
    flm_free_response(reinterpret_cast<RustResponse *>(handle));
}
