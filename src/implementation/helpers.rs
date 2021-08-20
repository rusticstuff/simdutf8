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

#[inline]
unsafe fn memcpy_u32(src: &mut *const u8, dest: &mut *mut u8) {
    #[allow(clippy::cast_ptr_alignment)]
    dest.cast::<u32>()
        .write_unaligned(src.cast::<u32>().read_unaligned());
    *src = src.offset(4);
    *dest = dest.offset(4);
}

#[inline]
unsafe fn memcpy_u64(src: &mut *const u8, dest: &mut *mut u8) {
    #[allow(clippy::cast_ptr_alignment)]
    dest.cast::<u64>()
        .write_unaligned(src.cast::<u64>().read_unaligned());
    *src = src.offset(8);
    *dest = dest.offset(8);
}

#[allow(dead_code)]
#[allow(clippy::missing_const_for_fn)] // clippy is wrong, it cannot really be const
pub(crate) unsafe fn memcpy_unaligned_nonoverlapping_inline_opt_lt_16(
    mut src: *const u8,
    mut dest: *mut u8,
    mut len: usize,
) {
    if len >= 8 {
        memcpy_u32(&mut src, &mut dest);
        memcpy_u32(&mut src, &mut dest);
        len -= 8;
    }
    if len >= 4 {
        memcpy_u32(&mut src, &mut dest);
        len -= 4;
    }
    while len > 0 {
        *dest = *src;
        src = src.offset(1);
        dest = dest.offset(1);
        len -= 1;
    }
}

#[allow(dead_code)]
#[allow(clippy::missing_const_for_fn)] // clippy is wrong, it cannot really be const
pub(crate) unsafe fn memcpy_unaligned_nonoverlapping_inline_opt_lt_32(
    mut src: *const u8,
    mut dest: *mut u8,
    mut len: usize,
) {
    if len >= 16 {
        memcpy_u64(&mut src, &mut dest);
        memcpy_u64(&mut src, &mut dest);
        len -= 16;
    }
    if len >= 8 {
        memcpy_u64(&mut src, &mut dest);
        len -= 8;
    }
    if len >= 4 {
        memcpy_u32(&mut src, &mut dest);
        len -= 4;
    }
    while len > 0 {
        *dest = *src;
        src = src.offset(1);
        dest = dest.offset(1);
        len -= 1;
    }
}

#[allow(dead_code)]
#[allow(clippy::missing_const_for_fn)] // clippy is wrong, it cannot really be const
pub(crate) unsafe fn memcpy_unaligned_nonoverlapping_inline_opt_lt_64(
    mut src: *const u8,
    mut dest: *mut u8,
    mut len: usize,
) {
    if len >= 32 {
        memcpy_u64(&mut src, &mut dest);
        memcpy_u64(&mut src, &mut dest);
        memcpy_u64(&mut src, &mut dest);
        memcpy_u64(&mut src, &mut dest);
        len -= 32;
    }
    if len >= 16 {
        memcpy_u64(&mut src, &mut dest);
        memcpy_u64(&mut src, &mut dest);
        len -= 16;
    }
    if len >= 8 {
        memcpy_u64(&mut src, &mut dest);
        len -= 8;
    }
    if len >= 4 {
        memcpy_u32(&mut src, &mut dest);
        len -= 4;
    }
    while len > 0 {
        *dest = *src;
        src = src.offset(1);
        dest = dest.offset(1);
        len -= 1;
    }
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
pub(crate) struct TempSimdChunkA16(pub(crate) [u8; SIMD_CHUNK_SIZE]);

#[allow(dead_code)]
impl TempSimdChunkA16 {
    #[inline]
    pub(crate) const fn new() -> Self {
        Self([0; SIMD_CHUNK_SIZE])
    }
}

#[repr(C, align(32))]
#[allow(dead_code)]
pub(crate) struct TempSimdChunkA32(pub(crate) [u8; SIMD_CHUNK_SIZE]);

#[allow(dead_code)]
impl TempSimdChunkA32 {
    #[inline]
    pub(crate) const fn new() -> Self {
        Self([0; SIMD_CHUNK_SIZE])
    }
}

#[derive(Clone, Copy)]
pub(crate) struct SimdU8Value<T>(pub(crate) T)
where
    T: Copy;

#[cfg(test)]
impl<T: Copy> core::fmt::Display for SimdU8Value<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        unsafe {
            if core::mem::size_of::<T>() == 16 {
                let arr: [u8; 16] = core::mem::transmute_copy(&self.0);
                write!(f, "{:?}", arr)
            } else if core::mem::size_of::<T>() == 32 {
                let arr: [u8; 32] = core::mem::transmute_copy(&self.0);
                write!(f, "{:?}", arr)
            } else {
                Err(core::fmt::Error)
            }
        }
    }
}

#[cfg(test)]
impl<T: Copy> core::fmt::LowerHex for SimdU8Value<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        unsafe {
            if core::mem::size_of::<T>() == 16 {
                let arr: [u8; 16] = core::mem::transmute_copy(&self.0);
                write!(f, "{:x?}", arr)
            } else if core::mem::size_of::<T>() == 32 {
                let arr: [u8; 32] = core::mem::transmute_copy(&self.0);
                write!(f, "{:x?}", arr)
            } else {
                Err(core::fmt::Error)
            }
        }
    }
}
