//! Contains UTF-8 validation implementations.

type Utf8ErrorCompat = crate::compat::Utf8Error;
type Utf8ErrorBasic = crate::basic::Utf8Error;

#[allow(unused_macros)]
#[macro_use]
mod macros;

// UTF-8 validation function types

#[allow(dead_code)]
type ValidateUtf8Fn = unsafe fn(input: &[u8]) -> Result<(), Utf8ErrorBasic>;

#[allow(dead_code)]
type ValidateUtf8CompatFn = unsafe fn(input: &[u8]) -> Result<(), Utf8ErrorCompat>;

// arch-specific functions

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64")))]
#[macro_use]
pub(crate) mod x86;

/// Fn needed instead of re-import, otherwise not inlined in non-std case
#[allow(clippy::inline_always)]
#[inline(always)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(super) unsafe fn validate_utf8_basic(input: &[u8]) -> Result<(), Utf8ErrorBasic> {
    x86::validate_utf8_basic(input)
}

/// Fn needed of re-import, otherwise not inlined in non-std case
#[allow(clippy::inline_always)]
#[inline(always)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(super) unsafe fn validate_utf8_compat(input: &[u8]) -> Result<(), Utf8ErrorCompat> {
    x86::validate_utf8_compat(input)
}

// fallback for non-x86

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub(super) use validate_utf8_basic_fallback as validate_utf8_basic;

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub(super) use validate_utf8_compat_fallback as validate_utf8_compat;

// fallback method implementions

#[inline]
#[allow(dead_code)]
pub(crate) fn validate_utf8_basic_fallback(input: &[u8]) -> Result<(), Utf8ErrorBasic> {
    match core::str::from_utf8(input) {
        Ok(_) => Ok(()),
        Err(_) => Err(Utf8ErrorBasic {}),
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
#[allow(dead_code)]
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

#[inline]
#[allow(dead_code)]
#[allow(clippy::missing_const_for_fn)] // cannot really be const
unsafe fn memcpy_unaligned_nonoverlapping_inline(
    mut src: *const u8,
    mut dest: *mut u8,
    mut len: usize,
) {
    while len >= 8 {
        #[allow(clippy::cast_ptr_alignment)]
        dest.cast::<u64>()
            .write_unaligned(src.cast::<u64>().read_unaligned());
        src = src.offset(8);
        dest = dest.offset(8);
        len -= 8;
    }
    while len > 0 {
        *dest = *src;
        src = src.offset(1);
        dest = dest.offset(1);
        len -= 1;
    }
}

#[repr(C, align(32))]
#[allow(dead_code)]
struct Utf8CheckingState<T> {
    prev: T,
    incomplete: T,
    error: T,
}

#[repr(C, align(32))]
#[allow(dead_code)]
struct Temp2x64A32([u8; 64], [u8; 64]);
