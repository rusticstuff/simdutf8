//! Contains the x86-64/x86 AVX2 UTF-8 validation implementation.

#![allow(clippy::too_many_arguments)]

#[cfg(target_arch = "x86")]
use core::arch::x86::{
    __m256i, _mm256_alignr_epi8, _mm256_and_si256, _mm256_blendv_ps, _mm256_castps_si256,
    _mm256_castsi256_ps, _mm256_cmpgt_epi8, _mm256_loadu_si256, _mm256_maskload_epi32,
    _mm256_movemask_epi8, _mm256_or_si256, _mm256_permute2x128_si256, _mm256_set1_epi32,
    _mm256_set1_epi8, _mm256_set_epi32, _mm256_setr_epi8, _mm256_setzero_si256,
    _mm256_shuffle_epi8, _mm256_sllv_epi32, _mm256_srli_epi16, _mm256_subs_epu8,
    _mm256_testz_si256, _mm256_xor_si256, _mm_prefetch, _MM_HINT_T0,
};
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
    __m256i, _mm256_alignr_epi8, _mm256_and_si256, _mm256_blendv_ps, _mm256_castps_si256,
    _mm256_castsi256_ps, _mm256_cmpgt_epi8, _mm256_loadu_si256, _mm256_maskload_epi32,
    _mm256_movemask_epi8, _mm256_or_si256, _mm256_permute2x128_si256, _mm256_set1_epi32,
    _mm256_set1_epi8, _mm256_set_epi32, _mm256_setr_epi8, _mm256_setzero_si256,
    _mm256_shuffle_epi8, _mm256_sllv_epi32, _mm256_srli_epi16, _mm256_subs_epu8,
    _mm256_testz_si256, _mm256_xor_si256, _mm_prefetch, _MM_HINT_T0,
};

use crate::implementation::helpers::Utf8CheckAlgorithm;

// AVX 2 SIMD primitives

type SimdU8Value = crate::implementation::helpers::SimdU8Value<__m256i>;

