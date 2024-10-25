//! Contains UTF-8 validation implementations.

#![forbid(unsafe_code)]

pub(crate) mod simd;

#[inline]
pub(crate) fn validate_utf8_basic(input: &[u8]) -> Result<(), crate::basic::Utf8Error> {
    if input.len() < simd::SIMD_CHUNK_SIZE {
        return validate_utf8_basic_fallback(input);
    }

    validate_utf8_basic_simd(input)
}

#[inline(never)]
fn validate_utf8_basic_simd(input: &[u8]) -> Result<(), crate::basic::Utf8Error> {
    simd::auto::validate_utf8_basic(input)
}

#[inline]
pub(crate) fn validate_utf8_compat(input: &[u8]) -> Result<(), crate::compat::Utf8Error> {
    if input.len() < simd::SIMD_CHUNK_SIZE {
        return validate_utf8_compat_fallback(input);
    }

    validate_utf8_compat_simd(input)
}

fn validate_utf8_compat_simd(input: &[u8]) -> Result<(), crate::compat::Utf8Error> {
    simd::auto::validate_utf8_compat(input)
}

// fallback method implementations
#[inline]
pub(crate) const fn validate_utf8_basic_fallback(
    input: &[u8],
) -> Result<(), crate::basic::Utf8Error> {
    match core::str::from_utf8(input) {
        Ok(_) => Ok(()),
        Err(_) => Err(crate::basic::Utf8Error {}),
    }
}

#[inline]
pub(crate) fn validate_utf8_compat_fallback(input: &[u8]) -> Result<(), crate::compat::Utf8Error> {
    validate_utf8_at_offset(input, 0)
}

type Utf8ErrorCompat = crate::compat::Utf8Error;

#[inline]
#[expect(clippy::cast_possible_truncation)]
pub(crate) fn validate_utf8_at_offset(input: &[u8], offset: usize) -> Result<(), Utf8ErrorCompat> {
    match core::str::from_utf8(&input[offset..]) {
        Ok(_) => Ok(()),
        Err(err) => Err(Utf8ErrorCompat {
            valid_up_to: err.valid_up_to() + offset,
            error_len: err.error_len().map(|len| {
                // never truncates since std::str::err::Utf8Error::error_len() never returns value larger than 4
                len as u8
            }),
        }),
    }
}

#[cold]
#[expect(clippy::unwrap_used)]
#[allow(dead_code)] // only used if there is a SIMD implementation
pub(crate) fn get_compat_error(input: &[u8], failing_block_pos: usize) -> Utf8ErrorCompat {
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
            .find(|i| input[failing_block_pos - i] >> 6 != 0b10)
            .map_or(failing_block_pos, |i| failing_block_pos - i)
    };
    // UNWRAP: safe because the SIMD UTF-8 validation found an error
    validate_utf8_at_offset(input, offset).unwrap_err()
}
