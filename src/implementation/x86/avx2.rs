//! Contains the x86-64/x86 AVX2 UTF-8 validation implementation.
//!
//! See Validating UTF-8 In Less Than One Instruction Per Byte, Software: Practice and Experience 51 (5), 2021
//! <https://arxiv.org/abs/2010.03090>
#[cfg(target_arch = "x86")]
use core::arch::x86::{
    __m256i, _mm256_alignr_epi8, _mm256_and_si256, _mm256_cmpgt_epi8, _mm256_loadu_si256,
    _mm256_movemask_epi8, _mm256_or_si256, _mm256_permute2x128_si256, _mm256_set1_epi8,
    _mm256_setr_epi8, _mm256_setzero_si256, _mm256_shuffle_epi8, _mm256_srli_epi16,
    _mm256_subs_epu8, _mm256_testz_si256, _mm256_xor_si256,
};
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
    __m256i, _mm256_alignr_epi8, _mm256_and_si256, _mm256_cmpgt_epi8, _mm256_loadu_si256,
    _mm256_movemask_epi8, _mm256_or_si256, _mm256_permute2x128_si256, _mm256_set1_epi8,
    _mm256_setr_epi8, _mm256_setzero_si256, _mm256_shuffle_epi8, _mm256_srli_epi16,
    _mm256_subs_epu8, _mm256_testz_si256, _mm256_xor_si256,
};

use crate::implementation::algorithm::Utf8CheckingState;

type SimdU8Value = crate::implementation::algorithm::SimdU8Value<__m256i>;

