//! Pure module for maximum speed on valid UTF-8 at the expense of early error
//! detection and error details.

use core::str::{from_utf8_unchecked, from_utf8_unchecked_mut};

use crate::implementation::validate_utf8_pure;

/// Simple UTF-8 error. The SIMD implementation does not provide further information.
#[derive(Debug)]
pub struct Utf8Error {}

/// Checks if the byte sequence is valid UTF-8 and returns `Ok(str)` if it is.
///
/// # Errors
/// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
pub fn from_utf8(input: &[u8]) -> Result<&str, Utf8Error> {
    unsafe {
        validate_utf8_pure(input)?;
        Ok(from_utf8_unchecked(input))
    }
}

/// Checks if the byte sequence is valid UTF-8 and returns `Ok(str)` if it is.
///
/// # Errors
/// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
pub fn from_utf8_mut(input: &mut [u8]) -> Result<&mut str, Utf8Error> {
    unsafe {
        validate_utf8_pure(input)?;
        Ok(from_utf8_unchecked_mut(input))
    }
}
