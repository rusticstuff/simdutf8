//! Contains the loongarch64 LASX UTF-8 validation implementation.

#[cfg(target_arch = "loongarch64")]
use core::arch::loongarch64::{
    lasx_xvand_v, lasx_xvld, lasx_xvldi, lasx_xvmskltz_b, lasx_xvmsknz_b, lasx_xvor_v,
    lasx_xvpermi_q, lasx_xvpickve2gr_d, lasx_xvreplgr2vr_b, lasx_xvshuf_b, lasx_xvsrli_b,
    lasx_xvssub_bu, lasx_xvxor_v, m256i,
};

use crate::implementation::helpers::Utf8CheckAlgorithm;

// LASX SIMD primitives

type SimdU8Value = crate::implementation::helpers::SimdU8Value<m256i>;

impl SimdU8Value {
    #[flexpect::e(clippy::too_many_arguments)]
    #[flexpect::e(clippy::cast_possible_wrap)]
    #[target_feature(enable = "lasx")]
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
        let arr: [u8; 32] = [
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18,
            v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31,
        ];
        Self::from(lasx_xvld::<0>(arr.as_ptr().cast()))
    }

    #[flexpect::e(clippy::too_many_arguments)]
    #[target_feature(enable = "lasx")]
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
        Self::from_32_cut_off_leading(
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v0, v1, v2, v3,
            v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15,
        )
    }

    #[flexpect::e(clippy::cast_ptr_alignment)]
    #[target_feature(enable = "lasx")]
    #[inline]
    unsafe fn load_from(ptr: *const u8) -> Self {
        Self::from(lasx_xvld::<0>(ptr.cast()))
    }

    #[flexpect::e(clippy::too_many_arguments)]
    #[target_feature(enable = "lasx")]
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

        Self::from(lasx_xvshuf_b(src, src, self.0))
    }

    #[flexpect::e(clippy::cast_possible_wrap)]
    #[target_feature(enable = "lasx")]
    #[inline]
    unsafe fn splat(val: u8) -> Self {
        Self::from(lasx_xvreplgr2vr_b(val as i32))
    }

    #[target_feature(enable = "lasx")]
    #[inline]
    unsafe fn splat0() -> Self {
        Self::from(lasx_xvldi::<0>())
    }

    #[target_feature(enable = "lasx")]
    #[inline]
    unsafe fn or(self, b: Self) -> Self {
        Self::from(lasx_xvor_v(self.0, b.0))
    }

    #[target_feature(enable = "lasx")]
    #[inline]
    unsafe fn and(self, b: Self) -> Self {
        Self::from(lasx_xvand_v(self.0, b.0))
    }

    #[target_feature(enable = "lasx")]
    #[inline]
    unsafe fn xor(self, b: Self) -> Self {
        Self::from(lasx_xvxor_v(self.0, b.0))
    }

    #[target_feature(enable = "lasx")]
    #[inline]
    unsafe fn saturating_sub(self, b: Self) -> Self {
        Self::from(lasx_xvssub_bu(self.0, b.0))
    }

    // ugly but shr<N> requires const generics
    #[target_feature(enable = "lasx")]
    #[inline]
    unsafe fn shr4(self) -> Self {
        Self::from(lasx_xvsrli_b::<4>(self.0))
    }

    // ugly but prev<N> requires const generics
    #[target_feature(enable = "lasx")]
    #[inline]
    unsafe fn prev1(self, prev: Self) -> Self {
        // This lets us end up with [ prev_hi | self_lo ]
        let bridge = lasx_xvpermi_q(self.0, prev.0, 0x21);
        // It shuffles [ b_lo | a_lo ] | [ b_hi | a_hi ]
        // ...aka [ bridge_lo | self_lo ] | [ bridge_hi | self_hi ]
        // ...aka [ prev_hi | self_lo ] | [ self_lo | self_hi ]
        let mask = [
            15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 15, 16, 17, 18, 19, 20,
            21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
        ];
        Self::from(lasx_xvshuf_b(self.0, bridge, lasx_xvld::<0>(mask.as_ptr())))
    }

    // ugly but prev<N> requires const generics
    #[target_feature(enable = "lasx")]
    #[inline]
    unsafe fn prev2(self, prev: Self) -> Self {
        let bridge = lasx_xvpermi_q(self.0, prev.0, 0x21);
        let mask = [
            14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 14, 15, 16, 17, 18, 19,
            20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
        ];
        Self::from(lasx_xvshuf_b(self.0, bridge, lasx_xvld::<0>(mask.as_ptr())))
    }

    // ugly but prev<N> requires const generics
    #[target_feature(enable = "lasx")]
    #[inline]
    unsafe fn prev3(self, prev: Self) -> Self {
        let bridge = lasx_xvpermi_q(self.0, prev.0, 0x21);
        let mask = [
            13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 13, 14, 15, 16, 17, 18,
            19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
        ];
        Self::from(lasx_xvshuf_b(self.0, bridge, lasx_xvld::<0>(mask.as_ptr())))
    }

    #[target_feature(enable = "lasx")]
    #[inline]
    unsafe fn any_bit_set(self) -> bool {
        let nonzero_mask = lasx_xvmsknz_b(self.0);
        let lo = lasx_xvpickve2gr_d::<0>(nonzero_mask);
        let hi = lasx_xvpickve2gr_d::<2>(nonzero_mask);
        lo != 0 || hi != 0
    }

    #[target_feature(enable = "lasx")]
    #[inline]
    unsafe fn is_ascii(self) -> bool {
        let high_bits = lasx_xvmskltz_b(self.0);
        let lo = lasx_xvpickve2gr_d::<0>(high_bits);
        let hi = lasx_xvpickve2gr_d::<2>(high_bits);
        (lo | hi) == 0
    }
}

impl From<m256i> for SimdU8Value {
    #[inline]
    fn from(val: m256i) -> Self {
        Self(val)
    }
}

impl Utf8CheckAlgorithm<SimdU8Value> {
    #[target_feature(enable = "lasx")]
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
use crate::implementation::helpers::TempSimdChunkA32 as TempSimdChunk;
simd_input_256_bit!(#[target_feature(enable = "lasx")]);
algorithm_simd!(#[target_feature(enable = "lasx")]);
