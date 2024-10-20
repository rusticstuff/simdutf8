//! Contains UTF-8 validation implementations.

#[macro_use]
#[allow(unused_macros)] // only used if there is a SIMD implementation
mod algorithm;

pub(crate) mod helpers;

// UTF-8 validation function types
pub(crate) mod portable;

// fallback method implementations
#[inline]
pub(crate) fn validate_utf8_basic_fallback(input: &[u8]) -> Result<(), crate::basic::Utf8Error> {
    match core::str::from_utf8(input) {
        Ok(_) => Ok(()),
        Err(_) => Err(crate::basic::Utf8Error {}),
    }
}

#[inline]
pub(crate) fn validate_utf8_compat_fallback(input: &[u8]) -> Result<(), crate::compat::Utf8Error> {
    helpers::validate_utf8_at_offset(input, 0)
}
