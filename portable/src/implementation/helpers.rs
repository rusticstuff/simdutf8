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

#[allow(dead_code)] // only used if there is a SIMD implementation
#[inline]
pub(crate) unsafe fn memcpy_unaligned_nonoverlapping_inline_opt_lt_64(
    src: *const u8,
    dest: *mut u8,
    len: usize,
) {
    src.copy_to_nonoverlapping(dest, len);
}

pub(crate) const SIMD_CHUNK_SIZE: usize = 64;

#[repr(C, align(32))]
#[allow(dead_code)] // only used if there is a SIMD implementation
pub(crate) struct Utf8CheckAlgorithm<T> {
    pub(crate) prev: T,
    pub(crate) incomplete: T,
    pub(crate) error: T,
}

#[repr(C, align(16))]
#[allow(dead_code)] // only used if a 128-bit SIMD implementation is used
pub(crate) struct TempSimdChunkA16(pub(crate) [u8; SIMD_CHUNK_SIZE]);

#[allow(dead_code)] // only used if there is a SIMD implementation
impl TempSimdChunkA16 {
    #[expect(clippy::inline_always)]
    #[inline(always)] // needs to be forced because otherwise it is not inlined on armv7 neo
    pub(crate) const fn new() -> Self {
        Self([0; SIMD_CHUNK_SIZE])
    }
}

#[repr(C, align(32))]
#[allow(dead_code)] // only used if a 256-bit SIMD implementation is used
pub(crate) struct TempSimdChunkA32(pub(crate) [u8; SIMD_CHUNK_SIZE]);

#[allow(dead_code)] // only used if there is a SIMD implementation
impl TempSimdChunkA32 {
    #[expect(clippy::inline_always)]
    #[inline(always)] // needs to be forced because otherwise it is not inlined on armv7 neo
    pub(crate) const fn new() -> Self {
        Self([0; SIMD_CHUNK_SIZE])
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)] // only used if there is a SIMD implementation
pub(crate) struct SimdU8Value<T>(pub(crate) T)
where
    T: Copy;