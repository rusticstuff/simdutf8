//! Contains the loongarch64 LSX UTF-8 validation implementation.

#[cfg(target_arch = "loongarch64")]
use core::arch::loongarch64::{
    lsx_vand_v, lsx_vld, lsx_vldi, lsx_vmskltz_b, lsx_vmsknz_b, lsx_vor_v, lsx_vpickve2gr_w,
    lsx_vreplgr2vr_b, lsx_vshuf_b, lsx_vsrli_b, lsx_vssub_bu, lsx_vxor_v, m128i,
};

use crate::implementation::helpers::Utf8CheckAlgorithm;

// LSX SIMD primitives

type SimdU8Value = crate::implementation::helpers::SimdU8Value<m128i>;

impl SimdU8Value {
    #[flexpect::e(clippy::too_many_arguments)]
    #[flexpect::e(clippy::cast_possible_wrap)]
    #[target_feature(enable = "lsx")]
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
        let arr: [u8; 16] = [
            v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31,
        ];
        Self::from(lsx_vld::<0>(arr.as_ptr().cast()))
    }

    #[flexpect::e(clippy::too_many_arguments)]
    #[flexpect::e(clippy::cast_possible_wrap)]
    #[target_feature(enable = "lsx")]
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
        let arr: [u8; 16] = [
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15,
        ];
        Self::from(lsx_vld::<0>(arr.as_ptr().cast()))
    }

    #[flexpect::e(clippy::cast_ptr_alignment)]
    #[target_feature(enable = "lsx")]
    #[inline]
    unsafe fn load_from(ptr: *const u8) -> Self {
        Self::from(lsx_vld::<0>(ptr.cast()))
    }

    #[flexpect::e(clippy::too_many_arguments)]
    #[target_feature(enable = "lsx")]
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
        let src = Self::repeat_16(
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15,
        )
        .0;

        Self::from(lsx_vshuf_b(src, src, self.0))
    }

    #[flexpect::e(clippy::cast_possible_wrap)]
    #[target_feature(enable = "lsx")]
    #[inline]
    unsafe fn splat(val: u8) -> Self {
        Self::from(lsx_vreplgr2vr_b(val as i32))
    }

    #[target_feature(enable = "lsx")]
    #[inline]
    unsafe fn splat0() -> Self {
        Self::from(lsx_vldi::<0>())
    }

    #[target_feature(enable = "lsx")]
    #[inline]
    unsafe fn or(self, b: Self) -> Self {
        Self::from(lsx_vor_v(self.0, b.0))
    }

    #[target_feature(enable = "lsx")]
    #[inline]
    unsafe fn and(self, b: Self) -> Self {
        Self::from(lsx_vand_v(self.0, b.0))
    }

    #[target_feature(enable = "lsx")]
    #[inline]
    unsafe fn xor(self, b: Self) -> Self {
        Self::from(lsx_vxor_v(self.0, b.0))
    }

    #[target_feature(enable = "lsx")]
    #[inline]
    unsafe fn saturating_sub(self, b: Self) -> Self {
        Self::from(lsx_vssub_bu(self.0, b.0))
    }

    // ugly but shr<N> requires const generics
    #[target_feature(enable = "lsx")]
    #[inline]
    unsafe fn shr4(self) -> Self {
        Self::from(lsx_vsrli_b::<4>(self.0))
    }

    // ugly but prev<N> requires const generics
    #[target_feature(enable = "lsx")]
    #[inline]
    unsafe fn prev1(self, prev: Self) -> Self {
        let ctrl_arr: [u8; 16] = [31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];

        Self::from(lsx_vshuf_b(
            prev.0,
            self.0,
            lsx_vld::<0>(ctrl_arr.as_ptr().cast()),
        ))
    }

    // ugly but prev<N> requires const generics
    #[target_feature(enable = "lsx")]
    #[inline]
    unsafe fn prev2(self, prev: Self) -> Self {
        let ctrl_arr: [u8; 16] = [30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

        Self::from(lsx_vshuf_b(
            prev.0,
            self.0,
            lsx_vld::<0>(ctrl_arr.as_ptr().cast()),
        ))
    }

    // ugly but prev<N> requires const generics
    #[target_feature(enable = "lsx")]
    #[inline]
    unsafe fn prev3(self, prev: Self) -> Self {
        let ctrl_arr: [u8; 16] = [29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

        Self::from(lsx_vshuf_b(
            prev.0,
            self.0,
            lsx_vld::<0>(ctrl_arr.as_ptr().cast()),
        ))
    }

    #[target_feature(enable = "lsx")]
    #[inline]
    unsafe fn any_bit_set(self) -> bool {
        lsx_vpickve2gr_w::<0>(lsx_vmsknz_b(self.0)) != 0
    }

    #[target_feature(enable = "lsx")]
    #[inline]
    unsafe fn is_ascii(self) -> bool {
        lsx_vpickve2gr_w::<0>(lsx_vmskltz_b(self.0)) == 0
    }
}

impl From<m128i> for SimdU8Value {
    #[inline]
    fn from(val: m128i) -> Self {
        Self(val)
    }
}

impl Utf8CheckAlgorithm<SimdU8Value> {
    #[target_feature(enable = "lsx")]
    #[inline]
    unsafe fn must_be_2_3_continuation(prev2: SimdU8Value, prev3: SimdU8Value) -> SimdU8Value {
        let is_third_byte = prev2.saturating_sub(SimdU8Value::splat(0xe0 - 0x80));
        let is_fourth_byte = prev3.saturating_sub(SimdU8Value::splat(0xf0 - 0x80));
        is_third_byte.or(is_fourth_byte)
    }
}

#[inline]
unsafe fn simd_prefetch(_ptr: *const u8) {}

const PREFETCH: bool = false;
use crate::implementation::helpers::TempSimdChunkA16 as TempSimdChunk;
simd_input_128_bit!(#[target_feature(enable = "lsx")]);
algorithm_simd!(#[target_feature(enable = "lsx")]);
