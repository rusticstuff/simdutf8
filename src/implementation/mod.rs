//! Contains UTF-8 validation implementations.

use crate::{Utf8Error, Utf8ErrorExact};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64")))]
#[macro_use]
mod macros;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64")))]
#[macro_use]
mod x86;

/// UTF-8 validation function type
#[allow(dead_code)]
pub(crate) type ValidateUtf8Fn = unsafe fn(input: &[u8]) -> Result<(), Utf8Error>;

#[cfg_attr(not(feature = "no-inline"), inline)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) unsafe fn validate_utf8(input: &[u8]) -> Result<(), Utf8Error> {
    x86::validate_utf8(input)
}

#[cfg_attr(not(feature = "no-inline"), inline)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) unsafe fn validate_utf8_exact(input: &[u8]) -> Result<(), Utf8ErrorExact> {
    x86::validate_utf8_exact(input)
}

#[cfg(all(not(any(target_arch = "x86", target_arch = "x86_64")), feature = "std"))]
pub(crate) fn get_fastest_available_implementation() -> ValidateUtf8Fn {
    validate_utf8_fallback
}

#[cfg(all(not(any(target_arch = "x86", target_arch = "x86_64")), feature = "std"))]
pub(crate) fn get_fastest_available_implementation() -> ValidateUtf8ExactFn {
    unimplemented!();
}

#[cfg_attr(not(feature = "no-inline"), inline)]
#[allow(dead_code)]
fn validate_utf8_fallback(input: &[u8]) -> Result<(), Utf8Error> {
    match core::str::from_utf8(input) {
        Ok(_) => Ok(()),
        Err(_) => Err(Utf8Error {}),
    }
}

fn get_error(input: &[u8], offset: usize) -> Utf8ErrorExact {
    use std::convert::TryFrom;
    let err = std::str::from_utf8(&input[offset..]).unwrap_err();
    Utf8ErrorExact {
        valid_up_to: err.valid_up_to() + offset,
        error_len: err.error_len().map(|len| {
            #[allow(clippy::unwrap_used)]
            u8::try_from(len).unwrap()
        }),
    }
}

#[cold]
fn get_exact_error(input: &[u8], failing_block_pos: usize) -> Utf8ErrorExact {
    if failing_block_pos == 0 {
        get_error(input, 0)
    } else {
        // previous block is OK except for maybe continuation of the block boundary
        // so find the starting index for from_utf8()
        for i in 1..=3 {
            if input[failing_block_pos - i] >> 6 != 0b10 {
                // not a continuation byte, so start here
                return get_error(input, failing_block_pos - i);
            }
        }
        // three continuation bytes found ending the previous block so it must
        // end with a four byte UTF-8 codepoint meaning that the previous block
        // is complete and valid UTF-8. Just need to check the current block.
        get_error(input, failing_block_pos)
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
struct Utf8CheckingState<T> {
    pub prev: T,
    pub incomplete: T,
    pub error: T,
}
