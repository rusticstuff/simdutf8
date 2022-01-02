//! A very basic C ABI wrapper over the simdutf8 APIs for the purposes of benchmarking.
//!
//! This is used to get a simple interface that we can embed WASM runtime(s) and run existing
//! benchmarks over the WASM targeted code.

#![no_std]

use core::ptr::slice_from_raw_parts;

mod implementation {
    pub(super) use core::str::from_utf8 as std_from_utf8;
    pub(super) use simdutf8::basic::from_utf8 as basic_from_utf8;
    pub(super) use simdutf8::compat::from_utf8 as compat_from_utf8;
}

// if we don't return something about the error--the optimizer will optimize
// compat into basic...

#[no_mangle]
pub unsafe extern "C" fn std_from_utf8(bytes: *const u8, len: usize) -> usize {
    if let Some(slice) = slice_from_raw_parts(bytes, len).as_ref() {
        if let Err(e) = implementation::std_from_utf8(slice) {
            e.valid_up_to()
        } else {
            slice.len()
        }
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn compat_from_utf8(bytes: *const u8, len: usize) -> usize {
    if let Some(slice) = slice_from_raw_parts(bytes, len).as_ref() {
        if let Err(e) = implementation::compat_from_utf8(slice) {
            e.valid_up_to()
        } else {
            slice.len()
        }
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn basic_from_utf8(bytes: *const u8, len: usize) -> bool {
    if let Some(slice) = slice_from_raw_parts(bytes, len).as_ref() {
        implementation::basic_from_utf8(slice).is_ok()
    } else {
        false
    }
}