impl SimdU8Value {
    #[target_feature(enable = "avx2")]
    #[inline]
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::cast_possible_wrap)]
    unsafe fn from_32_align_end(
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
        Self::from(_mm256_setr_epi8(
            v0 as i8, v1 as i8, v2 as i8, v3 as i8, v4 as i8, v5 as i8, v6 as i8, v7 as i8,
            v8 as i8, v9 as i8, v10 as i8, v11 as i8, v12 as i8, v13 as i8, v14 as i8, v15 as i8,
            v16 as i8, v17 as i8, v18 as i8, v19 as i8, v20 as i8, v21 as i8, v22 as i8, v23 as i8,
            v24 as i8, v25 as i8, v26 as i8, v27 as i8, v28 as i8, v29 as i8, v30 as i8, v31 as i8,
        ))
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::cast_possible_wrap)]
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
        Self::from_32_align_end(
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v0, v1, v2, v3,
            v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15,
        )
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    #[allow(clippy::too_many_arguments)]
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
    #[allow(clippy::cast_possible_wrap)]
    unsafe fn broadcast(val: u8) -> Self {
        Self::from(_mm256_set1_epi8(val as i8))
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    #[allow(clippy::cast_possible_wrap)]
    unsafe fn broadcast0() -> Self {
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

    #[target_feature(enable = "avx2")]
    #[allow(clippy::cast_lossless)]
    #[inline]
    unsafe fn shr(self, val: u8) -> Self {
        Self::from(_mm256_srli_epi16(self.0, val as i32)).and(Self::broadcast(0xFF_u8 >> val))
    }

    // ugly but prev<N> requires const generics
    #[target_feature(enable = "avx2")]
    #[allow(clippy::cast_lossless)]
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
    #[allow(clippy::cast_lossless)]
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
    #[allow(clippy::cast_lossless)]
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
    unsafe fn gt(self, other: Self) -> Self {
        Self::from(_mm256_cmpgt_epi8(self.0, other.0))
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn any_bit_set(self) -> bool {
        _mm256_testz_si256(self.0, self.0) != 1
    }
}

impl From<__m256i> for SimdU8Value {
    #[inline]
    fn from(val: __m256i) -> Self {
        Self { 0: val }
    }
}

impl Utf8CheckingState<__m256i> {
    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn default() -> Self {
        Self {
            prev: SimdU8Value::broadcast0().0,
            incomplete: SimdU8Value::broadcast0().0,
            error: SimdU8Value::broadcast0().0,
        }
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn or(a: __m256i, b: __m256i) -> __m256i {
        SimdU8Value::from(a).or(SimdU8Value::from(b)).0
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn check_eof(error: __m256i, incomplete: __m256i) -> __m256i {
        SimdU8Value::from(error).or(SimdU8Value::from(incomplete)).0
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn is_incomplete(input: __m256i) -> __m256i {
        SimdU8Value::from(input)
            .saturating_sub(SimdU8Value::from_32_align_end(
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0xff_u8,
                0b1111_0000_u8 - 1,
                0b1110_0000_u8 - 1,
                0b1100_0000_u8 - 1,
            ))
            .0
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn prev1(input: __m256i, prev: __m256i) -> __m256i {
        SimdU8Value::from(input).prev1(SimdU8Value::from(prev)).0
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    #[allow(clippy::too_many_lines)]
    unsafe fn check_special_cases(input: __m256i, prev1: __m256i) -> __m256i {
        const TOO_SHORT: u8 = 1 << 0;
        const TOO_LONG: u8 = 1 << 1;
        const OVERLONG_3: u8 = 1 << 2;
        const SURROGATE: u8 = 1 << 4;
        const OVERLONG_2: u8 = 1 << 5;
        const TWO_CONTS: u8 = 1 << 7;
        const TOO_LARGE: u8 = 1 << 3;
        const TOO_LARGE_1000: u8 = 1 << 6;
        const OVERLONG_4: u8 = 1 << 6;
        const CARRY: u8 = TOO_SHORT | TOO_LONG | TWO_CONTS;

        let input = SimdU8Value::from(input);
        let prev1 = SimdU8Value::from(prev1);
        let byte_1_high = prev1.shr(4).lookup_16(
            TOO_LONG,
            TOO_LONG,
            TOO_LONG,
            TOO_LONG,
            TOO_LONG,
            TOO_LONG,
            TOO_LONG,
            TOO_LONG,
            TWO_CONTS,
            TWO_CONTS,
            TWO_CONTS,
            TWO_CONTS,
            TOO_SHORT | OVERLONG_2,
            TOO_SHORT,
            TOO_SHORT | OVERLONG_3 | SURROGATE,
            TOO_SHORT | TOO_LARGE | TOO_LARGE_1000 | OVERLONG_4,
        );

        let byte_1_low = prev1.and(SimdU8Value::broadcast(0x0F)).lookup_16(
            CARRY | OVERLONG_3 | OVERLONG_2 | OVERLONG_4,
            CARRY | OVERLONG_2,
            CARRY,
            CARRY,
            CARRY | TOO_LARGE,
            CARRY | TOO_LARGE | TOO_LARGE_1000,
            CARRY | TOO_LARGE | TOO_LARGE_1000,
            CARRY | TOO_LARGE | TOO_LARGE_1000,
            CARRY | TOO_LARGE | TOO_LARGE_1000,
            CARRY | TOO_LARGE | TOO_LARGE_1000,
            CARRY | TOO_LARGE | TOO_LARGE_1000,
            CARRY | TOO_LARGE | TOO_LARGE_1000,
            CARRY | TOO_LARGE | TOO_LARGE_1000,
            CARRY | TOO_LARGE | TOO_LARGE_1000 | SURROGATE,
            CARRY | TOO_LARGE | TOO_LARGE_1000,
            CARRY | TOO_LARGE | TOO_LARGE_1000,
        );

        let byte_2_high = input.shr(4).lookup_16(
            TOO_SHORT,
            TOO_SHORT,
            TOO_SHORT,
            TOO_SHORT,
            TOO_SHORT,
            TOO_SHORT,
            TOO_SHORT,
            TOO_SHORT,
            TOO_LONG | OVERLONG_2 | TWO_CONTS | OVERLONG_3 | TOO_LARGE_1000 | OVERLONG_4,
            TOO_LONG | OVERLONG_2 | TWO_CONTS | OVERLONG_3 | TOO_LARGE,
            TOO_LONG | OVERLONG_2 | TWO_CONTS | SURROGATE | TOO_LARGE,
            TOO_LONG | OVERLONG_2 | TWO_CONTS | SURROGATE | TOO_LARGE,
            TOO_SHORT,
            TOO_SHORT,
            TOO_SHORT,
            TOO_SHORT,
        );

        byte_1_high.and(byte_1_low).and(byte_2_high).0
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn check_multibyte_lengths(
        input: __m256i,
        prev: __m256i,
        special_cases: __m256i,
    ) -> __m256i {
        let prev2 = SimdU8Value::from(input).prev2(SimdU8Value::from(prev)).0;
        let prev3 = SimdU8Value::from(input).prev3(SimdU8Value::from(prev)).0;
        let must23 = Self::must_be_2_3_continuation(prev2, prev3);
        let must23_80 = SimdU8Value::from(must23)
            .and(SimdU8Value::broadcast(0x80_u8))
            .0;
        SimdU8Value::from(must23_80)
            .xor(SimdU8Value::from(special_cases))
            .0
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn must_be_2_3_continuation(prev2: __m256i, prev3: __m256i) -> __m256i {
        let is_third_byte =
            SimdU8Value::from(prev2).saturating_sub(SimdU8Value::broadcast(0b1110_0000_u8 - 1));
        let is_fourth_byte =
            SimdU8Value::from(prev3).saturating_sub(SimdU8Value::broadcast(0b1111_0000_u8 - 1));

        is_third_byte
            .or(is_fourth_byte)
            .gt(SimdU8Value::broadcast0())
            .0
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn has_error(error: __m256i) -> bool {
        SimdU8Value::from(error).any_bit_set()
    }

    check_bytes!("avx2", __m256i);
}

#[repr(C, align(64))]
struct SimdInput {
    v0: __m256i,
    v1: __m256i,
}

impl SimdInput {
    #[target_feature(enable = "avx2")]
    #[inline]
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn new(ptr: &[u8]) -> Self {
        Self {
            v0: _mm256_loadu_si256(ptr.as_ptr().cast::<__m256i>()),
            v1: _mm256_loadu_si256(ptr.as_ptr().add(32).cast::<__m256i>()),
        }
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn new_utf8_checking_state() -> Utf8CheckingState<__m256i> {
        Utf8CheckingState::<__m256i>::default()
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn check_block(&self, state: &mut Utf8CheckingState<__m256i>) {
        Utf8CheckingState::<__m256i>::check_bytes(self.v0, state);
        Utf8CheckingState::<__m256i>::check_bytes(self.v1, state);
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn is_ascii(&self) -> bool {
        let res = SimdU8Value::from(self.v0).or(SimdU8Value::from(self.v1)).0;
        _mm256_movemask_epi8(res) == 0
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn check_eof(state: &mut Utf8CheckingState<__m256i>) {
        state.error = Utf8CheckingState::<__m256i>::check_eof(state.error, state.incomplete);
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn check_utf8_errors(state: &Utf8CheckingState<__m256i>) -> bool {
        Utf8CheckingState::<__m256i>::has_error(state.error)
    }

    check_utf8!("avx2", __m256i);
}

use crate::implementation::algorithm::Temp2xSimdChunkA32;
validate_utf8_basic_simd!("avx2", Temp2xSimdChunkA32);
validate_utf8_compat_simd!("avx2", Temp2xSimdChunkA32);
