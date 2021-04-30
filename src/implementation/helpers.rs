type Utf8ErrorCompat = crate::compat::Utf8Error;

#[inline]
pub(crate) fn validate_utf8_at_offset(input: &[u8], offset: usize) -> Result<(), Utf8ErrorCompat> {
    #[allow(clippy::cast_possible_truncation)]
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
#[allow(dead_code)]
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
            .into_iter()
            .find(|i| input[failing_block_pos - i] >> 6 != 0b10)
            .map_or(failing_block_pos, |i| failing_block_pos - i)
    };
    // UNWRAP: safe because the SIMD UTF-8 validation found an error
    validate_utf8_at_offset(input, offset).unwrap_err()
}

pub(crate) const SIMD_CHUNK_SIZE: usize = 64;

#[repr(C, align(32))]
#[allow(dead_code)]
pub(crate) struct Utf8CheckAlgorithm<T> {
    pub(crate) prev: T,
    pub(crate) incomplete: T,
    pub(crate) error: T,
}

#[repr(C, align(16))]
#[allow(dead_code)]
pub(crate) struct Temp2xSimdChunkA16(
    pub(crate) [u8; SIMD_CHUNK_SIZE],
    pub(crate) [u8; SIMD_CHUNK_SIZE],
);

#[allow(dead_code)]
impl Temp2xSimdChunkA16 {
    #[inline]
    pub(crate) const fn new() -> Self {
        Self([0; SIMD_CHUNK_SIZE], [0; SIMD_CHUNK_SIZE])
    }
}

#[repr(C, align(32))]
#[allow(dead_code)]
pub(crate) struct Temp2xSimdChunkA32(
    pub(crate) [u8; SIMD_CHUNK_SIZE],
    pub(crate) [u8; SIMD_CHUNK_SIZE],
);

#[allow(dead_code)]
impl Temp2xSimdChunkA32 {
    #[inline]
    pub(crate) const fn new() -> Self {
        Self([0; SIMD_CHUNK_SIZE], [0; SIMD_CHUNK_SIZE])
    }
}

#[derive(Clone, Copy)]
pub(crate) struct SimdU8Value<T>(pub(crate) T)
where
    T: Copy;
