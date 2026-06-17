//! Contains the x86-64 AVX512 UTF-8 validation implementation.

#[cfg(target_arch = "x86")]
use core::arch::x86::{
    __m512i, _mm512_alignr_epi8, _mm512_loadu_si512, _mm512_maskz_loadu_epi8, _mm512_movepi8_mask,
    _mm512_or_si512, _mm512_permutex2var_epi64, _mm512_permutexvar_epi8, _mm512_set1_epi8,
    _mm512_set_epi64, _mm512_set_epi8, _mm512_setzero_si512, _mm512_srli_epi16, _mm512_subs_epu8,
    _mm512_ternarylogic_epi32, _mm512_test_epi8_mask, _mm_prefetch, _MM_HINT_T0,
};

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
    __m512i, _mm512_alignr_epi8, _mm512_loadu_si512, _mm512_maskz_loadu_epi8, _mm512_movepi8_mask,
    _mm512_or_si512, _mm512_permutex2var_epi64, _mm512_permutexvar_epi8, _mm512_set1_epi8,
    _mm512_set_epi64, _mm512_set_epi8, _mm512_setzero_si512, _mm512_srli_epi16, _mm512_subs_epu8,
    _mm512_ternarylogic_epi32, _mm512_test_epi8_mask, _mm_prefetch, _MM_HINT_T0,
};

use crate::implementation::helpers::Utf8CheckAlgorithm;

// AVX 2 SIMD primitives

type SimdU8Value = crate::implementation::helpers::SimdU8Value<__m512i>;

impl SimdU8Value {
    #[flexpect::e(clippy::cast_possible_wrap)]
    #[flexpect::e(clippy::too_many_arguments)]
    #[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
    #[inline]
    unsafe fn from_32_cut_off_leading(
        v0: u8,
        v1: u8,
        v2: u8,
        v3: u8,
        v4: u8,
        v5: u8,
        v6: u8,
        v7: u8,
        v8: u8,
        v9: u8,
        v10: u8,
        v11: u8,
        v12: u8,
        v13: u8,
        v14: u8,
        v15: u8,
        v16: u8,
        v17: u8,
        v18: u8,
        v19: u8,
        v20: u8,
        v21: u8,
        v22: u8,
        v23: u8,
        v24: u8,
        v25: u8,
        v26: u8,
        v27: u8,
        v28: u8,
        v29: u8,
        v30: u8,
        v31: u8,
    ) -> Self {
        Self::from(_mm512_set_epi8(
            v31 as i8, v30 as i8, v29 as i8, v28 as i8, v27 as i8, v26 as i8, v25 as i8, v24 as i8,
            v23 as i8, v22 as i8, v21 as i8, v20 as i8, v19 as i8, v18 as i8, v17 as i8, v16 as i8,
            v15 as i8, v14 as i8, v13 as i8, v12 as i8, v11 as i8, v10 as i8, v9 as i8, v8 as i8,
            v7 as i8, v6 as i8, v5 as i8, v4 as i8, v3 as i8, v2 as i8, v1 as i8, v0 as i8,
            v0 as i8, v0 as i8, v0 as i8, v0 as i8, v0 as i8, v0 as i8, v0 as i8, v0 as i8,
            v0 as i8, v0 as i8, v0 as i8, v0 as i8, v0 as i8, v0 as i8, v0 as i8, v0 as i8,
            v0 as i8, v0 as i8, v0 as i8, v0 as i8, v0 as i8, v0 as i8, v0 as i8, v0 as i8,
            v0 as i8, v0 as i8, v0 as i8, v0 as i8, v0 as i8, v0 as i8, v0 as i8, v0 as i8,
        ))
    }

