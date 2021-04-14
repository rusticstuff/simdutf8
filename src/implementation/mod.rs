use crate::Utf8Error;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64")))]
#[macro_use]
mod macros;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(dead_code)]
pub(crate) mod avx2;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(dead_code)]
pub(crate) mod sse42;

pub(crate) type ValidateUtf8Implementation = fn(input: &[u8]) -> Result<(), Utf8Error>;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64")))]
pub(crate) fn get_fastest_available_implementation() -> ValidateUtf8Implementation {
    avx2::get_implementation()
        .or_else(sse42::get_implementation)
        .unwrap_or(validate_utf8_fallback)
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub(crate) fn get_fastest_available_implementation() -> ValidateUtf8Implementation {
    validate_utf8_fallback
}

pub fn validate_utf8_fallback(input: &[u8]) -> Result<(), Utf8Error> {
    match core::str::from_utf8(input) {
        Ok(_) => Ok(()),
        Err(_) => Err(Utf8Error {}),
    }
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64")))]
pub(crate) struct Utf8CheckingState<T> {
    pub prev: T,
    pub incomplete: T,
    pub error: T,
}
