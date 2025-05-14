//! A dispatcher module that passes the FFI function call to the right method internally on the Rust side

use crate::native_interface::{build_rust_response_error, FLMHandle, RustResponse};
use crate::outer_error::AGOuterError;
use crate::protobuf_generated::filter_list_manager;
use crate::protobuf_generated::filter_list_manager::{
    ChangeLocaleRequest, ChangeLocaleResponse, DeleteCustomFilterListsRequest,
    DeleteCustomFilterListsResponse, EmptyResponse, EnableFilterListsRequest,
    EnableFilterListsResponse, FetchFilterListMetadataRequest, FetchFilterListMetadataResponse,
    FetchFilterListMetadataWithBodyRequest, FetchFilterListMetadataWithBodyResponse,
    ForceUpdateFiltersByIdsRequest, ForceUpdateFiltersByIdsResponse, GetActiveRulesResponse,
    GetAllGroupsResponse, GetAllTagsResponse, GetDatabasePathResponse, GetDatabaseVersionResponse,
    GetDisabledRulesRequest, GetDisabledRulesResponse, GetFilterRulesAsStringsRequest,
    GetFilterRulesAsStringsResponse, GetFullFilterListByIdRequest, GetRulesCountRequest,
    GetRulesCountResponse, GetStoredFilterMetadataByIdRequest, GetStoredFilterMetadataByIdResponse,
    GetStoredFiltersMetadataResponse, InstallCustomFilterFromStringRequest,
    InstallCustomFilterFromStringResponse, InstallCustomFilterListRequest,
    InstallCustomFilterListResponse, InstallFilterListsRequest, InstallFilterListsResponse,
    SaveCustomFilterRulesRequest, SaveDisabledRulesRequest, SaveRulesToFileBlobRequest,
    SetProxyModeRequest, UpdateCustomFilterMetadataRequest, UpdateCustomFilterMetadataResponse,
    UpdateFiltersRequest, UpdateFiltersResponse,
};
use adguard_flm::RequestProxyMode;
use enum_stringify::EnumStringify;
use prost::Message;
use std::ffi::c_void;

/// Representation of method handle for [`flm_call_protobuf`]
#[repr(C)]
#[derive(EnumStringify)]
pub enum FFIMethod {
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
    FetchFilterListMetadataWithBody,
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
    SetProxyMode,
    GetRulesCount,
}

