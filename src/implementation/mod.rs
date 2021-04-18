//! Contains UTF-8 validation implementations.

type Utf8ErrorCompat = crate::compat::Utf8Error;
type Utf8ErrorPure = crate::pure::Utf8Error;

#[macro_use]
mod macros;

// UTF-8 validation function types

#[allow(dead_code)]
type ValidateUtf8Fn = unsafe fn(input: &[u8]) -> Result<(), Utf8ErrorPure>;

#[allow(dead_code)]
type ValidateUtf8CompatFn = unsafe fn(input: &[u8]) -> Result<(), Utf8ErrorCompat>;

// arch-specific imports

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64")))]
#[macro_use]
mod x86;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) use x86::validate_utf8_pure;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) use x86::validate_utf8_compat;

// fallback methods

#[inline]
#[allow(dead_code)]
pub(crate) fn validate_utf8_pure_fallback(input: &[u8]) -> Result<(), Utf8ErrorPure> {
    match core::str::from_utf8(input) {
        Ok(_) => Ok(()),
        Err(_) => Err(Utf8ErrorPure {}),
    }
}

#[inline]
#[allow(dead_code)]
pub(crate) fn validate_utf8_compat_fallback(input: &[u8]) -> Result<(), Utf8ErrorCompat> {
    validate_utf8_at_offset(input, 0)
}

// implementation helpers

#[inline]
fn validate_utf8_at_offset(input: &[u8], offset: usize) -> Result<(), Utf8ErrorCompat> {
    use core::convert::TryFrom;
    match core::str::from_utf8(&input[offset..]) {
        Ok(_) => Ok(()),
        Err(err) => Err(Utf8ErrorCompat {
            valid_up_to: err.valid_up_to() + offset,
            error_len: err.error_len().map(|len| {
                #[allow(clippy::unwrap_used)]
                // never panics since std::str::err::Utf8Error::error_len() never returns value larger than 4
                u8::try_from(len).unwrap()
            }),
        }),
    }
}

#[cold]
fn get_compat_error(input: &[u8], failing_block_pos: usize) -> Utf8ErrorCompat {
    let offset = if failing_block_pos == 0 {
        // Error must be in this block since it is the first.
        0
    } else {
        // The previous block is OK except for a possible continuation over the block boundary.
        // We go backwards over the last three bytes of the previous block and find the
        // last non-continuation byte as a starting point for an std validation. If the last
        // three bytes are all continuation bytes then the previous block ends with a four byte
        // UTF-8 codepoint, is thus complete and valid UTF-8. We start the check with the
        // current block in that case.
        (1..=3)
            .into_iter()
            .find(|i| input[failing_block_pos - i] >> 6 != 0b10)
            .map_or(failing_block_pos, |i| failing_block_pos - i)
    };
    validate_utf8_at_offset(input, offset).unwrap_err()
}

#[repr(C, align(64))]
struct Utf8CheckingState<T> {
    prev: T,
    incomplete: T,
    error: T,
}

#[repr(C, align(64))]
struct AlignToSixtyFour([u8; 64], [u8; 64]);
