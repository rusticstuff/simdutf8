/// Macros requires newtypes in scope:
/// `SimdU8Value` - implementation of SIMD primitives
/// `SimdInput` - which  holds 64 bytes of SIMD input
/// `Temp2xSimdChunk` - correctly aligned Temp2xSimdChunk, either Temp2xSimdChunkA16 or Temp2xSimdChunkA32

/// validate_utf8_basic_simd() strategy and wrapper
macro_rules! algorithm_simd {
    ($feat:expr) => {
        impl Utf8CheckAlgorithm<SimdU8Value> {
            #[target_feature(enable = $feat)]
            #[inline]
            unsafe fn default() -> Self {
                Self {
                    prev: SimdU8Value::broadcast0(),
                    incomplete: SimdU8Value::broadcast0(),
                    error: SimdU8Value::broadcast0(),
                }
            }

            #[target_feature(enable = $feat)]
            #[inline]
            unsafe fn or(a: SimdU8Value, b: SimdU8Value) -> SimdU8Value {
                a.or(b)
            }

            #[target_feature(enable = $feat)]
            #[inline]
            unsafe fn check_eof(&mut self) {
                self.error = self.error.or(self.incomplete)
            }

            #[target_feature(enable = $feat)]
            #[inline]
            unsafe fn is_incomplete(input: SimdU8Value) -> SimdU8Value {
                input.saturating_sub(SimdU8Value::from_32_cut_off_leading(
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

            #[target_feature(enable = $feat)]
            #[inline]
            unsafe fn prev1(input: SimdU8Value, prev: SimdU8Value) -> SimdU8Value {
                input.prev1(prev)
            }

            #[target_feature(enable = $feat)]
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

            #[target_feature(enable = $feat)]
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

            #[target_feature(enable = $feat)]
            #[inline]
            unsafe fn must_be_2_3_continuation(
                prev2: SimdU8Value,
                prev3: SimdU8Value,
            ) -> SimdU8Value {
                let is_third_byte = prev2.saturating_sub(SimdU8Value::broadcast(0b1110_0000 - 1));
                let is_fourth_byte = prev3.saturating_sub(SimdU8Value::broadcast(0b1111_0000 - 1));

                is_third_byte
                    .or(is_fourth_byte)
                    .gt(SimdU8Value::broadcast0())
            }

            #[target_feature(enable = $feat)]
            #[inline]
            unsafe fn has_error(&self) -> bool {
                self.error.any_bit_set()
            }

            #[target_feature(enable = $feat)]
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

            #[target_feature(enable = $feat)]
            #[inline]
            unsafe fn check_utf8(&mut self, input: SimdInput) {
                if likely!(input.is_ascii()) {
                    self.check_eof();
                } else {
                    self.check_block(input);
                }
            }

            #[target_feature(enable = $feat)]
            #[inline]
            unsafe fn check_block(&mut self, input: SimdInput) {
                for i in 0..input.vals.len() {
                    Self::check_bytes(input.vals[i], self);
                }
            }
        }

        /// Validation implementation for CPUs supporting the SIMD extension (see module).
        ///
        /// # Errors
        /// Return the zero-sized [`crate::basic::Utf8Error`] on failure.
        ///
        /// # Safety
        /// This function is inherently unsafe because it is compiled with SIMD extensions
        /// enabled. Make sure that the CPU supports it before calling.
        ///
        #[target_feature(enable = $feat)]
        #[inline]
        pub unsafe fn validate_utf8_basic(
            input: &[u8],
        ) -> core::result::Result<(), crate::basic::Utf8Error> {
            use crate::implementation::helpers::SIMD_CHUNK_SIZE;
            let len = input.len();
            let mut algorithm = Utf8CheckAlgorithm::<SimdU8Value>::default();
            let mut idx: usize = 0;
            let mut tmpbuf = Temp2xSimdChunk::new();
            let mut only_ascii = true;

            let align: usize = core::mem::align_of::<Temp2xSimdChunk>();
            if len >= 4096 {
                let off = (input.as_ptr() as usize) % align;
                if off != 0 {
                    let to_copy = align - off;
                    tmpbuf.0[SIMD_CHUNK_SIZE - align + off..]
                        .as_mut_ptr()
                        .copy_from_nonoverlapping(input.as_ptr(), to_copy);
                    let simd_input = SimdInput::new(&tmpbuf.0);
                    idx += to_copy;
                    if !simd_input.is_ascii() {
                        algorithm.check_block(simd_input);
                        only_ascii = false;
                    }
                }
            }

            let rem = len - idx;
            let iter_lim = idx + (rem - (rem % SIMD_CHUNK_SIZE));
            if only_ascii {
                while idx < iter_lim {
                    let simd_input = SimdInput::new(input.get_unchecked(idx as usize..));
                    idx += SIMD_CHUNK_SIZE;
                    if !simd_input.is_ascii() {
                        algorithm.check_block(simd_input);
                        only_ascii = false;
                        break;
                    }
                }
            }
            while idx < iter_lim {
                let input = SimdInput::new(input.get_unchecked(idx as usize..));
                algorithm.check_utf8(input);
                idx += SIMD_CHUNK_SIZE;
            }

            if idx < len {
                tmpbuf
                    .1
                    .as_mut_ptr()
                    .copy_from_nonoverlapping(input.as_ptr().add(idx), len - idx);
                let simd_input = SimdInput::new(&tmpbuf.1);
                if only_ascii {
                    if simd_input.is_ascii() {
                        return Ok(());
                    }
                    algorithm.check_block(simd_input);
                } else {
                    algorithm.check_utf8(simd_input);
                }
                // algorithm.check_utf8(simd_input);
            }
            algorithm.check_eof();
            if unlikely!(algorithm.has_error()) {
                Err(crate::basic::Utf8Error {})
            } else {
                Ok(())
            }
        }

        /// Validation implementation for CPUs supporting the SIMD extension (see module).
        ///
        /// # Errors
        /// Return [`crate::compat::Utf8Error`] with detailed error information on failure.
        ///
        /// # Safety
        /// This function is inherently unsafe because it is compiled with SIMD extensions
        /// enabled. Make sure that the CPU supports it before calling.
        ///
        #[target_feature(enable = $feat)]
        #[inline]
        pub unsafe fn validate_utf8_compat(
            input: &[u8],
        ) -> core::result::Result<(), crate::compat::Utf8Error> {
            validate_utf8_compat_simd0(input)
                .map_err(|idx| crate::implementation::helpers::get_compat_error(input, idx))
        }

        #[target_feature(enable = $feat)]
        #[inline]
        unsafe fn validate_utf8_compat_simd0(input: &[u8]) -> core::result::Result<(), usize> {
            use crate::implementation::helpers::SIMD_CHUNK_SIZE;
            let len = input.len();
            let mut algorithm = Utf8CheckAlgorithm::<SimdU8Value>::default();
            let mut idx: usize = 0;
            let mut tmpbuf = Temp2xSimdChunk::new();

            let align: usize = core::mem::align_of::<Temp2xSimdChunk>();
            if len >= 4096 {
                let off = (input.as_ptr() as usize) % align;
                if off != 0 {
                    let to_copy = align - off;
                    tmpbuf.0[SIMD_CHUNK_SIZE - align + off..]
                        .as_mut_ptr()
                        .copy_from_nonoverlapping(input.as_ptr(), to_copy);
                    let simd_input = SimdInput::new(&tmpbuf.0);
                    algorithm.check_utf8(simd_input);
                    if algorithm.has_error() {
                        return Err(idx);
                    }
                    idx += to_copy;
                }
            }

            let rem = len - idx;
            let iter_lim = idx + (rem - (rem % SIMD_CHUNK_SIZE));
            while idx < iter_lim {
                let simd_input = SimdInput::new(input.get_unchecked(idx as usize..));
                algorithm.check_utf8(simd_input);
                if algorithm.has_error() {
                    return Err(idx);
                }
                idx += SIMD_CHUNK_SIZE;
            }
            if idx < len {
                tmpbuf
                    .1
                    .as_mut_ptr()
                    .copy_from_nonoverlapping(input.as_ptr().add(idx), len - idx);
                let simd_input = SimdInput::new(&tmpbuf.1);

                algorithm.check_utf8(simd_input);
            }
            algorithm.check_eof();
            if unlikely!(algorithm.has_error()) {
                Err(idx)
            } else {
                Ok(())
            }
        }
    };
}

macro_rules! simd_input_128_bit {
    ($feat:expr) => {
        #[repr(C)]
        struct SimdInput {
            vals: [SimdU8Value; 4],
        }

        impl SimdInput {
            #[target_feature(enable = $feat)]
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

            #[target_feature(enable = $feat)]
            #[inline]
            unsafe fn is_ascii(&self) -> bool {
                let r1 = self.vals[0].or(self.vals[1]);
                let r2 = self.vals[2].or(self.vals[3]);
                let r = r1.or(r2);
                r.is_ascii()
            }
        }
    };
}

macro_rules! simd_input_256_bit {
    ($feat:expr) => {
        #[repr(C)]
        struct SimdInput {
            vals: [SimdU8Value; 2],
        }

        impl SimdInput {
            #[target_feature(enable = $feat)]
            #[inline]
            #[allow(clippy::cast_ptr_alignment)]
            unsafe fn new(ptr: &[u8]) -> Self {
                Self {
                    vals: [
                        SimdU8Value::load_from(ptr.as_ptr()),
                        SimdU8Value::load_from(ptr.as_ptr().add(32)),
                    ],
                }
            }

            #[target_feature(enable = $feat)]
            #[inline]
            unsafe fn is_ascii(&self) -> bool {
                self.vals[0].or(self.vals[1]).is_ascii()
            }
        }
    };
}