impl SimdU8Value {
    #[target_feature(enable = "avx2")]
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
        #[allow(clippy::cast_possible_wrap)]
        Self::from(_mm256_setr_epi8(
            v0 as i8, v1 as i8, v2 as i8, v3 as i8, v4 as i8, v5 as i8, v6 as i8, v7 as i8,
            v8 as i8, v9 as i8, v10 as i8, v11 as i8, v12 as i8, v13 as i8, v14 as i8, v15 as i8,
            v16 as i8, v17 as i8, v18 as i8, v19 as i8, v20 as i8, v21 as i8, v22 as i8, v23 as i8,
            v24 as i8, v25 as i8, v26 as i8, v27 as i8, v28 as i8, v29 as i8, v30 as i8, v31 as i8,
        ))
    }

    #[target_feature(enable = "avx2")]
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
        #[allow(clippy::cast_possible_wrap)]
        Self::from_32_cut_off_leading(
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v0, v1, v2, v3,
            v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15,
        )
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn load_from(ptr: *const u8) -> Self {
        #[allow(clippy::cast_ptr_alignment)]
        Self::from(_mm256_loadu_si256(ptr.cast::<__m256i>()))
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn vecmask_from_bitmask(mask: u8) -> Self {
        let vshift_count = _mm256_set_epi32(24, 25, 26, 27, 28, 29, 30, 31);
        let bcast = _mm256_set1_epi32(i32::from(mask));
        let shifted = _mm256_sllv_epi32(bcast, vshift_count); // high bit of each element = corresponding bit of the mask
        Self::from(shifted)
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn load_partial(ptr: *const u8, len: usize) -> Self {
        Self::load_partial_direct(ptr, len)
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn load_partial_direct(mut ptr: *const u8, len: usize) -> Self {
        if len == 0 {
            return Self::splat0();
        }
        let sel_mask = 1 << (len / 4);
        let mask = (sel_mask - 1) as u8;
        let mut res = _mm256_maskload_epi32(ptr.cast(), Self::vecmask_from_bitmask(mask).0);
        let remainder = len % 4;
        if remainder != 0 {
            ptr = ptr.add((len - len % 4) as usize);
            let remaining_bytes = match remainder {
                1 => u32::from(*ptr),
                2 => u32::from(*ptr) | u32::from(*ptr.add(1)) << 8,
                3 => u32::from(*ptr) | u32::from(*ptr.add(1)) << 8 | u32::from(*ptr.add(2)) << 16,
                _ => 0,
            };
            #[allow(clippy::cast_possible_wrap)]
            let remaining_vec = _mm256_set1_epi32(remaining_bytes as i32);
            res = _mm256_castps_si256(_mm256_blendv_ps(
                _mm256_castsi256_ps(res),
                _mm256_castsi256_ps(remaining_vec),
                _mm256_castsi256_ps(Self::vecmask_from_bitmask(sel_mask).0),
            ));
        }
        Self::from(res)
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn load_partial_copy(ptr: *const u8, len: usize) -> Self {
        let mut tmpbuf = [0_u8; 32];
        crate::implementation::helpers::memcpy_unaligned_nonoverlapping_inline_opt_lt_32(
            ptr,
            tmpbuf.as_mut_ptr(),
            len,
        );
        Self::load_from(tmpbuf.as_ptr())
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn lookup_16(
        self,
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
        Self::from(_mm256_shuffle_epi8(
            Self::repeat_16(
                v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15,
            )
            .0,
            self.0,
        ))
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn splat(val: u8) -> Self {
        #[allow(clippy::cast_possible_wrap)]
        Self::from(_mm256_set1_epi8(val as i8))
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn splat0() -> Self {
        Self::from(_mm256_setzero_si256())
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn or(self, b: Self) -> Self {
        Self::from(_mm256_or_si256(self.0, b.0))
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn and(self, b: Self) -> Self {
        Self::from(_mm256_and_si256(self.0, b.0))
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn xor(self, b: Self) -> Self {
        Self::from(_mm256_xor_si256(self.0, b.0))
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn saturating_sub(self, b: Self) -> Self {
        Self::from(_mm256_subs_epu8(self.0, b.0))
    }

    // ugly but shr<N> requires const generics
    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn shr4(self) -> Self {
        Self::from(_mm256_srli_epi16(self.0, 4)).and(Self::splat(0xFF >> 4))
    }

    // ugly but prev<N> requires const generics
    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn prev1(self, prev: Self) -> Self {
        Self::from(_mm256_alignr_epi8(
            self.0,
            _mm256_permute2x128_si256(prev.0, self.0, 0x21),
            16 - 1,
        ))
    }

    // ugly but prev<N> requires const generics
    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn prev2(self, prev: Self) -> Self {
        Self::from(_mm256_alignr_epi8(
            self.0,
            _mm256_permute2x128_si256(prev.0, self.0, 0x21),
            16 - 2,
        ))
    }

    // ugly but prev<N> requires const generics
    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn prev3(self, prev: Self) -> Self {
        Self::from(_mm256_alignr_epi8(
            self.0,
            _mm256_permute2x128_si256(prev.0, self.0, 0x21),
            16 - 3,
        ))
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn signed_gt(self, other: Self) -> Self {
        Self::from(_mm256_cmpgt_epi8(self.0, other.0))
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn any_bit_set(self) -> bool {
        _mm256_testz_si256(self.0, self.0) != 1
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn is_ascii(self) -> bool {
        _mm256_movemask_epi8(self.0) == 0
    }
}

impl From<__m256i> for SimdU8Value {
    #[inline]
    fn from(val: __m256i) -> Self {
        Self { 0: val }
    }
}

impl core::fmt::Display for SimdU8Value {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        unsafe {
            let arr: [u8; 32] = core::mem::transmute(self.0);
            write!(f, "{:?}", arr)
        }
    }
}

impl core::fmt::LowerHex for SimdU8Value {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        unsafe {
            let arr: [u8; 32] = core::mem::transmute(self.0);
            write!(f, "{:x?}", arr)
        }
    }
}

impl Utf8CheckAlgorithm<SimdU8Value> {
    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn must_be_2_3_continuation(prev2: SimdU8Value, prev3: SimdU8Value) -> SimdU8Value {
        let is_third_byte = prev2.saturating_sub(SimdU8Value::splat(0b1110_0000 - 1));
        let is_fourth_byte = prev3.saturating_sub(SimdU8Value::splat(0b1111_0000 - 1));

        is_third_byte
            .or(is_fourth_byte)
            .signed_gt(SimdU8Value::splat0())
    }
}

#[target_feature(enable = "avx2")]
#[inline]
unsafe fn simd_prefetch(ptr: *const u8) {
    _mm_prefetch(ptr.cast::<i8>(), _MM_HINT_T0);
}

mod test {
    #[cfg(not(features = "std"))]
    extern crate std;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    pub fn masked_load() {
        if std::is_x86_feature_detected!("avx2") {
            return;
        }

        let arr = [
            1_u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32,
        ];
        unsafe {
            for len in 0..32 {
                let loaded_arr: [u8; 32] =
                    core::mem::transmute(SimdU8Value::load_partial(arr.as_ptr(), len));
                for i in 0..len {
                    assert_eq!(arr[i], loaded_arr[i]);
                }
                for x in &loaded_arr[len..arr.len()] {
                    assert_eq!(*x, 0);
                }
            }
        }
    }
}

const PREFETCH: bool = true;
const PREVENT_REMAINDER_LOOP_UNROLLING: bool = false;
#[allow(unused_imports)]
use crate::implementation::helpers::TempSimdChunkA32 as TempSimdChunk;
simd_input_256_bit!("avx2");
algorithm_simd!("avx2");
