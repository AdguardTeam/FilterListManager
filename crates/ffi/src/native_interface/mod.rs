//! This module for C functions and structs

pub mod flm_call_protobuf;

use crate::outer_error::AGOuterError;
use crate::protobuf_generated::filter_list_manager;
use crate::protobuf_generated::filter_list_manager::ag_outer_error::Error as ProtobufErrorEnum;
use crate::result::AGResult;
use crate::{FilterListManager, FilterListManagerConstants};
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
#[repr(u8)]
pub enum RustResponseType {
    /// Contains u8 pointer with size
    RustBuffer,
    /// Contains [`FLMHandle`]
    FLMHandlePointer,
}

/// Opaque handle for external world
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
pub extern "C" fn flm_default_configuration_protobuf() -> *mut RustResponse {
    let conf: filter_list_manager::Configuration = Configuration::default().into();

    let mut rust_response = Box::<RustResponse>::default();

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
///
/// # Safety
///
/// 1. This function awaits protobuf pointer `bytes` and its size `size`
/// 2. `handle.result_data_len <= handle.result_data_capacity` is unsafe and will panic
/// 3. `handle.result_data.is_null()` || `size == 0` is safe, returns [`RustResponse`] with error
#[no_mangle]
pub unsafe extern "C" fn flm_init_protobuf(bytes: *const u8, size: usize) -> *mut RustResponse {
    let mut rust_response = Box::<RustResponse>::default();

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
        let mut data = vec![];
        let error = filter_list_manager::AgOuterError::from(factory_result.err().unwrap());
        error
            .encode(&mut data)
            .unwrap_or_else(|_| panic!("[Cannot encode error message]: {}", error.message));

        rust_response.result_data_capacity = data.capacity();
        rust_response.result_data_len = data.len();
        rust_response.result_data = Box::into_raw(data.into_boxed_slice()) as *mut c_void;

        return Box::into_raw(rust_response);
    };

    rust_response.result_data = Box::into_raw(Box::new(flm_handle)) as *mut c_void;
    rust_response.result_data_capacity = size_of::<usize>(); // hmm...
    rust_response.result_data_len = size_of::<usize>(); // hmm...
    rust_response.response_type = RustResponseType::FLMHandlePointer;

    Box::into_raw(rust_response)
}

/// Frees memory of [`RustResponse`] objects and their data.
/// NOTE: Actions for each discriminant are different.
///
/// # Safety
///
/// 1. `handle.result_data.is_null()` is safe
/// 2. `handle.result_data_len <= handle.result_data_capacity` is unsafe and will panic
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
///
/// # Safety
///
/// This function is safe as long as you pass designated pointer
#[no_mangle]
pub unsafe extern "C" fn flm_free_handle(handle: *mut FLMHandle) {
    if !handle.is_null() {
        let _ = Box::from_raw(handle);
    }
}

/// This represents short-circuit error for FFI processing. Returns as [`RustResponse`] with `.ffi_error = true`
#[cold]
fn build_rust_response_error(
    error: Box<dyn std::error::Error>,
    mut rust_response: Box<RustResponse>,
    error_span: &str,
) -> *mut RustResponse {
    let mut vec = vec![];
    filter_list_manager::AgOuterError {
        message: format!("{}: {}", error_span, error),
        error: Some(ProtobufErrorEnum::Other(filter_list_manager::Other {})),
    }
    .encode(&mut vec)
    // All that's left is to fall with the right error
    .unwrap_or_else(|_| panic!("[Cannot encode error message] {}: {}", error_span, error));

    rust_response.result_data_capacity = vec.capacity();
    rust_response.result_data_len = vec.len();
    rust_response.ffi_error = true;
    rust_response.result_data = Box::into_raw(vec.into_boxed_slice()) as *mut c_void;
    rust_response.response_type = RustResponseType::RustBuffer;

    Box::leak(rust_response)
}

/// Getter for the set of [`FilterListManager`] constants
#[no_mangle]
pub extern "C" fn flm_get_constants() -> FilterListManagerConstants {
    FilterListManagerConstants::default()
}
