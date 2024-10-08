//! This module for C functions and structs

use crate::outer_error::AGOuterError;
use crate::protobuf_generated::filter_list_manager;
use crate::protobuf_generated::filter_list_manager::ag_outer_error::Error as ProtobufErrorEnum;
use crate::protobuf_generated::filter_list_manager::{
    ChangeLocaleRequest, ChangeLocaleResponse, DeleteCustomFilterListsRequest,
    DeleteCustomFilterListsResponse, EmptyResponse, EnableFilterListsRequest,
    EnableFilterListsResponse, FetchFilterListMetadataRequest, FetchFilterListMetadataResponse,
    ForceUpdateFiltersByIdsRequest, ForceUpdateFiltersByIdsResponse, GetActiveRulesResponse,
    GetAllGroupsResponse, GetAllTagsResponse, GetDatabasePathResponse, GetDatabaseVersionResponse,
    GetFullFilterListByIdRequest, GetStoredFilterMetadataByIdResponse,
    GetStoredFiltersMetadataByIdRequest, GetStoredFiltersMetadataResponse,
    InstallCustomFilterFromStringRequest, InstallCustomFilterFromStringResponse,
    InstallCustomFilterListRequest, InstallCustomFilterListResponse, InstallFilterListsRequest,
    InstallFilterListsResponse, SaveCustomFilterRulesRequest, SaveDisabledRulesRequest,
    UpdateCustomFilterMetadataRequest, UpdateCustomFilterMetadataResponse, UpdateFiltersRequest,
    UpdateFiltersResponse,
};
use crate::result::AGResult;
pub use crate::top_level::*;
use crate::FilterListManager;
use adguard_flm::Configuration;
use prost::Message;
use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::null_mut;

/// Container for rust-formed responses into external world
/// UNSAFE: You must manually control the release of any types folded into the “response”
#[repr(C)]
pub struct RustResponse {
    /// Bytes count
    /// UNSAFE: You should put here the real data length, even for pointers
    pub result_data_len: usize,
    /// The real allocated data length
    /// UNSAFE: You should put here the real data length, even for pointers
    pub result_data_capacity: usize,
    /// UNSAFE: There can be many different pointer types
    pub result_data: *mut c_void,
    /// Special response case: If request or response have failed, try to send [`AGOuterError::Other`] error with the explanation
    /// See: [`build_rust_response_error`]
    pub ffi_error: bool,
    /// Data type discriminant
    pub response_type: RustResponseType,
}

impl Default for RustResponse {
    fn default() -> Self {
        Self {
            response_type: RustResponseType::RustBuffer,
            result_data: null_mut(),
            ffi_error: false,
            result_data_len: 0,
            result_data_capacity: 0,
        }
    }
}

/// Discriminant for [`RustResponse`] result_data value
#[repr(C)]
pub enum RustResponseType {
    /// Contains u8 pointer with size
    RustBuffer,
    /// Contains [`FLMHandle`]
    FLMHandlePointer,
}

/// Opaque handle for external world
#[repr(C)]
pub struct FLMHandle {
    pub(crate) flm: FilterListManager,
}

impl FLMHandle {
    /// Opaque handle factory
    pub(crate) fn new(configuration: Configuration) -> AGResult<Self> {
        Ok(Self {
            flm: FilterListManager::new(configuration)?,
        })
    }
}

/// Makes default [`Configuration`] object as protobuf in [`RustResponse`]
#[no_mangle]
pub unsafe extern "C" fn flm_default_configuration_protobuf() -> *mut RustResponse {
    let conf: filter_list_manager::Configuration = Configuration::default().into();

    let mut rust_response = Box::new(RustResponse::default());

    let mut vec = vec![];
    if let Err(why) = conf.encode(&mut vec) {
        return build_rust_response_error(
            Box::new(why),
            rust_response,
            "Cannot spawn configuration",
        );
    }

    let len = vec.len();
    let capacity = vec.capacity();

    let bytes_rust = Box::into_raw(vec.into_boxed_slice());

    rust_response.result_data_capacity = capacity;
    rust_response.result_data_len = len;
    rust_response.result_data = bytes_rust as *mut c_void;

    Box::into_raw(rust_response)
}