/// Calls FLM method described as [`FFIMethod`] for object behind [`FLMHandle`]
///
/// # Safety
///
/// 1. `handle.is_null()` is safe and returns error result
#[no_mangle]
pub unsafe extern "C" fn flm_call_protobuf(
    handle: *mut FLMHandle,
    method: FFIMethod,
    input_buffer: *mut u8,
    input_buf_len: usize,
) -> *mut RustResponse {
    let mut rust_response = Box::<RustResponse>::default();

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
                    &format!(
                        "Cannot decode output data for method '{}'",
                        method.to_string()
                    ),
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
        FFIMethod::InstallCustomFilterList => {
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
        FFIMethod::EnableFilterLists => {
            let request = decode_input_request!(EnableFilterListsRequest);

            do_simple_match!(
                flm_handle
                    .flm
                    .enable_filter_lists(request.ids, request.is_enabled),
                EnableFilterListsResponse,
                count
            )
        }
        FFIMethod::InstallFilterLists => {
            let request = decode_input_request!(InstallFilterListsRequest);

            do_simple_match!(
                flm_handle
                    .flm
                    .install_filter_lists(request.ids, request.is_installed),
                InstallFilterListsResponse,
                count
            )
        }
        FFIMethod::DeleteCustomFilterLists => {
            let request = decode_input_request!(DeleteCustomFilterListsRequest);

            do_simple_match!(
                flm_handle.flm.delete_custom_filter_lists(request.ids),
                DeleteCustomFilterListsResponse,
                count
            )
        }
        FFIMethod::GetStoredFiltersMetadata => {
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
        FFIMethod::GetStoredFilterMetadataById => {
            let request = decode_input_request!(GetStoredFilterMetadataByIdRequest);

            match flm_handle.flm.get_stored_filter_metadata_by_id(request.id) {
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
        FFIMethod::GetFullFilterListById => {
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
        FFIMethod::SaveCustomFilterRules => {
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
        FFIMethod::SaveDisabledRules => {
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
        FFIMethod::UpdateFilters => {
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
        FFIMethod::ForceUpdateFiltersByIds => {
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
        FFIMethod::FetchFilterListMetadata => {
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
        FFIMethod::FetchFilterListMetadataWithBody => {
            let request = decode_input_request!(FetchFilterListMetadataWithBodyRequest);

            match flm_handle
                .flm
                .fetch_filter_list_metadata_with_body(request.url)
            {
                Ok(value) => FetchFilterListMetadataWithBodyResponse {
                    metadata: Some(value.into()),
                    error: None,
                },
                Err(why) => FetchFilterListMetadataWithBodyResponse {
                    metadata: None,
                    error: Some(why.into()),
                },
            }
            .encode(&mut out_bytes_buffer)
        }
        FFIMethod::LiftUpDatabase => EmptyResponse {
            error: flm_handle.flm.lift_up_database().err().map(Into::into),
        }
        .encode(&mut out_bytes_buffer),
        FFIMethod::GetAllTags => match flm_handle.flm.get_all_tags() {
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
        FFIMethod::GetAllGroups => match flm_handle.flm.get_all_groups() {
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
        FFIMethod::ChangeLocale => {
            let request = decode_input_request!(ChangeLocaleRequest);

            do_simple_match!(
                flm_handle.flm.change_locale(request.suggested_locale),
                ChangeLocaleResponse,
                success
            )
        }
        FFIMethod::PullMetadata => EmptyResponse {
            error: flm_handle.flm.pull_metadata().err().map(Into::into),
        }
        .encode(&mut out_bytes_buffer),
        FFIMethod::UpdateCustomFilterMetadata => {
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
        FFIMethod::GetDatabasePath => {
            do_simple_match!(
                flm_handle.flm.get_database_path(),
                GetDatabasePathResponse,
                path
            )
        }
        FFIMethod::GetDatabaseVersion => {
            do_simple_match!(
                flm_handle.flm.get_database_version(),
                GetDatabaseVersionResponse,
                version
            )
        }
        FFIMethod::InstallCustomFilterFromString => {
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
        FFIMethod::GetActiveRules => match flm_handle.flm.get_active_rules() {
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
        FFIMethod::GetFilterRulesAsStrings => {
            let request = decode_input_request!(GetFilterRulesAsStringsRequest);

            match flm_handle.flm.get_filter_rules_as_strings(request.ids) {
                Ok(value) => GetFilterRulesAsStringsResponse {
                    rules_list: value.into_iter().map(Into::into).collect(),
                    error: None,
                },
                Err(why) => GetFilterRulesAsStringsResponse {
                    rules_list: vec![],
                    error: Some(why.into()),
                },
            }
        }
        .encode(&mut out_bytes_buffer),
        FFIMethod::SaveRulesToFileBlob => {
            let request = decode_input_request!(SaveRulesToFileBlobRequest);

            EmptyResponse {
                error: flm_handle
                    .flm
                    .save_rules_to_file_blob(request.filter_id, request.file_path)
                    .err()
                    .map(Into::into),
            }
        }
        .encode(&mut out_bytes_buffer),
        FFIMethod::GetDisabledRules => {
            let request = decode_input_request!(GetDisabledRulesRequest);

            match flm_handle.flm.get_disabled_rules(request.ids) {
                Ok(value) => GetDisabledRulesResponse {
                    rules_raw: value.into_iter().map(Into::into).collect(),
                    error: None,
                },
                Err(why) => GetDisabledRulesResponse {
                    rules_raw: vec![],
                    error: Some(why.into()),
                },
            }
        }
        .encode(&mut out_bytes_buffer),
        FFIMethod::SetProxyMode => {
            let request = decode_input_request!(SetProxyModeRequest);

            let inner = match request.mode {
                1 => RequestProxyMode::NoProxy,
                2 => RequestProxyMode::UseCustomProxy {
                    addr: request.custom_proxy_addr,
                },
                _ => RequestProxyMode::UseSystemProxy,
            };

            EmptyResponse {
                error: flm_handle.flm.set_proxy_mode(inner).err().map(Into::into),
            }
        }
        .encode(&mut out_bytes_buffer),
        FFIMethod::GetRulesCount => {
            let request = decode_input_request!(GetRulesCountRequest);

            match flm_handle.flm.get_rules_count(request.ids) {
                Ok(value) => GetRulesCountResponse {
                    rules_count_by_filter: value.into_iter().map(Into::into).collect(),
                    error: None,
                },
                Err(why) => GetRulesCountResponse {
                    rules_count_by_filter: vec![],
                    error: Some(why.into()),
                },
            }
        }
        .encode(&mut out_bytes_buffer),
    };

    if let Err(encode_error) = encode_result {
        return build_rust_response_error(
            Box::new(encode_error),
            rust_response,
            &format!("Cannot encode output data for method '{}'", method),
        );
    }

    rust_response.result_data_capacity = out_bytes_buffer.capacity();
    rust_response.result_data_len = out_bytes_buffer.len();
    rust_response.result_data = Box::into_raw(out_bytes_buffer.into_boxed_slice()) as *mut c_void;

    Box::leak(rust_response)
}
