use std::simd::{
    cmp::SimdPartialOrd,
    num::{SimdInt, SimdUint},
    simd_swizzle, u8x16, LaneCount, Simd, SupportedLaneCount,
};

use crate::{basic, compat, implementation::helpers::SIMD_CHUNK_SIZE};

#[cfg(all(
    any(target_arch = "aarch64", target_arch = "arm"),
    target_feature = "neon"
))]
const HAS_FAST_REDUCE_MAX: bool = true;

#[cfg(not(all(
    any(target_arch = "aarch64", target_arch = "arm"),
    target_feature = "neon"
)))]
const HAS_FAST_REDUCE_MAX: bool = false;

const HAS_FAST_MASKED_LOAD: bool = false; // FIXME avx512, avx2 (?)

#[repr(C, align(32))]
#[allow(dead_code)] // only used if a 128-bit SIMD implementation is used
pub(crate) struct TempSimdChunk(pub(crate) [u8; SIMD_CHUNK_SIZE]);

#[allow(dead_code)] // only used if there is a SIMD implementation
impl TempSimdChunk {
    #[expect(clippy::inline_always)]
    #[inline(always)] // FIXME needs to be forced because otherwise it is not inlined on armv7 neo
    pub(crate) const fn new() -> Self {
        Self([0; SIMD_CHUNK_SIZE])
    }
}

#[repr(C)]
struct SimdInput<const N: usize, const O: usize>
where
    LaneCount<N>: SupportedLaneCount,
{
    vals: [Simd<u8, N>; O],
}

trait SimdInputTrait {
    fn new(ptr: &[u8]) -> Self;
    fn new_partial_masked_load(slice: &[u8]) -> Self;
    fn new_partial_copy(slice: &[u8]) -> Self;
    fn new_partial(slice: &[u8]) -> Self
    where
        Self: Sized,
    {
        if HAS_FAST_MASKED_LOAD {
            Self::new_partial_masked_load(slice)
        } else {
            Self::new_partial_copy(slice)
        }
    }
    fn is_ascii(&self) -> bool;
}

impl SimdInputTrait for SimdInput<16, 4> {
    #[inline]
    fn new(s: &[u8]) -> Self {
        assert!(s.len() == 64);
        Self {
            vals: [
                u8x16::from_slice(&s[..16]),
                u8x16::from_slice(&s[16..32]),
                u8x16::from_slice(&s[32..48]),
                u8x16::from_slice(&s[48..64]),
            ],
        }
    }

    #[inline]
    fn new_partial_masked_load(mut slice: &[u8]) -> Self {
        let val0 = load_masked_opt(slice);
        slice = &slice[slice.len().min(16)..];
        if slice.is_empty() {
            return Self {
                vals: [val0, u8x16::default(), u8x16::default(), u8x16::default()],
            };
        }
        let val1 = load_masked_opt(slice);
        slice = &slice[slice.len().min(16)..];
        if slice.is_empty() {
            return Self {
                vals: [val0, val1, u8x16::default(), u8x16::default()],
            };
        }
        let val2 = load_masked_opt(slice);
        slice = &slice[slice.len().min(16)..];
        if slice.is_empty() {
            return Self {
                vals: [val0, val1, val2, u8x16::default()],
            };
        }
        let val3 = load_masked_opt(slice);
        Self {
            vals: [val0, val1, val2, val3],
        }
    }

    #[inline]
    fn new_partial_copy(slice: &[u8]) -> Self {
        let mut buf = [0; 64];
        buf[..slice.len()].copy_from_slice(slice);
        Self::new(&buf)
    }

    #[inline]
    fn is_ascii(&self) -> bool {
        (self.vals[0] | self.vals[1] | self.vals[2] | self.vals[3]).is_ascii()
    }
}

#[inline]
fn load_masked_opt(slice: &[u8]) -> Simd<u8, 16> {
    if slice.len() > 15 {
        u8x16::from_slice(&slice[..16])
    } else {
        u8x16::load_or_default(slice)
    }
}

struct Utf8CheckAlgorithm<const N: usize, const O: usize>
where
    LaneCount<N>: SupportedLaneCount,
{
    pub(crate) prev: Simd<u8, N>,
    pub(crate) incomplete: Simd<u8, N>, // FIXME: should be a mask?
    pub(crate) error: Simd<u8, N>,      // FIXME: should be a mask?
}

