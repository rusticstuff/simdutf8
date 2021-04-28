//! Contains the x86-64/x86 SSE4.2 UTF-8 validation implementation.
//!
//! See Validating UTF-8 In Less Than One Instruction Per Byte, Software: Practice and Experience 51 (5), 2021
//! <https://arxiv.org/abs/2010.03090>
#[allow(dead_code)]
#[cfg(target_arch = "x86")]
use core::arch::x86::{
    __m128i, _mm_alignr_epi8, _mm_and_si128, _mm_cmpgt_epi8, _mm_loadu_si128, _mm_movemask_epi8,
    _mm_or_si128, _mm_set1_epi8, _mm_setr_epi8, _mm_setzero_si128, _mm_shuffle_epi8,
    _mm_srli_epi16, _mm_subs_epu8, _mm_testz_si128, _mm_xor_si128,
};
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
    __m128i, _mm_alignr_epi8, _mm_and_si128, _mm_cmpgt_epi8, _mm_loadu_si128, _mm_movemask_epi8,
    _mm_or_si128, _mm_set1_epi8, _mm_setr_epi8, _mm_setzero_si128, _mm_shuffle_epi8,
    _mm_srli_epi16, _mm_subs_epu8, _mm_testz_si128, _mm_xor_si128,
};

use crate::implementation::algorithm::Utf8CheckingState;

type SimdU8Value = crate::implementation::algorithm::SimdU8Value<__m128i>;