/// Makes an FLM object and returns opaque pointer of [`FLMHandle`]
#[no_mangle]
pub unsafe extern "C" fn flm_init_protobuf(bytes: *const u8, size: usize) -> *mut RustResponse {
    let mut rust_response = Box::new(RustResponse::default());

    if bytes.is_null() || size == 0 {
        return build_rust_response_error(
            Box::new(AGOuterError::Other(String::from(
                "Got empty configuration object, while init flm",
            ))),
            rust_response,
            "",
        );
    }

    let config_data = std::slice::from_raw_parts(bytes, size);
    let decode_result = filter_list_manager::Configuration::decode(config_data);
    let Ok(conf) = decode_result else {
        return build_rust_response_error(
            Box::new(decode_result.unwrap_err()),
            rust_response,
            "Cannot decode Configuration",
        );
    };

    let factory_result = FLMHandle::new(conf.into());
    let Ok(flm_handle) = factory_result else {
        return build_rust_response_error(
            Box::new(factory_result.err().unwrap()),
            rust_response,
            "Cannot encode new FLM Instance",
        );
    };

    rust_response.result_data = Box::into_raw(Box::new(flm_handle)) as *mut c_void;
    rust_response.result_data_capacity = size_of::<usize>(); // hmm...
    rust_response.result_data_len = size_of::<usize>(); // hmm...
    rust_response.response_type = RustResponseType::FLMHandlePointer;

    Box::into_raw(rust_response)
}