    #[flexpect::e(clippy::too_many_arguments)]
    #[flexpect::e(clippy::cast_possible_wrap)]
    #[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
    #[inline]
    unsafe fn repeat_16(
        v0: u8,
        v1: u8,
        v2: u8,
        v3: u8,
        v4: u8,
        v5: u8,
        v6: u8,
        v7: u8,
        v8: u8,
        v9: u8,
        v10: u8,
        v11: u8,
        v12: u8,
        v13: u8,
        v14: u8,
        v15: u8,
    ) -> Self {
        Self::from(_mm512_set_epi8(
            v15 as i8, v14 as i8, v13 as i8, v12 as i8, v11 as i8, v10 as i8, v9 as i8, v8 as i8,
            v7 as i8, v6 as i8, v5 as i8, v4 as i8, v3 as i8, v2 as i8, v1 as i8, v0 as i8,
            v15 as i8, v14 as i8, v13 as i8, v12 as i8, v11 as i8, v10 as i8, v9 as i8, v8 as i8,
            v7 as i8, v6 as i8, v5 as i8, v4 as i8, v3 as i8, v2 as i8, v1 as i8, v0 as i8,
            v15 as i8, v14 as i8, v13 as i8, v12 as i8, v11 as i8, v10 as i8, v9 as i8, v8 as i8,
            v7 as i8, v6 as i8, v5 as i8, v4 as i8, v3 as i8, v2 as i8, v1 as i8, v0 as i8,
            v15 as i8, v14 as i8, v13 as i8, v12 as i8, v11 as i8, v10 as i8, v9 as i8, v8 as i8,
            v7 as i8, v6 as i8, v5 as i8, v4 as i8, v3 as i8, v2 as i8, v1 as i8, v0 as i8,
        ))
    }

    #[flexpect::e(clippy::cast_ptr_alignment)]
    #[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
    #[inline]
    unsafe fn load_from(ptr: *const u8) -> Self {
        Self::from(_mm512_loadu_si512(ptr.cast::<__m512i>()))
    }

    #[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
    #[inline]
    unsafe fn load_from_partial(ptr: *const u8, len: usize) -> Self {
        let res = _mm512_maskz_loadu_epi8(u64::MAX >> (64 - len), ptr.cast::<i8>());
        Self::from(res)
    }

    #[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
    #[inline]
    unsafe fn lookup_16(self, tbl: Self) -> Self {
        // VPERMB (avx512vbmi) differs from other dynamic swizzle instructions in that it
        // completely ignores the high bits of the index. Only the low 6 bits of each
        // byte are used to select a byte from the 64-byte table.
        Self::from(_mm512_permutexvar_epi8(self.0, tbl.0))
    }

    #[flexpect::e(clippy::cast_possible_wrap)]
    #[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
    #[inline]
    unsafe fn splat(val: u8) -> Self {
        Self::from(_mm512_set1_epi8(val as i8))
    }

    #[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
    #[inline]
    unsafe fn splat0() -> Self {
        Self::from(_mm512_setzero_si512())
    }

    #[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
    #[inline]
    unsafe fn or(self, b: Self) -> Self {
        Self::from(_mm512_or_si512(self.0, b.0))
    }

    #[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
    #[inline]
    unsafe fn saturating_sub(self, b: Self) -> Self {
        Self::from(_mm512_subs_epu8(self.0, b.0))
    }

    // For ternary ops reference see https://www.felixcloutier.com/x86/vpternlogd:vpternlogq

    /// `self & b & c` fused into a single `vpternlogd`.
    #[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
    #[inline]
    unsafe fn and3(self, b: Self, c: Self) -> Self {
        Self::from(_mm512_ternarylogic_epi32(self.0, b.0, c.0, 0x80))
    }

    /// `(self | b) & c` fused into a single `vpternlogd`.
    #[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
    #[inline]
    unsafe fn or_and(self, b: Self, c: Self) -> Self {
        Self::from(_mm512_ternarylogic_epi32(self.0, b.0, c.0, 0xA8))
    }

    /// `(self ^ b) | c` fused into a single `vpternlogd`.
    #[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
    #[inline]
    unsafe fn xor_or(self, b: Self, c: Self) -> Self {
        Self::from(_mm512_ternarylogic_epi32(c.0, self.0, b.0, 0xF6))
    }

    /// CAVE: Uses 16-bit-lane shifts so the high nibble of every
    /// other byte is polluted with the low nibble of the neighbouring byte.
    /// This is fine for our use case since we only care about the low nibble.
    #[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
    #[inline]
    unsafe fn shr4_dirty(self) -> Self {
        Self::from(_mm512_srli_epi16(self.0, 4))
    }

