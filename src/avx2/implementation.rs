#[cfg(target_arch = "x86")]
use std::arch::x86::{
    __m256i, _mm256_alignr_epi8, _mm256_and_si256, _mm256_cmpgt_epi8, _mm256_loadu_si256,
    _mm256_movemask_epi8, _mm256_or_si256, _mm256_permute2x128_si256, _mm256_set1_epi8,
    _mm256_setr_epi8, _mm256_setzero_si256, _mm256_shuffle_epi8, _mm256_srli_epi16,
    _mm256_subs_epu8, _mm256_testz_si256, _mm256_xor_si256,
};
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{
    __m256i, _mm256_alignr_epi8, _mm256_and_si256, _mm256_cmpgt_epi8, _mm256_loadu_si256,
    _mm256_movemask_epi8, _mm256_or_si256, _mm256_permute2x128_si256, _mm256_set1_epi8,
    _mm256_setr_epi8, _mm256_setzero_si256, _mm256_shuffle_epi8, _mm256_srli_epi16,
    _mm256_subs_epu8, _mm256_testz_si256, _mm256_xor_si256,
};

use crate::utf8check::Utf8CheckingState;
use crate::{mem, static_cast_i8};

impl Utf8CheckingState<__m256i> {
    #[target_feature(enable = "avx2")]
    #[cfg_attr(not(feature = "no-inline"), inline)]
    unsafe fn default() -> Self {
        Self {
            prev: _mm256_setzero_si256(),
            incomplete: _mm256_setzero_si256(),
            error: _mm256_setzero_si256(),
        }
    }

    #[target_feature(enable = "avx2")]
    #[cfg_attr(not(feature = "no-inline"), inline)]
    unsafe fn or(a: __m256i, b: __m256i) -> __m256i {
        _mm256_or_si256(a, b)
    }

    #[target_feature(enable = "avx2")]
    #[cfg_attr(not(feature = "no-inline"), inline)]
    unsafe fn is_ascii(input: __m256i) -> bool {
        _mm256_movemask_epi8(input) == 0
    }

    #[target_feature(enable = "avx2")]
    #[cfg_attr(not(feature = "no-inline"), inline)]
    unsafe fn check_eof(error: __m256i, incomplete: __m256i) -> __m256i {
        Self::or(error, incomplete)
    }

