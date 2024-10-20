//! Contains the aarch64 UTF-8 validation implementation.

use core::arch::arm::{
    uint8x16_t, uint8x8x2_t, vandq_u8, vcgtq_u8, vcombine_u8, vdupq_n_u8, veorq_u8, vextq_u8,
    vget_high_u8, vget_lane_u8, vget_low_u8, vld1q_u8, vmovq_n_u8, vorrq_u8, vpmax_u8, vqsubq_u8,
    vshrq_n_u8, vtbl2_u8,
};

use crate::implementation::helpers::Utf8CheckAlgorithm;

// armv7 NEON SIMD primitives

#[inline]
#[target_feature(enable = "neon")]
unsafe fn vqtbl1q_u8(lut: uint8x16_t, idx: uint8x16_t) -> uint8x16_t {
    let lut = uint8x8x2_t(vget_low_u8(lut), vget_high_u8(lut));
    let lo = vtbl2_u8(lut, vget_low_u8(idx));
    let hi = vtbl2_u8(lut, vget_high_u8(idx));
    vcombine_u8(lo, hi)
}

#[inline]
#[target_feature(enable = "neon")]
unsafe fn vmaxvq_u8(v: uint8x16_t) -> u8 {
    let max = vpmax_u8(vget_low_u8(v), vget_high_u8(v));
    let max = vpmax_u8(max, max);
    let max = vpmax_u8(max, max);
    let max = vpmax_u8(max, max);
    vget_lane_u8(max, 0)
}

type SimdU8Value = crate::implementation::helpers::SimdU8Value<uint8x16_t>;

impl SimdU8Value {
    #[inline]
    #[target_feature(enable = "neon")]
    #[flexpect::e(clippy::too_many_arguments)]
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
        let arr: [u8; 16] = [
            v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31,
        ];
        Self(vld1q_u8(arr.as_ptr()))
    }

    #[inline]
    #[target_feature(enable = "neon")]
    #[flexpect::e(clippy::too_many_arguments)]
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
        let arr: [u8; 16] = [
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15,
        ];
        Self(vld1q_u8(arr.as_ptr()))
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn load_from(ptr: *const u8) -> Self {
        // WORKAROUND for https://github.com/rust-lang/stdarch/issues/1148
        // The vld1q_u8 intrinsic is currently broken, it treats it as individual
        // byte loads so the compiler sometimes decides it is a better to load
        // individual bytes to "optimize" a subsequent SIMD shuffle
        //
        // This code forces a full 128-bit load.
        let mut dst = core::mem::MaybeUninit::<uint8x16_t>::uninit();
        core::ptr::copy_nonoverlapping(
            ptr.cast::<u8>(),
            dst.as_mut_ptr().cast::<u8>(),
            core::mem::size_of::<uint8x16_t>(),
        );
        Self(dst.assume_init())
    }

    #[inline]
    #[target_feature(enable = "neon")]
    #[flexpect::e(clippy::too_many_arguments)]
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
        let rep = Self::repeat_16(
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15,
        )
        .0;
        Self(vqtbl1q_u8(rep, self.0))
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn splat(val: u8) -> Self {
        Self(vmovq_n_u8(val))
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn splat0() -> Self {
        Self(vdupq_n_u8(0))
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn or(self, b: Self) -> Self {
        Self(vorrq_u8(self.0, b.0))
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn and(self, b: Self) -> Self {
        Self(vandq_u8(self.0, b.0))
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn xor(self, b: Self) -> Self {
        Self(veorq_u8(self.0, b.0))
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn saturating_sub(self, b: Self) -> Self {
        Self(vqsubq_u8(self.0, b.0))
    }

    // ugly but shr<N> requires const generics

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn shr4(self) -> Self {
        Self(vshrq_n_u8(self.0, 4))
    }

    // ugly but prev<N> requires const generics

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn prev1(self, prev: Self) -> Self {
        Self(vextq_u8(prev.0, self.0, 16 - 1))
    }

    // ugly but prev<N> requires const generics

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn prev2(self, prev: Self) -> Self {
        Self(vextq_u8(prev.0, self.0, 16 - 2))
    }

    // ugly but prev<N> requires const generics

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn prev3(self, prev: Self) -> Self {
        Self(vextq_u8(prev.0, self.0, 16 - 3))
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn unsigned_gt(self, other: Self) -> Self {
        Self(vcgtq_u8(self.0, other.0))
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn any_bit_set(self) -> bool {
        vmaxvq_u8(self.0) != 0
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn is_ascii(self) -> bool {
        vmaxvq_u8(self.0) > 0b1000_0000_u8
    }
}

impl From<uint8x16_t> for SimdU8Value {
    #[inline]
    fn from(val: uint8x16_t) -> Self {
        Self(val)
    }
}

impl Utf8CheckAlgorithm<SimdU8Value> {
    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn must_be_2_3_continuation(prev2: SimdU8Value, prev3: SimdU8Value) -> SimdU8Value {
        let is_third_byte = prev2.unsigned_gt(SimdU8Value::splat(0b1110_0000 - 1));
        let is_fourth_byte = prev3.unsigned_gt(SimdU8Value::splat(0b1111_0000 - 1));

        is_third_byte.or(is_fourth_byte)
    }
}

#[inline]
unsafe fn simd_prefetch(_ptr: *const u8) {}

const PREFETCH: bool = false;
use crate::implementation::helpers::TempSimdChunkA16 as TempSimdChunk;
simd_input_128_bit!(#[target_feature(enable = "neon")]);
algorithm_simd!(#[target_feature(enable = "neon")]);