trait SimdU8Value<const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
    Self: Copy,
{
    #[expect(clippy::too_many_arguments)]
    fn from_32_cut_off_leading(
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
    ) -> Self;

    #[expect(clippy::too_many_arguments)]
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
    ) -> Self;

    #[expect(clippy::too_many_arguments)]
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
    ) -> Self;

    // const generics would be more awkward and verbose with the current
    // portable SIMD swizzle implementation and compiler limitations.
    fn prev1(self, prev: Self) -> Self;
    fn prev2(self, prev: Self) -> Self;
    fn prev3(self, prev: Self) -> Self;

    fn is_ascii(self) -> bool;
}

impl SimdU8Value<16> for u8x16 {
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
        Self::from_array([
            v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31,
        ])
    }

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
        Self::from_array([
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15,
        ])
    }

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
        let src = Self::repeat_16(
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15,
        );
        src.swizzle_dyn(self)
    }

    #[inline]
    fn prev1(self, prev: Self) -> Self {
        simd_swizzle!(
            self,
            prev,
            [31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,]
        )
    }

    #[inline]
    fn prev2(self, prev: Self) -> Self {
        simd_swizzle!(
            self,
            prev,
            [30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,]
        )
    }

    #[inline]
    fn prev3(self, prev: Self) -> Self {
        simd_swizzle!(
            self,
            prev,
            [29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,]
        )
    }

    #[inline]
    fn is_ascii(self) -> bool {
        if HAS_FAST_REDUCE_MAX {
            self.reduce_max() < 0b1000_0000
        } else {
            (self & Self::splat(0b1000_0000)) == Self::splat(0)
        }
    }
}

