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

#[cfg(feature = "expose_implementations")]
pub mod imp {
    //! Architecture-specific implementation exposed.
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub mod x86 {
        //! x86-specific implementation exposed.
        pub mod avx2 {
            //! AVX-2 implementation exposed.

            use super::super::super::Utf8Error;
            use crate::implementation::x86::avx2::validate_utf8_compat_simd as validate_utf8_compat;
            use core::str::{from_utf8_unchecked, from_utf8_unchecked_mut};

            /// AVX 2 implementation of `crate::compat::from_utf8()`
            ///
            /// # Errors
            /// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
            pub fn from_utf8(input: &[u8]) -> Result<&str, Utf8Error> {
                unsafe {
                    validate_utf8_compat(input)?;
                    Ok(from_utf8_unchecked(input))
                }
            }

            /// AVX 2 implementation of `crate::compat::from_utf8_mut()`
            ///
            /// # Errors
            /// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
            pub fn from_utf8_mut(input: &mut [u8]) -> Result<&mut str, Utf8Error> {
                unsafe {
                    validate_utf8_compat(input)?;
                    Ok(from_utf8_unchecked_mut(input))
                }
            }
        }

        pub mod sse42 {
            //! SSE 4.2 implementation exposed.

            use super::super::super::Utf8Error;
            use crate::implementation::x86::sse42::validate_utf8_compat_simd as validate_utf8_compat;
            use core::str::{from_utf8_unchecked, from_utf8_unchecked_mut};

            /// SSE 4.2 implementation of `crate::compat::from_utf8()`
            ///
            /// # Errors
            /// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
            pub fn from_utf8(input: &[u8]) -> Result<&str, Utf8Error> {
                unsafe {
                    validate_utf8_compat(input)?;
                    Ok(from_utf8_unchecked(input))
                }
            }

            /// SSE 4.2 implementation of `crate::compat::from_utf8_mut()`
            ///
            /// # Errors
            /// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
            pub fn from_utf8_mut(input: &mut [u8]) -> Result<&mut str, Utf8Error> {
                unsafe {
                    validate_utf8_compat(input)?;
                    Ok(from_utf8_unchecked_mut(input))
                }
            }
        }
    }
}