    // ugly but prev<N> requires const generics
    #[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
    #[inline]
    unsafe fn prev1(self, prev: Self) -> Self {
        const SHIFT: i32 = 16 - 1;
        return Self::from(_mm512_alignr_epi8(
            self.0,
            _mm512_permutex2var_epi64(prev.0, _mm512_set_epi64(13, 12, 11, 10, 9, 8, 7, 6), self.0),
            SHIFT,
        ));
    }
    // ugly but prev<N> requires const generics
    #[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
    #[inline]
    unsafe fn prev2(self, prev: Self) -> Self {
        const SHIFT: i32 = 16 - 2;
        return Self::from(_mm512_alignr_epi8(
            self.0,
            _mm512_permutex2var_epi64(prev.0, _mm512_set_epi64(13, 12, 11, 10, 9, 8, 7, 6), self.0),
            SHIFT,
        ));
    }

    // ugly but prev<N> requires const generics
    #[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
    #[inline]
    unsafe fn prev3(self, prev: Self) -> Self {
        const SHIFT: i32 = 16 - 3;
        return Self::from(_mm512_alignr_epi8(
            self.0,
            _mm512_permutex2var_epi64(prev.0, _mm512_set_epi64(13, 12, 11, 10, 9, 8, 7, 6), self.0),
            SHIFT,
        ));
    }

    #[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
    #[inline]
    unsafe fn any_bit_set(self) -> bool {
        _mm512_test_epi8_mask(self.0, self.0) != 0
    }

    #[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
    #[inline]
    unsafe fn is_ascii(self) -> bool {
        _mm512_movepi8_mask(self.0) == 0
    }
}

impl From<__m512i> for SimdU8Value {
    #[inline]
    fn from(val: __m512i) -> Self {
        Self(val)
    }
}

/// AVX-512 specializations of `check_special_cases` and `check_multibyte_lengths`.
impl Utf8CheckAlgorithm<SimdU8Value> {
    #[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
    #[inline]
    unsafe fn check_special_cases(input: SimdU8Value, prev1: SimdU8Value) -> SimdU8Value {
        let (byte_1_high_table, byte_1_low_table, byte_2_high_table) = Self::special_case_tables();

        // `lookup_16` (VPERMB) on tables where the 128-bit lanes are all identical
        // ignores the high nibble, so the pollution by `shr4_dirty` does not matter.
        let byte_1_high = prev1.shr4_dirty().lookup_16(byte_1_high_table);
        // `lookup_16` (VPERMB) on tables where the 128-bit lanes are all identical
        // ignores the high nibble, so no masking needed.
        let byte_1_low = prev1.lookup_16(byte_1_low_table);
        let byte_2_high = input.shr4_dirty().lookup_16(byte_2_high_table);

        byte_1_high.and3(byte_1_low, byte_2_high)
    }

    #[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
    #[inline]
    unsafe fn check_multibyte_lengths(
        input: SimdU8Value,
        prev: SimdU8Value,
        special_cases: SimdU8Value,
        error: SimdU8Value,
    ) -> SimdU8Value {
        let prev2 = input.prev2(prev);
        let prev3 = input.prev3(prev);
        let is_third_byte = prev2.saturating_sub(SimdU8Value::splat(0xe0 - 0x80));
        let is_fourth_byte = prev3.saturating_sub(SimdU8Value::splat(0xf0 - 0x80));
        let must23_80 = is_third_byte.or_and(is_fourth_byte, SimdU8Value::splat(0x80));
        must23_80.xor_or(special_cases, error)
    }
}

#[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
#[inline]
unsafe fn simd_prefetch(ptr: *const u8) {
    _mm_prefetch(ptr.cast::<i8>(), _MM_HINT_T0);
}

const PREFETCH: bool = true;
#[cfg(feature = "public_imp")]
use crate::implementation::helpers::TempSimdChunkA64 as TempSimdChunk;
simd_input_512_bit!(#[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]);
algorithm_simd!(#[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]);