/// Calls FLM method described as [`FFIMethods`] for object behind [`FLMHandle`]
#[no_mangle]
pub unsafe extern "C" fn flm_call_protobuf(
    handle: *mut FLMHandle,
    method: FFIMethods,
    input_buffer: *mut u8,
    input_buf_len: usize,
) -> *mut RustResponse {
    let mut rust_response = Box::new(RustResponse::default());

    if handle.is_null() {
        return build_rust_response_error(
            Box::new(AGOuterError::Other(String::from(
                "Got empty configuration object, while init flm",
            ))),
            rust_response,
            "",
        );
    }

    // Get handle
    let flm_handle = &mut *handle;

    // Get args
    let input_bytes = std::slice::from_raw_parts(input_buffer, input_buf_len);

    macro_rules! decode_input_request {
        ($type:ty) => {{
            let decode_result = <$type>::decode(input_bytes);

            let Ok(request) = decode_result else {
                return build_rust_response_error(
                    Box::new(decode_result.err().unwrap()),
                    rust_response,
                    &format!("Cannot decode output data for method '{}'", "todo: method"), // TODO:
                );
            };

            request as $type
        }};
    }

    let mut out_bytes_buffer = vec![];

    macro_rules! do_simple_match {
        ($stmt:expr, $resp_type:ident, $field:ident) => {
            match $stmt {
                Ok(value) => $resp_type {
                    $field: value,
                    error: None,
                },
                Err(why) => $resp_type {
                    $field: Default::default(),
                    error: Some(why.into()),
                },
            }
            .encode(&mut out_bytes_buffer)
        };
    }

    let encode_result = match method {
        FFIMethods::InstallCustomFilterList => {
            let request = decode_input_request!(InstallCustomFilterListRequest);

            match flm_handle.flm.install_custom_filter_list(
                request.download_url,
                request.is_trusted,
                request.title,
                request.description,
            ) {
                Ok(value) => InstallCustomFilterListResponse {
                    filter_list: Some(value.into()),
                    error: None,
                },
                Err(why) => InstallCustomFilterListResponse {
                    filter_list: None,
                    error: Some(why.into()),
                },
            }
        }
        .encode(&mut out_bytes_buffer),
        FFIMethods::EnableFilterLists => {
            let request = decode_input_request!(EnableFilterListsRequest);

            do_simple_match!(
                flm_handle
                    .flm
                    .enable_filter_lists(request.ids, request.is_enabled),
                EnableFilterListsResponse,
                count
            )
        }
        FFIMethods::InstallFilterLists => {
            let request = decode_input_request!(InstallFilterListsRequest);

            do_simple_match!(
                flm_handle
                    .flm
                    .install_filter_lists(request.ids, request.is_installed),
                InstallFilterListsResponse,
                count
            )
        }
        FFIMethods::DeleteCustomFilterLists => {
            let request = decode_input_request!(DeleteCustomFilterListsRequest);

            do_simple_match!(
                flm_handle.flm.delete_custom_filter_lists(request.ids),
                DeleteCustomFilterListsResponse,
                count
            )
        }
        FFIMethods::GetStoredFiltersMetadata => {
            match flm_handle.flm.get_stored_filters_metadata() {
                Ok(value) => GetStoredFiltersMetadataResponse {
                    filter_lists: value.into_iter().map(Into::into).collect(),
                    error: None,
                },
                Err(why) => GetStoredFiltersMetadataResponse {
                    filter_lists: vec![],
                    error: Some(why.into()),
                },
            }
        }
        .encode(&mut out_bytes_buffer),
        FFIMethods::GetStoredFilterMetadataById => {
            let request = decode_input_request!(GetStoredFiltersMetadataByIdRequest);

            match flm_handle.flm.get_stored_filters_metadata_by_id(request.id) {
                Ok(value) => GetStoredFilterMetadataByIdResponse {
                    filter_list: value.map(Into::into),
                    error: None,
                },
                Err(why) => GetStoredFilterMetadataByIdResponse {
                    filter_list: None,
                    error: Some(why.into()),
                },
            }
        }
        .encode(&mut out_bytes_buffer),
        FFIMethods::GetFullFilterLists => match flm_handle.flm.get_full_filter_lists() {
            Ok(list) => {
                let converted = list
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<filter_list_manager::FullFilterList>>();

                filter_list_manager::GetFullFilterListsResponse {
                    filter_lists: converted,
                    error: None,
                }
            }
            Err(why) => filter_list_manager::GetFullFilterListsResponse {
                filter_lists: vec![],
                error: Some(why.into()),
            },
        }
        .encode(&mut out_bytes_buffer),
        FFIMethods::GetFullFilterListById => {
            let request = decode_input_request!(GetFullFilterListByIdRequest);

            match flm_handle.flm.get_full_filter_list_by_id(request.id) {
                Ok(value) => filter_list_manager::GetFullFilterListByIdResponse {
                    filter_list: value.map(Into::into),
                    error: None,
                },
                Err(why) => filter_list_manager::GetFullFilterListByIdResponse {
                    filter_list: None,
                    error: Some(why.into()),
                },
            }
            .encode(&mut out_bytes_buffer)
        }
        FFIMethods::SaveCustomFilterRules => {
            let request = decode_input_request!(SaveCustomFilterRulesRequest);

            let Some(rules) = request.rules else {
                let err = AGOuterError::Other(String::from(
                    "Cannot decode SaveCustomFilterRulesRequest, because rules in none",
                ));
                return build_rust_response_error(Box::new(err), rust_response, "");
            };

            EmptyResponse {
                error: flm_handle
                    .flm
                    .save_custom_filter_rules(rules.into())
                    .err()
                    .map(Into::into),
            }
            .encode(&mut out_bytes_buffer)
        }
        FFIMethods::SaveDisabledRules => {
            let request = decode_input_request!(SaveDisabledRulesRequest);

            EmptyResponse {
                error: flm_handle
                    .flm
                    .save_disabled_rules(request.filter_id, request.disabled_rules)
                    .err()
                    .map(Into::into),
            }
            .encode(&mut out_bytes_buffer)
        }
        FFIMethods::UpdateFilters => {
            let request = decode_input_request!(UpdateFiltersRequest);

            match flm_handle.flm.update_filters(
                request.ignore_filters_expiration,
                request.loose_timeout,
                request.ignore_filters_status,
            ) {
                Ok(update_result) => UpdateFiltersResponse {
                    result: update_result.map(Into::into),
                    error: None,
                },
                Err(why) => UpdateFiltersResponse {
                    result: None,
                    error: Some(why.into()),
                },
            }
            .encode(&mut out_bytes_buffer)
        }
        FFIMethods::ForceUpdateFiltersByIds => {
            let request = decode_input_request!(ForceUpdateFiltersByIdsRequest);

            match flm_handle
                .flm
                .force_update_filters_by_ids(request.ids, request.loose_timeout)
            {
                Ok(value) => ForceUpdateFiltersByIdsResponse {
                    result: value.map(Into::into),
                    error: None,
                },
                Err(why) => ForceUpdateFiltersByIdsResponse {
                    result: None,
                    error: Some(why.into()),
                },
            }
            .encode(&mut out_bytes_buffer)
        }
        FFIMethods::FetchFilterListMetadata => {
            let request = decode_input_request!(FetchFilterListMetadataRequest);

            match flm_handle.flm.fetch_filter_list_metadata(request.url) {
                Ok(value) => FetchFilterListMetadataResponse {
                    metadata: Some(value.into()),
                    error: None,
                },
                Err(why) => FetchFilterListMetadataResponse {
                    metadata: None,
                    error: Some(why.into()),
                },
            }
            .encode(&mut out_bytes_buffer)
        }
        FFIMethods::LiftUpDatabase => EmptyResponse {
            error: flm_handle.flm.lift_up_database().err().map(Into::into),
        }
        .encode(&mut out_bytes_buffer),
        FFIMethods::GetAllTags => match flm_handle.flm.get_all_tags() {
            Ok(tags) => GetAllTagsResponse {
                tags: tags.into_iter().map(Into::into).collect(),
                error: None,
            },
            Err(why) => GetAllTagsResponse {
                tags: vec![],
                error: Some(why.into()),
            },
        }
        .encode(&mut out_bytes_buffer),
        FFIMethods::GetAllGroups => match flm_handle.flm.get_all_groups() {
            Ok(groups) => GetAllGroupsResponse {
                groups: groups.into_iter().map(Into::into).collect(),
                error: None,
            },
            Err(why) => GetAllGroupsResponse {
                groups: vec![],
                error: Some(why.into()),
            },
        }
        .encode(&mut out_bytes_buffer),
        FFIMethods::ChangeLocale => {
            let request = decode_input_request!(ChangeLocaleRequest);

            do_simple_match!(
                flm_handle.flm.change_locale(request.suggested_locale),
                ChangeLocaleResponse,
                success
            )
        }
        FFIMethods::PullMetadata => EmptyResponse {
            error: flm_handle.flm.pull_metadata().err().map(Into::into),
        }
        .encode(&mut out_bytes_buffer),
        FFIMethods::UpdateCustomFilterMetadata => {
            let request = decode_input_request!(UpdateCustomFilterMetadataRequest);

            do_simple_match!(
                flm_handle.flm.update_custom_filter_metadata(
                    request.filter_id,
                    request.title,
                    request.is_trusted
                ),
                UpdateCustomFilterMetadataResponse,
                success
            )
        }
        FFIMethods::GetDatabasePath => {
            do_simple_match!(
                flm_handle.flm.get_database_path(),
                GetDatabasePathResponse,
                path
            )
        }
        FFIMethods::GetDatabaseVersion => {
            do_simple_match!(
                flm_handle.flm.get_database_version(),
                GetDatabaseVersionResponse,
                version
            )
        }
        FFIMethods::InstallCustomFilterFromString => {
            let request = decode_input_request!(InstallCustomFilterFromStringRequest);

            match flm_handle.flm.install_custom_filter_from_string(
                request.download_url,
                request.last_download_time,
                request.is_enabled,
                request.is_trusted,
                request.filter_body,
                request.custom_title,
                request.custom_description,
            ) {
                Ok(value) => InstallCustomFilterFromStringResponse {
                    filter_list: Some(value.into()),
                    error: None,
                },
                Err(why) => InstallCustomFilterFromStringResponse {
                    filter_list: None,
                    error: Some(why.into()),
                },
            }
            .encode(&mut out_bytes_buffer)
        }
        FFIMethods::GetActiveRules => match flm_handle.flm.get_active_rules() {
            Ok(value) => GetActiveRulesResponse {
                rules: value.into_iter().map(Into::into).collect(),
                error: None,
            },
            Err(why) => GetActiveRulesResponse {
                rules: vec![],
                error: Some(why.into()),
            },
        }
        .encode(&mut out_bytes_buffer),
    };

    if let Err(encode_error) = encode_result {
        return build_rust_response_error(
            Box::new(encode_error),
            rust_response,
            &format!("Cannot encode output data for method '{}'", "todo: method"), // TODO:
        );
    }

    rust_response.result_data_capacity = out_bytes_buffer.capacity();
    rust_response.result_data_len = out_bytes_buffer.len();
    rust_response.result_data = Box::into_raw(out_bytes_buffer.into_boxed_slice()) as *mut c_void;

    Box::leak(rust_response)
}

