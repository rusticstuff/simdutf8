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
    use std::sync::atomic::{AtomicU8, Ordering};

    const UNINIT: u8 = 0;
    const AVX2: u8 = 1;
    const SSE42: u8 = 2;
    const FALLBACK: u8 = 3;

    static METHOD: AtomicU8 = AtomicU8::new(UNINIT);

    match METHOD.load(Ordering::Relaxed) {
        AVX2 => unsafe {
            implementation::avx2::validate_utf8_simd(input)?;
        },
        SSE42 => unsafe {
            implementation::sse42::validate_utf8_simd(input)?;
        },
        FALLBACK => {
            implementation::validate_utf8_fallback(input)?;
        }
        _ => {
            if implementation::avx2::get_implementation().is_some() {
                METHOD.store(AVX2, Ordering::Relaxed);
                unsafe {
                    implementation::avx2::validate_utf8_simd(input)?;
                }
            } else if implementation::sse42::get_implementation().is_some() {
                METHOD.store(SSE42, Ordering::Relaxed);
                unsafe {
                    implementation::sse42::validate_utf8_simd(input)?;
                }
            } else {
                METHOD.store(FALLBACK, Ordering::Relaxed);
                implementation::validate_utf8_fallback(input)?;
            }
        }
    }
    unsafe { Ok(core::str::from_utf8_unchecked(input)) }
}

#[cfg(test)]
mod tests;
