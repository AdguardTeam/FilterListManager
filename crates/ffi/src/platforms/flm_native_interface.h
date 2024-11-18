/* This is an autogenerated file. Do not edit. See build.rs */

#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Representation of method handle for [`flm_call_protobuf`]
 */
typedef enum FFIMethod {
    InstallCustomFilterList,
    EnableFilterLists,
    InstallFilterLists,
    DeleteCustomFilterLists,
    GetFullFilterListById,
    GetStoredFiltersMetadata,
    GetStoredFilterMetadataById,
    SaveCustomFilterRules,
    SaveDisabledRules,
    UpdateFilters,
    ForceUpdateFiltersByIds,
    FetchFilterListMetadata,
    LiftUpDatabase,
    GetAllTags,
    GetAllGroups,
    ChangeLocale,
    PullMetadata,
    UpdateCustomFilterMetadata,
    GetDatabasePath,
    GetDatabaseVersion,
    InstallCustomFilterFromString,
    GetActiveRules,
    GetFilterRulesAsStrings,
    SaveRulesToFileBlob,
    GetDisabledRules,
} FFIMethod;

/**
 * Discriminant for [`RustResponse`] result_data value
 */
enum RustResponseType
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
    /**
     * Contains u8 pointer with size
     */
    RustBuffer,
    /**
     * Contains [`FLMHandle`]
     */
    FLMHandlePointer,
};
#ifndef __cplusplus
typedef uint8_t RustResponseType;
#endif // __cplusplus

/**
 * Opaque handle for external world
 */
typedef struct FLMHandle FLMHandle;

/**
 * Container for rust-formed responses into external world
 * UNSAFE: You must manually control the release of any types folded into the “response”
 */
typedef struct RustResponse {
    /**
     * Bytes count
     * UNSAFE: You should put here the real data length, even for pointers
     */
    size_t result_data_len;
    /**
     * The real allocated data length
     * UNSAFE: You should put here the real data length, even for pointers
     */
    size_t result_data_capacity;
    /**
     * UNSAFE: There can be many different pointer types
     */
    void *result_data;
    /**
     * Special response case: If request or response have failed, try to send [`AGOuterError::Other`] error with the explanation
     * See: [`build_rust_response_error`]
     */
    bool ffi_error;
    /**
     * Data type discriminant
     */
    RustResponseType response_type;
} RustResponse;

/**
 * Structure used for passing constants through FFI
 */
typedef struct FilterListManagerConstants {
    /**
     * Filter ID for *User rules* filter
     */
    int32_t user_rules_id;
    /**
     * Group ID for special *custom filters group*
     */
    int32_t custom_group_id;
    /**
     * Group ID for *special service filters*
     */
    int32_t special_group_id;
    /**
     * Smallest possible filter_id. You can safely occupy any filter with an id lower than this number.
     * The library is guaranteed to never create a filter with this id
     */
    int32_t smallest_filter_id;
} FilterListManagerConstants;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Makes default [`Configuration`] object as protobuf in [`RustResponse`]
 */
struct RustResponse *flm_default_configuration_protobuf(void);

/**
 * Makes an FLM object and returns opaque pointer of [`FLMHandle`]
 */
struct RustResponse *flm_init_protobuf(const uint8_t *bytes, size_t size);

/**
 * Frees memory of [`RustResponse`] objects and their data.
 * NOTE: Actions for each discriminant are different.
 */
void flm_free_response(struct RustResponse *handle);

/**
 * Drops [`FLMHandle`]
 */
void flm_free_handle(struct FLMHandle *handle);

/**
 * Getter for the set of [`FilterListManager`] constants
 */
struct FilterListManagerConstants flm_get_constants(void);

/**
 * Calls FLM method described as [`FFIMethod`] for object behind [`FLMHandle`]
 */
struct RustResponse *flm_call_protobuf(struct FLMHandle *handle,
                                       enum FFIMethod method,
                                       uint8_t *input_buffer,
                                       size_t input_buf_len);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
