#![deny(warnings)]
#![warn(unused_extern_crates)]
#![deny(
    clippy::all,
    clippy::unwrap_used,
    clippy::unnecessary_unwrap,
    clippy::pedantic
)]
#![deny(missing_docs)]
#![cfg_attr(feature = "hints", feature(core_intrinsics))]
#![cfg_attr(not(feature = "std"), no_std)]

//! UTF-8 checking crate

pub mod implementation;

/// UTF-8 validation error
#[derive(Debug)]
pub struct Utf8Error {}

/// Checks if the byte sequence is valid UTF-8 and returns `Ok(str)` if it is.
///
/// # Errors
/// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
#[cfg(all(
    not(feature = "std"),
    not(target_feature = "avx2"),
    not(target_feature = "sse4.2")
))]
pub fn from_utf8(input: &[u8]) -> core::result::Result<&str, Utf8Error> {
    unsafe {
        implementation::validate_utf8_fallback(input)?;
        Ok(core::str::from_utf8_unchecked(input))
    }
}

/// Checks if the byte sequence is valid UTF-8 and returns `Ok(str)` if it is.
///
/// # Errors
/// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
#[cfg(all(
    not(feature = "std"),
    not(target_feature = "avx2"),
    target_feature = "sse4.2"
))]
pub fn from_utf8(input: &[u8]) -> core::result::Result<&str, Utf8Error> {
    unsafe {
        implementation::sse42::validate_utf8_simd(input)?;
        Ok(core::str::from_utf8_unchecked(input))
    }
}

/// Checks if the byte sequence is valid UTF-8 and returns `Ok(str)` if it is.
///
/// # Errors
/// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
#[cfg(target_feature = "avx2")]
pub fn from_utf8(input: &[u8]) -> core::result::Result<&str, Utf8Error> {
    unsafe {
        implementation::avx2::validate_utf8_simd(input)?;
        Ok(core::str::from_utf8_unchecked(input))
    }
}

/// Checks if the byte sequence is valid UTF-8 and returns `Ok(str)` if it is.
///
/// # Errors
/// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
#[cfg(all(feature = "std", not(target_feature = "avx2")))]
pub fn from_utf8(input: &[u8]) -> core::result::Result<&str, Utf8Error> {
    implementation::get_fastest_available_implementation()(input)?;
    unsafe { Ok(core::str::from_utf8_unchecked(input)) }
}

#[cfg(test)]
mod tests;
