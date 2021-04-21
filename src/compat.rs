//! Compat module for full compatibility with `std::from_utf8`

use core::fmt::Display;
use core::fmt::Formatter;

use core::str::{from_utf8_unchecked, from_utf8_unchecked_mut};

use crate::implementation::validate_utf8_compat;

/// UTF-8 validation error compatible with `std::str::err::Utf8Error`
#[derive(Copy, Eq, PartialEq, Clone, Debug)]
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

impl Display for Utf8Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        if let Some(error_len) = self.error_len {
            write!(
                f,
                "invalid utf-8 sequence of {} bytes from index {}",
                error_len, self.valid_up_to
            )
        } else {
            write!(
                f,
                "incomplete utf-8 byte sequence from index {}",
                self.valid_up_to
            )
        }
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
/// imp mod
#[cfg(feature = "public_imp")]
#[cfg_attr(docsrs, doc(cfg(feature = "public_imp")))]
pub mod imp {
    /// x86 mod
    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64")))]
    pub mod x86 {
        /// avx2 mod
        pub mod avx2 {
            pub use crate::implementation::x86::avx2::validate_utf8_compat as validate_utf8;
        }
        /// sse42 mod
        pub mod sse42 {
            pub use crate::implementation::x86::sse42::validate_utf8_compat as validate_utf8;
        }
    }
}
