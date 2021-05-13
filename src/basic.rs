//! The `basic` API flavor provides barebones UTF-8 checking at the highest speed.
//!
//! It is fastest on valid UTF-8, but only checks for errors after processing the whole byte sequence
//! and does not provide detailed information if the data is not valid UTF-8. [`Utf8Error`] is a zero-sized error struct.
//!
//! If you need detailed error information use the functions from the [`crate::compat`] module instead.

use core::str::{from_utf8_unchecked, from_utf8_unchecked_mut};

use crate::implementation::validate_utf8_basic;

/// Simple zero-sized UTF-8 error.
///
/// No information is provided where the error occured or how long the invalid byte
/// byte sequence is.
#[derive(Copy, Eq, PartialEq, Clone, Debug)]
pub struct Utf8Error;

impl core::fmt::Display for Utf8Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("invalid utf-8 sequence")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Utf8Error {}

/// Analogue to [`std::str::from_utf8()`].
///
/// Checks if the passed byte sequence is valid UTF-8 and returns an
/// [`std::str``] reference to the passed byte slice wrapped in `Ok()` if it is.
///
/// # Errors
/// Will return the zero-sized Err([`Utf8Error`]) on if the input contains invalid UTF-8.
#[inline]
pub fn from_utf8(input: &[u8]) -> Result<&str, Utf8Error> {
    unsafe {
        validate_utf8_basic(input)?;
        Ok(from_utf8_unchecked(input))
    }
}

/// Analogue to [`std::str::from_utf8_mut()`].
///
/// Checks if the passed mutable byte sequence is valid UTF-8 and returns a mutable
/// [`std::str``] reference to the passed byte slice wrapped in `Ok()` if it is.
///
/// # Errors
/// Will return the zero-sized Err([`Utf8Error`]) on if the input contains invalid UTF-8.
#[inline]
pub fn from_utf8_mut(input: &mut [u8]) -> Result<&mut str, Utf8Error> {
    unsafe {
        validate_utf8_basic(input)?;
        Ok(from_utf8_unchecked_mut(input))
    }
}

/// Allows direct access to the platform-specific unsafe validation implementations.
#[cfg(feature = "public_imp")]
#[cfg_attr(docsrs, doc(cfg(feature = "public_imp")))]
pub mod imp {
    /// Includes the x86/x86-64 SIMD implementations.
    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64")))]
    pub mod x86 {
        /// Includes the validation implementation for AVX 2-compatible CPUs.
        pub mod avx2 {
            pub use crate::implementation::x86::avx2::validate_utf8_basic as validate_utf8;
            pub use crate::implementation::x86::avx2::Utf8Validator;
        }
        /// Includes the validation implementation for SSE 4.2-compatible CPUs.
        pub mod sse42 {
            pub use crate::implementation::x86::sse42::validate_utf8_basic as validate_utf8;
            pub use crate::implementation::x86::sse42::Utf8Validator;
        }
    }

    /// Includes the aarch64 SIMD implementations.
    #[cfg(all(feature = "aarch64_neon", target_arch = "aarch64"))]
    pub mod aarch64 {
        /// Includes the validation implementation for Neon SIMD.
        pub mod neon {
            pub use crate::implementation::aarch64::neon::validate_utf8_basic as validate_utf8;
            pub use crate::implementation::aarch64::neon::Utf8Validator;
        }
    }
}
