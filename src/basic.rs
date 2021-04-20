//! Pure module for maximum speed on valid UTF-8 at the expense of early error
//! detection and error details.

use core::str::{from_utf8_unchecked, from_utf8_unchecked_mut};

use crate::implementation::validate_utf8_basic;

/// Simple UTF-8 error. The SIMD implementation does not provide further information.
#[derive(Copy, Eq, PartialEq, Clone, Debug)]
pub struct Utf8Error {}

/// Checks if the byte sequence is valid UTF-8 and returns `Ok(str)` if it is.
///
/// # Errors
/// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
pub fn from_utf8(input: &[u8]) -> Result<&str, Utf8Error> {
    unsafe {
        validate_utf8_basic(input)?;
        Ok(from_utf8_unchecked(input))
    }
}

/// Checks if the byte sequence is valid UTF-8 and returns `Ok(str)` if it is.
///
/// # Errors
/// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
pub fn from_utf8_mut(input: &mut [u8]) -> Result<&mut str, Utf8Error> {
    unsafe {
        validate_utf8_basic(input)?;
        Ok(from_utf8_unchecked_mut(input))
    }
}

/// imp mod
#[cfg(feature = "public_imp")]
pub mod imp {
    /// x86 mod
    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64")))]
    pub mod x86 {
        /// avx2 mod
        pub mod avx2 {
            pub use crate::implementation::x86::avx2::validate_utf8_basic as validate_utf8;
        }
        /// sse42 mod
        pub mod sse42 {
            pub use crate::implementation::x86::sse42::validate_utf8_basic as validate_utf8;
        }
    }
}
