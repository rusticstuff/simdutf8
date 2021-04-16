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

/// Checks if the byte sequence is valid UTF-8 and returns `Ok(str)` if it is.
///
/// # Errors
/// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
pub fn from_utf8(input: &[u8]) -> core::result::Result<&str, Utf8Error> {
    unsafe {
        implementation::validate_utf8(input)?;
        Ok(core::str::from_utf8_unchecked(input))
    }
}

/// Checks if the byte sequence is valid UTF-8 and returns `Ok(str)` if it is.
///
/// # Errors
/// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
pub fn from_utf8_mut(input: &mut [u8]) -> core::result::Result<&mut str, Utf8Error> {
    unsafe {
        implementation::validate_utf8(input)?;
        Ok(core::str::from_utf8_unchecked_mut(input))
    }
}

/// Checks if the byte sequence is valid UTF-8 and returns `Ok(str)` if it is.
///
/// # Errors
/// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
pub fn from_utf8_exact(input: &[u8]) -> core::result::Result<&str, Utf8ErrorExact> {
    unsafe {
        implementation::validate_utf8_exact(input)?;
        Ok(core::str::from_utf8_unchecked(input))
    }
}

/// Checks if the byte sequence is valid UTF-8 and returns `Ok(str)` if it is.
///
/// # Errors
/// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
pub fn from_utf8_mut_exact(input: &mut [u8]) -> core::result::Result<&mut str, Utf8ErrorExact> {
    unsafe {
        implementation::validate_utf8_exact(input)?;
        Ok(core::str::from_utf8_unchecked_mut(input))
    }
}

/// UTF-8 validation error
#[derive(Debug)]
pub struct Utf8Error {}

/// Exact UTF-8 validation error
#[derive(Debug)]
pub struct Utf8ErrorExact {
    pub(crate) valid_up_to: usize,
    pub(crate) error_len: Option<u8>,
}

impl Utf8ErrorExact {
    /// Returns the index in the given string up to which valid UTF-8 was
    /// verified.
    #[inline]
    #[must_use]
    pub fn valid_up_to(&self) -> usize {
        self.valid_up_to
    }

    /// Provides more information about the failure.
    #[inline]
    #[must_use]
    pub fn error_len(&self) -> Option<usize> {
        self.error_len.map(|len| len as usize)
    }
}

#[cfg(test)]
mod tests;