impl<const N: usize, const O: usize> Utf8CheckAlgorithm<N, O>
where
    LaneCount<N>: SupportedLaneCount,
    Simd<u8, N>: SimdU8Value<N>,
    SimdInput<N, O>: SimdInputTrait,
{
    #[inline]
    fn new() -> Self {
        Self {
            prev: Simd::<u8, N>::splat(0),
            incomplete: Simd::<u8, N>::splat(0),
            error: Simd::<u8, N>::splat(0),
        }
    }

    #[inline]
    fn check_incomplete_pending(&mut self) {
        self.error |= self.incomplete;
    }

    #[inline]
    fn is_incomplete(input: Simd<u8, N>) -> Simd<u8, N> {
        input.saturating_sub(SimdU8Value::<N>::from_32_cut_off_leading(
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

    #[inline]
    fn check_special_cases(input: Simd<u8, N>, prev1: Simd<u8, N>) -> Simd<u8, N> {
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

        let byte_1_high = (prev1 >> 4).lookup_16(
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

        let byte_1_low = (prev1 & Simd::<u8, N>::splat(0x0F)).lookup_16(
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

        let byte_2_high = (input >> 4).lookup_16(
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

        byte_1_high & byte_1_low & byte_2_high
    }

    #[inline]
    fn must_be_2_3_continuation(prev2: Simd<u8, N>, prev3: Simd<u8, N>) -> Simd<u8, N> {
        let is_third_byte = prev2
            .simd_gt(Simd::<u8, N>::splat(0b1110_0000 - 1))
            .to_int();
        let is_fourth_byte = prev3
            .simd_gt(Simd::<u8, N>::splat(0b1111_0000 - 1))
            .to_int();

        (is_third_byte | is_fourth_byte).cast()
    }

    #[inline]
    fn check_multibyte_lengths(
        input: Simd<u8, N>,
        prev: Simd<u8, N>,
        special_cases: Simd<u8, N>,
    ) -> Simd<u8, N> {
        let prev2 = input.prev2(prev);
        let prev3 = input.prev3(prev);
        let must23 = Self::must_be_2_3_continuation(prev2, prev3);
        let must23_80 = must23 & Simd::<u8, N>::splat(0x80);
        must23_80 ^ special_cases
    }

    #[inline]
    fn has_error(&self) -> bool {
        // FIXME: max workaround
        if HAS_FAST_REDUCE_MAX {
            self.error.reduce_max() != 0
        } else {
            self.error != Simd::<u8, N>::splat(0)
        }
    }

    #[inline]
    fn check_bytes(&mut self, input: Simd<u8, N>) {
        let prev1 = input.prev1(self.prev);
        let sc = Self::check_special_cases(input, prev1);
        self.error |= Self::check_multibyte_lengths(input, self.prev, sc);
        self.prev = input;
    }

    #[inline]
    fn check_utf8(&mut self, input: &SimdInput<N, O>) {
        if input.is_ascii() {
            self.check_incomplete_pending();
        } else {
            self.check_block(input);
        }
    }

    #[inline]
    fn check_block(&mut self, input: &SimdInput<N, O>) {
        // WORKAROUND
        // necessary because the for loop is not unrolled on ARM64
        if input.vals.len() == 2 {
            self.check_bytes(input.vals[0]);
            self.check_bytes(input.vals[1]);
            self.incomplete = Self::is_incomplete(input.vals[1]);
        } else if input.vals.len() == 4 {
            self.check_bytes(input.vals[0]);
            self.check_bytes(input.vals[1]);
            self.check_bytes(input.vals[2]);
            self.check_bytes(input.vals[3]);
            self.incomplete = Self::is_incomplete(input.vals[3]);
        } else {
            panic!("Unsupported number of chunks");
        }
    }

    /// Validation implementation for CPUs supporting the SIMD extension (see module).
    ///
    /// # Errors
    /// Returns the zero-sized [`basic::Utf8Error`] on failure.
    ///
    #[inline]
    pub fn validate_utf8_basic(input: &[u8]) -> core::result::Result<(), basic::Utf8Error> {
        use crate::implementation::helpers::SIMD_CHUNK_SIZE;
        let mut algorithm = Self::new();
        let mut chunks = input.chunks_exact(SIMD_CHUNK_SIZE);
        for chunk in chunks.by_ref() {
            let simd_input = SimdInput::<N, O>::new(chunk);
            if !simd_input.is_ascii() {
                algorithm.check_block(&simd_input);
                break;
            }
        }
        for chunk in chunks.by_ref() {
            let simd_input = SimdInput::<N, O>::new(chunk);
            algorithm.check_utf8(&simd_input);
        }
        let rem = chunks.remainder();
        if !rem.is_ascii() {
            // FIXME: simd???
            let simd_input = SimdInput::<N, O>::new_partial(rem);
            algorithm.check_utf8(&simd_input);
        }
        algorithm.check_incomplete_pending();
        if algorithm.has_error() {
            Err(basic::Utf8Error {})
        } else {
            Ok(())
        }
    }

    #[inline]
    #[expect(clippy::redundant_else)] // more readable
    fn validate_utf8_compat_simd0(input: &[u8]) -> core::result::Result<(), usize> {
        use crate::implementation::helpers::SIMD_CHUNK_SIZE;
        let mut algorithm = Self::new();
        let mut idx = 0;
        let mut chunks = input.chunks_exact(SIMD_CHUNK_SIZE);
        let mut only_ascii = true;

        'outer: loop {
            if only_ascii {
                while let Some(chunk) = chunks.next() {
                    let simd_input = SimdInput::new(chunk);
                    if !simd_input.is_ascii() {
                        algorithm.check_block(&simd_input);
                        if algorithm.has_error() {
                            return Err(idx);
                        } else {
                            only_ascii = false;
                            idx += SIMD_CHUNK_SIZE;
                            continue 'outer;
                        }
                    }
                    idx += SIMD_CHUNK_SIZE;
                }
            } else {
                while let Some(chunk) = chunks.next() {
                    let simd_input = SimdInput::new(chunk);
                    if simd_input.is_ascii() {
                        algorithm.check_incomplete_pending();
                        if algorithm.has_error() {
                            return Err(idx);
                        } else {
                            // we are in pure ASCII territory again
                            only_ascii = true;
                            idx += SIMD_CHUNK_SIZE;
                            continue 'outer;
                        }
                    } else {
                        algorithm.check_block(&simd_input);
                        if algorithm.has_error() {
                            return Err(idx);
                        }
                    }
                    idx += SIMD_CHUNK_SIZE;
                }
            }
            break;
        }
        let rem = chunks.remainder();
        if !rem.is_ascii() {
            // FIXME: simd???
            let simd_input = SimdInput::<N, O>::new_partial(rem);
            algorithm.check_utf8(&simd_input);
        }
        algorithm.check_incomplete_pending();
        if algorithm.has_error() {
            Err(idx)
        } else {
            Ok(())
        }
    }
}

#[inline]
pub fn validate_utf8_basic(input: &[u8]) -> core::result::Result<(), basic::Utf8Error> {
    Utf8CheckAlgorithm::<16, 4>::validate_utf8_basic(input)
}

/// Validation implementation for CPUs supporting the SIMD extension (see module).
///
/// # Errors
/// Returns [`compat::Utf8Error`] with detailed error information on failure.
///
/// # Safety
/// This function is inherently unsafe because it is compiled with SIMD extensions
/// enabled. Make sure that the CPU supports it before calling.
///
#[inline]
pub fn validate_utf8_compat(input: &[u8]) -> core::result::Result<(), compat::Utf8Error> {
    Utf8CheckAlgorithm::<16, 4>::validate_utf8_compat_simd0(input)
        .map_err(|idx| crate::implementation::helpers::get_compat_error(input, idx))
}