    #[target_feature(enable = "avx2")]
    #[cfg_attr(not(feature = "no-inline"), inline)]
    unsafe fn is_incomplete(input: __m256i) -> __m256i {
        _mm256_subs_epu8(
            input,
            _mm256_setr_epi8(
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0xff_u8),
                static_cast_i8!(0b1111_0000_u8 - 1),
                static_cast_i8!(0b1110_0000_u8 - 1),
                static_cast_i8!(0b1100_0000_u8 - 1),
            ),
        )
    }

    #[target_feature(enable = "avx2")]
    #[cfg_attr(not(feature = "no-inline"), inline)]
    unsafe fn prev1(input: __m256i, prev: __m256i) -> __m256i {
        _mm256_alignr_epi8(input, _mm256_permute2x128_si256(prev, input, 0x21), 16 - 1)
    }

    #[target_feature(enable = "avx2")]
    #[cfg_attr(not(feature = "no-inline"), inline)]
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

        let byte_1_high: __m256i = _mm256_shuffle_epi8(
            _mm256_setr_epi8(
                static_cast_i8!(TOO_LONG),
                static_cast_i8!(TOO_LONG),
                static_cast_i8!(TOO_LONG),
                static_cast_i8!(TOO_LONG),
                static_cast_i8!(TOO_LONG),
                static_cast_i8!(TOO_LONG),
                static_cast_i8!(TOO_LONG),
                static_cast_i8!(TOO_LONG),
                static_cast_i8!(TWO_CONTS),
                static_cast_i8!(TWO_CONTS),
                static_cast_i8!(TWO_CONTS),
                static_cast_i8!(TWO_CONTS),
                static_cast_i8!(TOO_SHORT | OVERLONG_2),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT | OVERLONG_3 | SURROGATE),
                static_cast_i8!(TOO_SHORT | TOO_LARGE | TOO_LARGE_1000 | OVERLONG_4),
                static_cast_i8!(TOO_LONG),
                static_cast_i8!(TOO_LONG),
                static_cast_i8!(TOO_LONG),
                static_cast_i8!(TOO_LONG),
                static_cast_i8!(TOO_LONG),
                static_cast_i8!(TOO_LONG),
                static_cast_i8!(TOO_LONG),
                static_cast_i8!(TOO_LONG),
                static_cast_i8!(TWO_CONTS),
                static_cast_i8!(TWO_CONTS),
                static_cast_i8!(TWO_CONTS),
                static_cast_i8!(TWO_CONTS),
                static_cast_i8!(TOO_SHORT | OVERLONG_2),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT | OVERLONG_3 | SURROGATE),
                static_cast_i8!(TOO_SHORT | TOO_LARGE | TOO_LARGE_1000 | OVERLONG_4),
            ),
            _mm256_and_si256(
                _mm256_srli_epi16(prev1, 4),
                _mm256_set1_epi8(static_cast_i8!(0xFF_u8 >> 4)),
            ),
        );

        let byte_1_low: __m256i = _mm256_shuffle_epi8(
            _mm256_setr_epi8(
                static_cast_i8!(CARRY | OVERLONG_3 | OVERLONG_2 | OVERLONG_4),
                static_cast_i8!(CARRY | OVERLONG_2),
                static_cast_i8!(CARRY),
                static_cast_i8!(CARRY),
                static_cast_i8!(CARRY | TOO_LARGE),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000 | SURROGATE),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000),
                static_cast_i8!(CARRY | OVERLONG_3 | OVERLONG_2 | OVERLONG_4),
                static_cast_i8!(CARRY | OVERLONG_2),
                static_cast_i8!(CARRY),
                static_cast_i8!(CARRY),
                static_cast_i8!(CARRY | TOO_LARGE),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000 | SURROGATE),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000),
                static_cast_i8!(CARRY | TOO_LARGE | TOO_LARGE_1000),
            ),
            _mm256_and_si256(prev1, _mm256_set1_epi8(0x0F)),
        );

        let byte_2_high: __m256i = _mm256_shuffle_epi8(
            _mm256_setr_epi8(
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(
                    TOO_LONG | OVERLONG_2 | TWO_CONTS | OVERLONG_3 | TOO_LARGE_1000 | OVERLONG_4
                ),
                static_cast_i8!(TOO_LONG | OVERLONG_2 | TWO_CONTS | OVERLONG_3 | TOO_LARGE),
                static_cast_i8!(TOO_LONG | OVERLONG_2 | TWO_CONTS | SURROGATE | TOO_LARGE),
                static_cast_i8!(TOO_LONG | OVERLONG_2 | TWO_CONTS | SURROGATE | TOO_LARGE),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(
                    TOO_LONG | OVERLONG_2 | TWO_CONTS | OVERLONG_3 | TOO_LARGE_1000 | OVERLONG_4
                ),
                static_cast_i8!(TOO_LONG | OVERLONG_2 | TWO_CONTS | OVERLONG_3 | TOO_LARGE),
                static_cast_i8!(TOO_LONG | OVERLONG_2 | TWO_CONTS | SURROGATE | TOO_LARGE),
                static_cast_i8!(TOO_LONG | OVERLONG_2 | TWO_CONTS | SURROGATE | TOO_LARGE),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
                static_cast_i8!(TOO_SHORT),
            ),
            _mm256_and_si256(
                _mm256_srli_epi16(input, 4),
                _mm256_set1_epi8(static_cast_i8!(0xFF_u8 >> 4)),
            ),
        );

        _mm256_and_si256(_mm256_and_si256(byte_1_high, byte_1_low), byte_2_high)
    }

    #[target_feature(enable = "avx2")]
    #[cfg_attr(not(feature = "no-inline"), inline)]
    unsafe fn check_multibyte_lengths(
        input: __m256i,
        prev: __m256i,
        special_cases: __m256i,
    ) -> __m256i {
        let prev2 = _mm256_alignr_epi8(input, _mm256_permute2x128_si256(prev, input, 0x21), 16 - 2);
        let prev3 = _mm256_alignr_epi8(input, _mm256_permute2x128_si256(prev, input, 0x21), 16 - 3);
        let must23 = Self::must_be_2_3_continuation(prev2, prev3);
        let must23_80 = _mm256_and_si256(must23, _mm256_set1_epi8(static_cast_i8!(0x80_u8)));
        _mm256_xor_si256(must23_80, special_cases)
    }

    #[target_feature(enable = "avx2")]
    #[cfg_attr(not(feature = "no-inline"), inline)]
    unsafe fn must_be_2_3_continuation(prev2: __m256i, prev3: __m256i) -> __m256i {
        let is_third_byte =
            _mm256_subs_epu8(prev2, _mm256_set1_epi8(static_cast_i8!(0b1110_0000_u8 - 1)));
        let is_fourth_byte =
            _mm256_subs_epu8(prev3, _mm256_set1_epi8(static_cast_i8!(0b1111_0000_u8 - 1)));
        _mm256_cmpgt_epi8(
            _mm256_or_si256(is_third_byte, is_fourth_byte),
            _mm256_set1_epi8(0),
        )
    }

    #[target_feature(enable = "avx2")]
    #[cfg_attr(not(feature = "no-inline"), inline)]
    unsafe fn has_error(error: __m256i) -> bool {
        _mm256_testz_si256(error, error) != 1
    }

    #[target_feature(enable = "avx2")]
    #[cfg_attr(not(feature = "no-inline"), inline)]
    check_bytes!(__m256i);
}