impl SimdU8Value {
    #[target_feature(enable = "sse4.2")]
    #[inline]
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::cast_possible_wrap)]
    unsafe fn from_32_align_end(
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
        Self::from(_mm_setr_epi8(
            v16 as i8, v17 as i8, v18 as i8, v19 as i8, v20 as i8, v21 as i8, v22 as i8, v23 as i8,
            v24 as i8, v25 as i8, v26 as i8, v27 as i8, v28 as i8, v29 as i8, v30 as i8, v31 as i8,
        ))
    }

    #[target_feature(enable = "sse4.2")]
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
        Self::from(_mm_setr_epi8(
            v0 as i8, v1 as i8, v2 as i8, v3 as i8, v4 as i8, v5 as i8, v6 as i8, v7 as i8,
            v8 as i8, v9 as i8, v10 as i8, v11 as i8, v12 as i8, v13 as i8, v14 as i8, v15 as i8,
        ))
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn load_from(ptr: *const u8) -> Self {
        Self::from(_mm_loadu_si128(ptr.cast::<__m128i>()))
    }

    #[target_feature(enable = "sse4.2")]
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
    #[allow(clippy::cast_possible_wrap)]
    unsafe fn broadcast(val: u8) -> Self {
        Self::from(_mm_set1_epi8(val as i8))
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    #[allow(clippy::cast_possible_wrap)]
    unsafe fn broadcast0() -> Self {
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
    #[allow(clippy::cast_lossless)]
    #[inline]
    unsafe fn shr4(self) -> Self {
        Self::from(_mm_srli_epi16(self.0, 4)).and(Self::broadcast(0xFF >> 4))
    }

    // ugly but prev<N> requires const generics
    #[target_feature(enable = "sse4.2")]
    #[allow(clippy::cast_lossless)]
    #[inline]
    unsafe fn prev1(self, prev: Self) -> Self {
        Self::from(_mm_alignr_epi8(self.0, prev.0, 16 - 1))
    }

    // ugly but prev<N> requires const generics
    #[target_feature(enable = "sse4.2")]
    #[allow(clippy::cast_lossless)]
    #[inline]
    unsafe fn prev2(self, prev: Self) -> Self {
        Self::from(_mm_alignr_epi8(self.0, prev.0, 16 - 2))
    }

    // ugly but prev<N> requires const generics
    #[target_feature(enable = "sse4.2")]
    #[allow(clippy::cast_lossless)]
    #[inline]
    unsafe fn prev3(self, prev: Self) -> Self {
        Self::from(_mm_alignr_epi8(self.0, prev.0, 16 - 3))
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn gt(self, other: Self) -> Self {
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

// ------- generic implementation starts here -------

impl Utf8CheckingState<SimdU8Value> {
    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn default() -> Self {
        Self {
            prev: SimdU8Value::broadcast0(),
            incomplete: SimdU8Value::broadcast0(),
            error: SimdU8Value::broadcast0(),
        }
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn or(a: SimdU8Value, b: SimdU8Value) -> SimdU8Value {
        a.or(b)
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn check_eof(&mut self) {
        self.error = self.error.or(self.incomplete)
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn is_incomplete(input: SimdU8Value) -> SimdU8Value {
        input.saturating_sub(SimdU8Value::from_32_align_end(
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0b1111_0000 - 1,
            0b1110_0000 - 1,
            0b1100_0000 - 1,
        ))
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn prev1(input: SimdU8Value, prev: SimdU8Value) -> SimdU8Value {
        input.prev1(prev)
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    #[allow(clippy::too_many_lines)]
    unsafe fn check_special_cases(input: SimdU8Value, prev1: SimdU8Value) -> SimdU8Value {
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

        let byte_1_high = prev1.shr4().lookup_16(
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

        let byte_2_high = input.shr4().lookup_16(
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

        byte_1_high.and(byte_1_low).and(byte_2_high)
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn check_multibyte_lengths(
        input: SimdU8Value,
        prev: SimdU8Value,
        special_cases: SimdU8Value,
    ) -> SimdU8Value {
        let prev2 = input.prev2(prev);
        let prev3 = input.prev3(prev);
        let must23 = Self::must_be_2_3_continuation(prev2, prev3);
        let must23_80 = must23.and(SimdU8Value::broadcast(0x80));
        must23_80.xor(special_cases)
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn must_be_2_3_continuation(prev2: SimdU8Value, prev3: SimdU8Value) -> SimdU8Value {
        let is_third_byte = prev2.saturating_sub(SimdU8Value::broadcast(0b1110_0000 - 1));
        let is_fourth_byte = prev3.saturating_sub(SimdU8Value::broadcast(0b1111_0000 - 1));

        is_third_byte
            .or(is_fourth_byte)
            .gt(SimdU8Value::broadcast0())
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn has_error(&self) -> bool {
        self.error.any_bit_set()
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn check_bytes(current: SimdU8Value, previous: &mut Self) {
        let prev1 = Self::prev1(current, previous.prev);
        let sc = Self::check_special_cases(current, prev1);
        previous.error = Self::or(
            previous.error,
            Self::check_multibyte_lengths(current, previous.prev, sc),
        );
        previous.incomplete = Self::is_incomplete(current);
        previous.prev = current
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn check_utf8(&mut self, input: &SimdInput) {
        if likely!(input.is_ascii()) {
            self.check_eof();
        } else {
            self.check_block(input);
        }
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn check_block(&mut self, input: &SimdInput) {
        for i in 0..input.vals.len() {
            Self::check_bytes(input.vals[i], self);
        }
    }
}

#[repr(C)]
struct SimdInput {
    vals: [SimdU8Value; 4],
}

impl SimdInput {
    #[target_feature(enable = "sse4.2")]
    #[inline]
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn new(ptr: &[u8]) -> Self {
        Self {
            vals: [
                SimdU8Value::load_from(ptr.as_ptr()),
                SimdU8Value::load_from(ptr.as_ptr().add(16)),
                SimdU8Value::load_from(ptr.as_ptr().add(32)),
                SimdU8Value::load_from(ptr.as_ptr().add(48)),
            ],
        }
    }

    #[target_feature(enable = "sse4.2")]
    #[inline]
    unsafe fn is_ascii(&self) -> bool {
        let r1 = self.vals[0].or(self.vals[1]);
        let r2 = self.vals[2].or(self.vals[3]);
        let r = r1.or(r2);
        r.is_ascii()
    }
}

use crate::implementation::algorithm::Temp2xSimdChunkA16 as Temp2xSimdChunk;
algorithm_simd!("sse4.2");
