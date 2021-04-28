/// Macros requires newtypes in scope: `SimdInput` with holds 64 bytes of SIMD input

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
            use crate::implementation::algorithm::SIMD_CHUNK_SIZE;
            let len = input.len();
            let mut state = SimdInput::new_utf8_checking_state();
            let mut idx: usize = 0;
            let mut tmpbuf = $buf2type::new();

            let align: usize = core::mem::align_of::<$buf2type>();
            if len >= 4096 {
                let off = (input.as_ptr() as usize) % align;
                if off != 0 {
                    let to_copy = align - off;
                    crate::implementation::algorithm::memcpy_unaligned_nonoverlapping_inline(
                        input.as_ptr(),
                        tmpbuf.0[SIMD_CHUNK_SIZE - align + off..].as_mut_ptr(),
                        to_copy,
                    );
                    let simd_input = SimdInput::new(&tmpbuf.0);
                    state.check_utf8(&simd_input);
                    idx += to_copy;
                }
            }

            let rem = len - idx;
            let iter_lim = idx + (rem - (rem % SIMD_CHUNK_SIZE));
            while idx < iter_lim {
                let input = SimdInput::new(input.get_unchecked(idx as usize..));
                state.check_utf8(&input);
                idx += SIMD_CHUNK_SIZE;
            }

            if idx < len {
                crate::implementation::algorithm::memcpy_unaligned_nonoverlapping_inline(
                    input.as_ptr().add(idx),
                    tmpbuf.1.as_mut_ptr(),
                    len - idx,
                );
                let input = SimdInput::new(&tmpbuf.1);

                state.check_utf8(&input);
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
                .map_err(|idx| crate::implementation::algorithm::get_compat_error(input, idx))
        }

        #[target_feature(enable = $feat)]
        #[inline]
        unsafe fn validate_utf8_compat_simd0(input: &[u8]) -> core::result::Result<(), usize> {
            use crate::implementation::algorithm::SIMD_CHUNK_SIZE;
            let len = input.len();
            let mut state = SimdInput::new_utf8_checking_state();
            let mut idx: usize = 0;
            let mut tmpbuf = $buf2type::new();

            let align: usize = core::mem::align_of::<$buf2type>();
            if len >= 4096 {
                let off = (input.as_ptr() as usize) % align;
                if off != 0 {
                    let to_copy = align - off;
                    crate::implementation::algorithm::memcpy_unaligned_nonoverlapping_inline(
                        input.as_ptr(),
                        tmpbuf.0[SIMD_CHUNK_SIZE - align + off..].as_mut_ptr(),
                        to_copy,
                    );
                    let simd_input = SimdInput::new(&tmpbuf.0);
                    state.check_utf8(&simd_input);
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
                state.check_utf8(&simd_input);
                if SimdInput::check_utf8_errors(&state) {
                    return Err(idx);
                }
                idx += SIMD_CHUNK_SIZE;
            }
            if idx < len {
                crate::implementation::algorithm::memcpy_unaligned_nonoverlapping_inline(
                    input.as_ptr().add(idx),
                    tmpbuf.1.as_mut_ptr(),
                    len - idx,
                );
                let simd_input = SimdInput::new(&tmpbuf.1);

                state.check_utf8(&simd_input);
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
