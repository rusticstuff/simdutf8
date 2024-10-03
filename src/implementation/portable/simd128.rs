//! Contains the portable SIMD UTF-8 validation implementation.

#![allow(clippy::too_many_arguments)]
#![allow(clippy::needless_pass_by_value, clippy::pedantic, clippy::all)]
use crate::implementation::helpers::Utf8CheckAlgorithm;
use core::simd::prelude::*;
use core::simd::{simd_swizzle, u8x16};

// Portable SIMD primitives
type SimdU8Value = crate::implementation::helpers::SimdU8Value<u8x16>;

impl SimdU8Value {
    // ist OK
    #[inline]
    fn from_32_cut_off_leading(
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
        Self::from(u8x16::from_array([
            v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31,
        ]))
    }

    // ist OK
    #[inline]
    fn repeat_16(
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
        Self::from(u8x16::from_array([
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15,
        ]))
    }

    #[inline]
    unsafe fn load_from(ptr: *const u8) -> Self {
        Self::from(ptr.cast::<u8x16>().read_unaligned())
    }

    // ist OK
    #[inline]
    fn lookup_16(
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
        // We need to ensure that 'self' only contains the lower 4 bits, unlike the avx instruction
        // this will otherwise lead to bad results
        let idx: u8x16 = self.0;
        let src: u8x16 = Self::repeat_16(
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15,
        )
        .0;
        let res = src.swizzle_dyn(idx);
        Self::from(res)
    }
    // ist OK
    #[inline]
    fn splat(val: u8) -> Self {
        #[allow(clippy::cast_possible_wrap)]
        Self::from(u8x16::splat(val))
    }
    // ist OK
    #[inline]
    fn splat0() -> Self {
        Self::from(u8x16::splat(0))
    }

    #[inline]
    fn or(self, b: Self) -> Self {
        Self::from(self.0 | b.0)
    }

    #[inline]
    fn and(self, b: Self) -> Self {
        Self::from(self.0 & b.0)
    }

    #[inline]
    fn xor(self, b: Self) -> Self {
        Self::from(self.0 ^ b.0)
    }

    #[inline]
    fn saturating_sub(self, b: Self) -> Self {
        Self::from(self.0.saturating_sub(b.0))
    }
    // ist OK
    // ugly but shr<N> requires const generics
    #[inline]
    fn shr4(self) -> Self {
        Self::from(self.0 >> 4)
    }
    // ist OK
    #[inline]
    fn prev1(self, prev: Self) -> Self {
        Self::from(simd_swizzle!(
            self.0,
            prev.0,
            [31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,]
        ))
    }

    // ist OK
    // ugly but prev<N> requires const generics
    #[inline]
    fn prev2(self, prev: Self) -> Self {
        Self::from(simd_swizzle!(
            self.0,
            prev.0,
            [30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,]
        ))
    }

    // ist OK
    // ugly but prev<N> requires const generics
    #[inline]
    fn prev3(self, prev: Self) -> Self {
        Self::from(simd_swizzle!(
            self.0,
            prev.0,
            [29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,]
        ))
    }

    #[inline]
    fn unsigned_gt(self, other: Self) -> Self {
        let gt = self.0.simd_gt(other.0).to_int();
        Self::from(gt.cast())
    }

    #[inline]
    fn any_bit_set(self) -> bool {
        self.0 != u8x16::splat(0)
    }

    #[inline]
    fn is_ascii(self) -> bool {
        if HAS_FAST_REDUCE_MAX {
            self.0.reduce_max() < 0b1000_0000
        } else {
            (self.0 & u8x16::splat(0b1000_0000)) == u8x16::splat(0)
        }
    }
}

impl From<u8x16> for SimdU8Value {
    #[inline]
    fn from(val: u8x16) -> Self {
        Self(val)
    }
}

impl Utf8CheckAlgorithm<SimdU8Value> {
    #[inline]
    fn must_be_2_3_continuation(prev2: SimdU8Value, prev3: SimdU8Value) -> SimdU8Value {
        let is_third_byte = prev2.saturating_sub(SimdU8Value::splat(0b1110_0000 - 1));
        let is_fourth_byte = prev3.saturating_sub(SimdU8Value::splat(0b1111_0000 - 1));

        is_third_byte
            .or(is_fourth_byte)
            .unsigned_gt(SimdU8Value::splat0())
    }
}

#[inline]
unsafe fn simd_prefetch(_ptr: *const u8) {}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
const HAS_FAST_REDUCE_MAX: bool = true;

#[cfg(not(all(target_arch = "aarch64", target_feature = "neon")))]
const HAS_FAST_REDUCE_MAX: bool = false;

const PREFETCH: bool = false;
use crate::implementation::helpers::TempSimdChunkA16 as TempSimdChunk;
simd_input_128_bit!();
algorithm_simd!();
