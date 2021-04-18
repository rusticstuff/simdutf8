//! Pure module for maximum speed on valid UTF-8 at the expense of early error
//! detection and error details.

use core::str::{from_utf8_unchecked, from_utf8_unchecked_mut};

use crate::implementation::validate_utf8_pure;

/// Simple UTF-8 error. The SIMD implementation does not provide further information.
#[derive(Copy, Eq, PartialEq, Clone, Debug)]
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

#[cfg(feature = "expose_implementations")]
pub mod imp {
    //! Architecture-specific implementation exposed.
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub mod x86 {
        //! x86-specific implementation exposed.
        pub mod avx2 {
            //! AVX-2 implementation exposed.

            use super::super::super::Utf8Error;
            use crate::implementation::x86::avx2::validate_utf8_pure_simd_always_inline as validate_utf8_pure;
            use core::str::{from_utf8_unchecked, from_utf8_unchecked_mut};

            /// AVX 2 implementation of `crate::compat::from_utf8()`
            ///
            /// # Errors
            /// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
            pub fn from_utf8(input: &[u8]) -> Result<&str, Utf8Error> {
                unsafe {
                    validate_utf8_pure(input)?;
                    Ok(from_utf8_unchecked(input))
                }
            }

            /// AVX 2 implementation of `crate::compat::from_utf8_mut()`
            ///
            /// # Errors
            /// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
            pub fn from_utf8_mut(input: &mut [u8]) -> Result<&mut str, Utf8Error> {
                unsafe {
                    validate_utf8_pure(input)?;
                    Ok(from_utf8_unchecked_mut(input))
                }
            }
        }

        pub mod sse42 {
            //! SSE 4.2 implementation exposed.

            use super::super::super::Utf8Error;
            use crate::implementation::x86::sse42::validate_utf8_pure_simd_always_inline as validate_utf8_pure;
            use core::str::{from_utf8_unchecked, from_utf8_unchecked_mut};

            /// SSE 4.2 implementation of `crate::compat::from_utf8()`
            ///
            /// # Errors
            /// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
            pub fn from_utf8(input: &[u8]) -> Result<&str, Utf8Error> {
                unsafe {
                    validate_utf8_pure(input)?;
                    Ok(from_utf8_unchecked(input))
                }
            }

            /// SSE 4.2 implementation of `crate::compat::from_utf8_mut()`
            ///
            /// # Errors
            /// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
            pub fn from_utf8_mut(input: &mut [u8]) -> Result<&mut str, Utf8Error> {
                unsafe {
                    validate_utf8_pure(input)?;
                    Ok(from_utf8_unchecked_mut(input))
                }
            }
        }
    }
}
