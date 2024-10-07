
#include <stdio.h>
#include <stdbool.h>

// FLM opaque pointer
typedef void* FLMHandle;
// FLM Configuration transparent pointer
typedef void* FLMConfiguration;

// FFI method variants
// This enum must have the same order as its rust-file counterpart
typedef enum FFIMethod {
    InstallCustomFilterList,
    EnableFilterLists,
    InstallFilterLists,
    DeleteCustomFilterLists,
    GetFullFilterLists,
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
} FFIMethod;

// Response data possible discriminants
// This enum must have the same order as its rust-file counterpart
typedef enum RustResponseType {
    // Byte buffer
    RustBuffer,
    // FLM Handle
    FLMHandlePointer
} RustResponseType;

// Rust response
typedef struct RustResponse {
    // Protobuf length
    size_t result_data_len;
    // Result protobuf
    void* result_data;
    // Special case if result couldn't be processed: (abnormal ffi behaviour)
    bool ffi_error;
    // Response data discriminant
    RustResponseType discriminant;
} RustResponse;

// Spawn configuration
RustResponse* flm_default_configuration_protobuf();
// Init FLM object with configuration protobuf
RustResponse* flm_init_protobuf(FLMConfiguration, size_t);
// Call FLM method with FLM object handle, method selector and args protobuf buffer
RustResponse* flm_call_protobuf(FLMHandle handle, FFIMethod method, void* input_data_buffer, size_t input_buf_len);

// Free Rust response with the data_len
void flm_free_response(RustResponse* response, size_t data_len, RustResponseType discriminant);

// Free FLMHandle
void flm_free_handle(FLMHandle handle);