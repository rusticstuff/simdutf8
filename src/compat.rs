//! Compat module for full compatibility with `std::from_utf8`

use core::str::{from_utf8_unchecked, from_utf8_unchecked_mut};

use crate::implementation::validate_utf8_compat;

/// UTF-8 validation error compatible with `std::str::err::Utf8Error`
#[derive(Debug)]
pub struct Utf8Error {
    pub(crate) valid_up_to: usize,
    pub(crate) error_len: Option<u8>,
}

impl Utf8Error {
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

/// Checks if the byte sequence is valid UTF-8 and returns `Ok(str)` if it is.
///
/// # Errors
/// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
pub fn from_utf8(input: &[u8]) -> Result<&str, Utf8Error> {
    unsafe {
        validate_utf8_compat(input)?;
        Ok(from_utf8_unchecked(input))
    }
}

/// Checks if the byte sequence is valid UTF-8 and returns `Ok(str)` if it is.
///
/// # Errors
/// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
pub fn from_utf8_mut(input: &mut [u8]) -> Result<&mut str, Utf8Error> {
    unsafe {
        validate_utf8_compat(input)?;
        Ok(from_utf8_unchecked_mut(input))
    }
}