#[derive(Debug)]
struct SimdInput {
    v0: __m256i,
    v1: __m256i,
}

impl SimdInput {
    #[target_feature(enable = "avx2")]
    #[cfg_attr(not(feature = "no-inline"), inline)]
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn new(ptr: &[u8]) -> Self {
        Self {
            v0: _mm256_loadu_si256(ptr.as_ptr().cast::<__m256i>()),
            v1: _mm256_loadu_si256(ptr.as_ptr().add(32).cast::<__m256i>()),
        }
    }

    #[target_feature(enable = "avx2")]
    #[cfg_attr(not(feature = "no-inline"), inline)]
    unsafe fn new_utf8_checking_state() -> Utf8CheckingState<__m256i> {
        Utf8CheckingState::<__m256i>::default()
    }

    #[target_feature(enable = "avx2")]
    #[cfg_attr(not(feature = "no-inline"), inline)]
    unsafe fn check_utf8(&self, state: &mut Utf8CheckingState<__m256i>) {
        Utf8CheckingState::<__m256i>::check_bytes(self.v0, state);
        Utf8CheckingState::<__m256i>::check_bytes(self.v1, state);
    }

    #[target_feature(enable = "avx2")]
    #[cfg_attr(not(feature = "no-inline"), inline)]
    unsafe fn check_eof(state: &mut Utf8CheckingState<__m256i>) {
        state.error = Utf8CheckingState::<__m256i>::check_eof(state.error, state.incomplete);
    }

    #[target_feature(enable = "avx2")]
    #[cfg_attr(not(feature = "no-inline"), inline)]
    unsafe fn check_utf8_errors(state: &Utf8CheckingState<__m256i>) -> bool {
        Utf8CheckingState::<__m256i>::has_error(state.error)
    }
}

#[target_feature(enable = "avx2")]
validate_utf8_simd!();
