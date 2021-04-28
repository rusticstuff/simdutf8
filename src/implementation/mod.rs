//! Contains UTF-8 validation implementations.

type Utf8ErrorCompat = crate::compat::Utf8Error;
type Utf8ErrorBasic = crate::basic::Utf8Error;

#[allow(unused_macros)]
#[macro_use]
mod macros;

#[allow(unused_macros)]
#[macro_use]
mod algorithm_macros;

pub(crate) mod algorithm;

// UTF-8 validation function types

#[allow(dead_code)]
type ValidateUtf8Fn = unsafe fn(input: &[u8]) -> Result<(), Utf8ErrorBasic>;

#[allow(dead_code)]
type ValidateUtf8CompatFn = unsafe fn(input: &[u8]) -> Result<(), Utf8ErrorCompat>;

// arch-specific functions

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) mod x86;

/// Fn needed instead of re-import, otherwise not inlined in non-std case
#[allow(clippy::inline_always)]
#[inline(always)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(super) unsafe fn validate_utf8_basic(input: &[u8]) -> Result<(), Utf8ErrorBasic> {
    x86::validate_utf8_basic(input)
}

/// Fn needed instead of re-import, otherwise not inlined in non-std case
#[allow(clippy::inline_always)]
#[inline(always)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(super) unsafe fn validate_utf8_compat(input: &[u8]) -> Result<(), Utf8ErrorCompat> {
    x86::validate_utf8_compat(input)
}

// fallback for non-x86

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub(super) use validate_utf8_basic_fallback as validate_utf8_basic;

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub(super) use validate_utf8_compat_fallback as validate_utf8_compat;

// fallback method implementations

#[inline]
#[allow(dead_code)]
pub(crate) fn validate_utf8_basic_fallback(input: &[u8]) -> Result<(), Utf8ErrorBasic> {
    match core::str::from_utf8(input) {
        Ok(_) => Ok(()),
        Err(_) => Err(Utf8ErrorBasic {}),
    }
}

#[inline]
#[allow(dead_code)]
pub(crate) fn validate_utf8_compat_fallback(input: &[u8]) -> Result<(), Utf8ErrorCompat> {
    algorithm::validate_utf8_at_offset(input, 0)
}