/// Frees memory of [`RustResponse`] objects and their data.
/// NOTE: Actions for each discriminant are different.
#[no_mangle]
pub unsafe extern "C" fn flm_free_response(handle: *mut RustResponse) {
    if !handle.is_null() {
        let response = Box::from_raw(handle);

        if !response.result_data.is_null() {
            // One of `Vec::from_raw_parts` invariants
            assert!(
                response.result_data_len <= response.result_data_capacity,
                "Cannot free RustResponse mem, because buffer capacity greater than its length"
            );

            match response.response_type {
                RustResponseType::RustBuffer => {
                    // Placement new into allocated memory. Then autodrop after leaving block

                    let _ = Vec::from_raw_parts(
                        response.result_data as *mut u8,
                        response.result_data_len,
                        response.result_data_capacity,
                    );
                }
                RustResponseType::FLMHandlePointer => {
                    // The handle must live. It will be deleted later in an explicit way
                }
            }
        }
    }
}

/// Drops [`FLMHandle`]
#[no_mangle]
pub unsafe extern "C" fn flm_free_handle(handle: *mut FLMHandle) {
    if !handle.is_null() {
        let _ = Box::from_raw(handle);
    }
}

/// This represents short-circuit error for FFI processing. Returns as [`RustResponse`] with `.ffi_error = true`
#[inline]
fn build_rust_response_error(
    error: Box<dyn std::error::Error>,
    mut rust_response: Box<RustResponse>,
    error_span: &str,
) -> *mut RustResponse {
    let mut vec = vec![];
    filter_list_manager::AgOuterError {
        message: format!("{}: {}", error_span, error.to_string()),
        error: Some(ProtobufErrorEnum::Other(filter_list_manager::Other {})),
    }
    .encode(&mut vec)
    // All that's left is to fall with the right error
    .expect(&format!(
        "[Cannot encode error message] {}: {}",
        error_span,
        error.to_string()
    ));

    rust_response.result_data_capacity = vec.capacity();
    rust_response.result_data_len = vec.len();
    rust_response.ffi_error = true;
    rust_response.result_data = Box::into_raw(vec.into_boxed_slice()) as *mut c_void;
    rust_response.response_type = RustResponseType::RustBuffer;

    return Box::leak(rust_response);
}

/// This enum must have the same order as its header-file counterpart
#[repr(C)]
pub enum FFIMethods {
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
}
