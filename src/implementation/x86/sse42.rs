//! Contains the x86-64/x86 SSE4.2 UTF-8 validation implementation.

#![allow(clippy::too_many_arguments)]

#[cfg(target_arch = "x86")]
use core::arch::x86::{
    __m128i, _mm_alignr_epi8, _mm_and_si128, _mm_cmpgt_epi8, _mm_insert_epi16, _mm_insert_epi8,
    _mm_loadu_si128, _mm_movemask_epi8, _mm_or_si128, _mm_prefetch, _mm_set1_epi8, _mm_setr_epi16,
    _mm_setr_epi32, _mm_setr_epi8, _mm_setzero_si128, _mm_shuffle_epi8, _mm_srli_epi16,
    _mm_subs_epu8, _mm_testz_si128, _mm_xor_si128, _MM_HINT_T0,
};
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
    __m128i, _mm_alignr_epi8, _mm_and_si128, _mm_cmpgt_epi8, _mm_insert_epi16, _mm_insert_epi8,
    _mm_loadu_si128, _mm_movemask_epi8, _mm_or_si128, _mm_prefetch, _mm_set1_epi8, _mm_setr_epi16,
    _mm_setr_epi32, _mm_setr_epi8, _mm_setzero_si128, _mm_shuffle_epi8, _mm_srli_epi16,
    _mm_subs_epu8, _mm_testz_si128, _mm_xor_si128, _MM_HINT_T0,
};

use crate::implementation::helpers::Utf8CheckAlgorithm;

// SSE 4.2 SIMD primitives

type SimdU8Value = crate::implementation::helpers::SimdU8Value<__m128i>;

impl SimdU8Value {
    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn from_32_cut_off_leading(
        _v0: u8,
        _v1: u8,
        _v2: u8,
        _v3: u8,
        _v4: u8,
        _v5: u8,
        _v6: u8,
        _v7: u8,
        _v8: u8,
        _v9: u8,
        _v10: u8,
        _v11: u8,
        _v12: u8,
        _v13: u8,
        _v14: u8,
        _v15: u8,
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
        Self::from(_mm_setr_epi8(
            v16 as i8, v17 as i8, v18 as i8, v19 as i8, v20 as i8, v21 as i8, v22 as i8, v23 as i8,
            v24 as i8, v25 as i8, v26 as i8, v27 as i8, v28 as i8, v29 as i8, v30 as i8, v31 as i8,
        ))
    }

