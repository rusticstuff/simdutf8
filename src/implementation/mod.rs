//! Contains UTF-8 validation implementations.

use crate::Utf8Error;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64")))]
#[macro_use]
mod macros;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64")))]
#[macro_use]
mod x86;

/// UTF-8 validation function type
pub type ValidateUtf8Fn = unsafe fn(input: &[u8]) -> Result<(), Utf8Error>;

#[cfg_attr(not(feature = "no-inline"), inline)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) unsafe fn validate_utf8(input: &[u8]) -> Result<(), Utf8Error> {
    x86::validate_utf8(input)
}

#[cfg(all(not(any(target_arch = "x86", target_arch = "x86_64")), feature = "std"))]
pub(crate) fn get_fastest_available_implementation() -> ValidateUtf8Fn {
    validate_utf8_fallback
}

#[cfg_attr(not(feature = "no-inline"), inline)]
#[allow(dead_code)]
pub(crate) fn validate_utf8_fallback(input: &[u8]) -> Result<(), Utf8Error> {
    match core::str::from_utf8(input) {
        Ok(_) => Ok(()),
        Err(_) => Err(Utf8Error {}),
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
struct Utf8CheckingState<T> {
    pub prev: T,
    pub incomplete: T,
    pub error: T,
}
