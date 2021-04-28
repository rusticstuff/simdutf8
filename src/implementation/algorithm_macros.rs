/// check_utf8() strategy
macro_rules! check_utf8 {
    ($feat:expr, $t:ident) => {
        #[target_feature(enable = $feat)]
        #[inline]
        unsafe fn check_utf8(&self, previous: &mut Utf8CheckingState<$t>) {
            if likely!(self.is_ascii()) {
                previous.error =
                    Utf8CheckingState::<$t>::check_eof(previous.error, previous.incomplete)
            } else {
                self.check_block(previous);
            }
        }
    };
}

/// check_bytes() strategy
macro_rules! check_bytes {
    ($feat:expr, $t:ident) => {
        #[target_feature(enable = $feat)]
        #[inline]
        unsafe fn check_bytes(current: $t, previous: &mut Utf8CheckingState<$t>) {
            let prev1 = Self::prev1(current, previous.prev);
            let sc = Self::check_special_cases(current, prev1);
            previous.error = Self::or(
                previous.error,
                Self::check_multibyte_lengths(current, previous.prev, sc),
            );
            previous.incomplete = Self::is_incomplete(current);
            previous.prev = current
        }
    };
}

/// validate_utf8_basic_simd() strategy and wrapper
macro_rules! validate_utf8_basic_simd {
    ($feat:expr, $buf2type:ident) => {
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
            use crate::implementation::SIMD_CHUNK_SIZE;
            let len = input.len();
            let mut state = SimdInput::new_utf8_checking_state();
            let mut idx: usize = 0;
            let mut tmpbuf = $buf2type::new();

            let align: usize = core::mem::align_of::<$buf2type>();
            if len >= 4096 {
                let off = (input.as_ptr() as usize) % align;
                if off != 0 {
                    let to_copy = align - off;
                    crate::implementation::memcpy_unaligned_nonoverlapping_inline(
                        input.as_ptr(),
                        tmpbuf.0[SIMD_CHUNK_SIZE - align + off..].as_mut_ptr(),
                        to_copy,
                    );
                    let simd_input = SimdInput::new(&tmpbuf.0);
                    simd_input.check_utf8(&mut state);
                    idx += to_copy;
                }
            }

            let rem = len - idx;
            let iter_lim = idx + (rem - (rem % SIMD_CHUNK_SIZE));
            while idx < iter_lim {
                let input = SimdInput::new(input.get_unchecked(idx as usize..));
                input.check_utf8(&mut state);
                idx += SIMD_CHUNK_SIZE;
            }

            if idx < len {
                crate::implementation::memcpy_unaligned_nonoverlapping_inline(
                    input.as_ptr().add(idx),
                    tmpbuf.1.as_mut_ptr(),
                    len - idx,
                );
                let input = SimdInput::new(&tmpbuf.1);

                input.check_utf8(&mut state);
            }
            SimdInput::check_eof(&mut state);
            if unlikely!(SimdInput::check_utf8_errors(&state)) {
                Err(crate::basic::Utf8Error {})
            } else {
                Ok(())
            }
        }
    };
}

/// validate_utf8_compat_simd() strategy and wrapper
macro_rules! validate_utf8_compat_simd {
    ($feat:expr, $buf2type:ident) => {
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
                .map_err(|idx| crate::implementation::get_compat_error(input, idx))
        }

        #[target_feature(enable = $feat)]
        #[inline]
        unsafe fn validate_utf8_compat_simd0(input: &[u8]) -> core::result::Result<(), usize> {
            use crate::implementation::SIMD_CHUNK_SIZE;
            let len = input.len();
            let mut state = SimdInput::new_utf8_checking_state();
            let mut idx: usize = 0;
            let mut tmpbuf = $buf2type::new();

            let align: usize = core::mem::align_of::<$buf2type>();
            if len >= 4096 {
                let off = (input.as_ptr() as usize) % align;
                if off != 0 {
                    let to_copy = align - off;
                    crate::implementation::memcpy_unaligned_nonoverlapping_inline(
                        input.as_ptr(),
                        tmpbuf.0[SIMD_CHUNK_SIZE - align + off..].as_mut_ptr(),
                        to_copy,
                    );
                    let simd_input = SimdInput::new(&tmpbuf.0);
                    simd_input.check_utf8(&mut state);
                    if SimdInput::check_utf8_errors(&state) {
                        return Err(idx);
                    }
                    idx += to_copy;
                }
            }

            let rem = len - idx;
            let iter_lim = idx + (rem - (rem % SIMD_CHUNK_SIZE));
            while idx < iter_lim {
                let simd_input = SimdInput::new(input.get_unchecked(idx as usize..));
                simd_input.check_utf8(&mut state);
                if SimdInput::check_utf8_errors(&state) {
                    return Err(idx);
                }
                idx += SIMD_CHUNK_SIZE;
            }
            if idx < len {
                crate::implementation::memcpy_unaligned_nonoverlapping_inline(
                    input.as_ptr().add(idx),
                    tmpbuf.1.as_mut_ptr(),
                    len - idx,
                );
                let simd_input = SimdInput::new(&tmpbuf.1);

                simd_input.check_utf8(&mut state);
            }
            SimdInput::check_eof(&mut state);
            if unlikely!(SimdInput::check_utf8_errors(&state)) {
                Err(idx)
            } else {
                Ok(())
            }
        }
    };
}