    #[target_feature(enable = "sse4.2")]
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
        Self::from(_mm_setr_epi8(
            v0 as i8, v1 as i8, v2 as i8, v3 as i8, v4 as i8, v5 as i8, v6 as i8, v7 as i8,
            v8 as i8, v9 as i8, v10 as i8, v11 as i8, v12 as i8, v13 as i8, v14 as i8, v15 as i8,
        ))
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn load_from(ptr: *const u8) -> Self {
        #[allow(clippy::cast_ptr_alignment)]
        Self::from(_mm_loadu_si128(ptr.cast::<__m128i>()))
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn load_partial(ptr: *const u8, len: usize) -> Self {
        Self::from(match len {
            1 => _mm_setr_epi8(
                ptr.cast::<i8>().read_unaligned(),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ),
            2 => _mm_setr_epi16(ptr.cast::<i16>().read_unaligned(), 0, 0, 0, 0, 0, 0, 0),
            3 => _mm_setr_epi8(
                ptr.cast::<i8>().read_unaligned(),
                ptr.add(1).cast::<i8>().read_unaligned(),
                ptr.add(2).cast::<i8>().read_unaligned(),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ),
            4 => _mm_setr_epi32(ptr.cast::<i32>().read_unaligned(), 0, 0, 0), // assembly???
            5 => {
                let val = _mm_setr_epi32(ptr.cast::<i32>().read_unaligned(), 0, 0, 0);
                _mm_insert_epi8(val, i32::from(ptr.add(4).cast::<i8>().read_unaligned()), 4)
            }
            6 => _mm_setr_epi16(
                ptr.cast::<i16>().read_unaligned(),
                ptr.add(2).cast::<i16>().read_unaligned(),
                ptr.add(4).cast::<i16>().read_unaligned(),
                0,
                0,
                0,
                0,
                0,
            ),
            7 => {
                let val = _mm_setr_epi16(
                    ptr.cast::<i16>().read_unaligned(),
                    ptr.add(2).cast::<i16>().read_unaligned(),
                    ptr.add(4).cast::<i16>().read_unaligned(),
                    0,
                    0,
                    0,
                    0,
                    0,
                );
                _mm_insert_epi8(val, i32::from(ptr.add(6).cast::<i8>().read_unaligned()), 6)
            }
            8 => _mm_setr_epi32(
                ptr.cast::<i32>().read_unaligned(),
                ptr.add(4).cast::<i32>().read_unaligned(),
                0,
                0,
            ),
            9 => {
                let val = _mm_setr_epi32(
                    ptr.cast::<i32>().read_unaligned(),
                    ptr.add(4).cast::<i32>().read_unaligned(),
                    0,
                    0,
                );
                _mm_insert_epi8(val, i32::from(ptr.add(8).cast::<i8>().read_unaligned()), 8)
            }
            10 => {
                let val = _mm_setr_epi32(
                    ptr.cast::<i32>().read_unaligned(),
                    ptr.add(4).cast::<i32>().read_unaligned(),
                    0,
                    0,
                );
                _mm_insert_epi16(val, i32::from(ptr.add(8).cast::<i16>().read_unaligned()), 4)
            }
            11 => {
                let mut val = _mm_setr_epi32(
                    ptr.cast::<i32>().read_unaligned(),
                    ptr.add(4).cast::<i32>().read_unaligned(),
                    0,
                    0,
                );
                val =
                    _mm_insert_epi16(val, i32::from(ptr.add(8).cast::<i16>().read_unaligned()), 4);
                _mm_insert_epi8(
                    val,
                    i32::from(ptr.add(10).cast::<i8>().read_unaligned()),
                    10,
                )
            }
            12 => _mm_setr_epi32(
                ptr.cast::<i32>().read_unaligned(),
                ptr.add(4).cast::<i32>().read_unaligned(),
                ptr.add(8).cast::<i32>().read_unaligned(),
                0,
            ),
            13 => {
                let val = _mm_setr_epi32(
                    ptr.cast::<i32>().read_unaligned(),
                    ptr.add(4).cast::<i32>().read_unaligned(),
                    ptr.add(8).cast::<i32>().read_unaligned(),
                    0,
                );
                _mm_insert_epi8(
                    val,
                    i32::from(ptr.add(12).cast::<i8>().read_unaligned()),
                    12,
                )
            }
            14 => {
                let val = _mm_setr_epi32(
                    ptr.cast::<i32>().read_unaligned(),
                    ptr.add(4).cast::<i32>().read_unaligned(),
                    ptr.add(8).cast::<i32>().read_unaligned(),
                    0,
                );
                _mm_insert_epi16(
                    val,
                    i32::from(ptr.add(12).cast::<i16>().read_unaligned()),
                    6,
                )
            }
            15 => {
                let mut val = _mm_setr_epi32(
                    ptr.cast::<i32>().read_unaligned(),
                    ptr.add(4).cast::<i32>().read_unaligned(),
                    ptr.add(8).cast::<i32>().read_unaligned(),
                    0,
                );
                val = _mm_insert_epi16(
                    val,
                    i32::from(ptr.add(12).cast::<i16>().read_unaligned()),
                    6,
                );
                _mm_insert_epi8(
                    val,
                    i32::from(ptr.add(14).cast::<i8>().read_unaligned()),
                    14,
                )
            }
            _ => Self::splat0().0,
        })
    }

    unsafe fn load_partial_copy(ptr: *const u8, len: usize) -> Self {
        let mut tmpbuf = [0_u8; 16];
        crate::implementation::helpers::memcpy_unaligned_nonoverlapping_inline_opt_lt_16(
            ptr,
            tmpbuf.as_mut_ptr(),
            len,
        );
        Self::load_from(tmpbuf.as_ptr())
    }

    #[target_feature(enable = "sse4.2")]
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
        Self::from(_mm_shuffle_epi8(
            Self::repeat_16(
                v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15,
            )
            .0,
            self.0,
        ))
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn splat(val: u8) -> Self {
        #[allow(clippy::cast_possible_wrap)]
        Self::from(_mm_set1_epi8(val as i8))
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn splat0() -> Self {
        Self::from(_mm_setzero_si128())
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn or(self, b: Self) -> Self {
        Self::from(_mm_or_si128(self.0, b.0))
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn and(self, b: Self) -> Self {
        Self::from(_mm_and_si128(self.0, b.0))
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn xor(self, b: Self) -> Self {
        Self::from(_mm_xor_si128(self.0, b.0))
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn saturating_sub(self, b: Self) -> Self {
        Self::from(_mm_subs_epu8(self.0, b.0))
    }

    // ugly but shr<N> requires const generics
    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn shr4(self) -> Self {
        Self::from(_mm_srli_epi16(self.0, 4)).and(Self::splat(0xFF >> 4))
    }

    // ugly but prev<N> requires const generics
    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn prev1(self, prev: Self) -> Self {
        Self::from(_mm_alignr_epi8(self.0, prev.0, 16 - 1))
    }

    // ugly but prev<N> requires const generics
    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn prev2(self, prev: Self) -> Self {
        Self::from(_mm_alignr_epi8(self.0, prev.0, 16 - 2))
    }

    // ugly but prev<N> requires const generics
    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn prev3(self, prev: Self) -> Self {
        Self::from(_mm_alignr_epi8(self.0, prev.0, 16 - 3))
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn signed_gt(self, other: Self) -> Self {
        Self::from(_mm_cmpgt_epi8(self.0, other.0))
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn any_bit_set(self) -> bool {
        _mm_testz_si128(self.0, self.0) != 1
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn is_ascii(self) -> bool {
        _mm_movemask_epi8(self.0) == 0
    }
}

impl From<__m128i> for SimdU8Value {
    #[inline]
    fn from(val: __m128i) -> Self {
        Self { 0: val }
    }
}

impl Utf8CheckAlgorithm<SimdU8Value> {
    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn must_be_2_3_continuation(prev2: SimdU8Value, prev3: SimdU8Value) -> SimdU8Value {
        let is_third_byte = prev2.saturating_sub(SimdU8Value::splat(0b1110_0000 - 1));
        let is_fourth_byte = prev3.saturating_sub(SimdU8Value::splat(0b1111_0000 - 1));

        is_third_byte
            .or(is_fourth_byte)
            .signed_gt(SimdU8Value::splat0())
    }
}

#[target_feature(enable = "sse4.2")]
#[inline]
unsafe fn simd_prefetch(ptr: *const u8) {
    _mm_prefetch(ptr.cast::<i8>(), _MM_HINT_T0);
}

#[cfg(test)]
mod test {
    #[cfg(not(features = "std"))]
    extern crate std;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    pub fn masked_load() {
        if !std::is_x86_feature_detected!("sse4.2") {
            return;
        }

        let arr = [1_u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        unsafe {
            for len in 0..16 {
                let loaded_arr: [u8; 16] =
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

const PREFETCH: bool = false;
const PREVENT_REMAINDER_LOOP_UNROLLING: bool = true;
#[allow(unused_imports)]
use crate::implementation::helpers::TempSimdChunkA16 as TempSimdChunk;
simd_input_128_bit!("sse4.2");
algorithm_simd!("sse4.2");
