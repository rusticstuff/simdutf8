/// possible compiler hint that a branch is likely
#[cfg(feature = "hints")]
macro_rules! likely {
    ($e:expr) => {
        std::intrinsics::likely($e)
    };
}

/// possible compiler hint that a branch is likely
#[cfg(not(feature = "hints"))]
macro_rules! likely {
    ($e:expr) => {
        $e
    };
}

/// possible compiler hint that a branch is unlikely
#[cfg(feature = "hints")]
macro_rules! unlikely {
    ($e:expr) => {
        std::intrinsics::unlikely($e)
    };
}

/// possible compiler hint that a branch is unlikely
#[cfg(not(feature = "hints"))]
macro_rules! unlikely {
    ($e:expr) => {
        $e
    };
}

/// static cast to an i8
macro_rules! static_cast_i8 {
    ($v:expr) => {
        mem::transmute::<_, i8>($v)
    };
}

/// check_bytes() strategy
macro_rules! check_bytes {
    ($feat:expr, $t:ident) => {
        #[target_feature(enable = $feat)]
        unsafe fn check_bytes(current: $t, previous: &mut Utf8CheckingState<$t>) {
            if likely!(Self::is_ascii(current)) {
                previous.error = Self::check_eof(previous.error, previous.incomplete)
            } else {
                let prev1 = Self::prev1(current, previous.prev);
                let sc = Self::check_special_cases(current, prev1);
                previous.error = Self::or(
                    previous.error,
                    Self::check_multibyte_lengths(current, previous.prev, sc),
                );
                previous.incomplete = Self::is_incomplete(current);
            }
            previous.prev = current
        }
    };
}

/// validate_utf8_pure_simd() strategy and wrapper
macro_rules! validate_utf8_pure_simd {
    ($feat:expr) => {
        #[target_feature(enable = $feat)]
        #[inline]
        pub(crate) unsafe fn validate_utf8_pure(
            input: &[u8],
        ) -> core::result::Result<(), crate::pure::Utf8Error> {
            const SIMDINPUT_LENGTH: usize = 64;
            let len = input.len();
            let mut state = SimdInput::new_utf8_checking_state();
            let lenminus64: usize = if len < 64 { 0 } else { len as usize - 64 };
            let mut idx: usize = 0;
            let mut tmpbuf = crate::implementation::AlignToSixtyFour([0; 64], [0; 64]);

            if lenminus64 >= 4096 {
                let off = ((input.as_ptr() as usize) & 31);
                if off != 0 {
                    let to_copy = 32 - off;
                    tmpbuf.0[off..]
                        .as_mut_ptr()
                        .copy_from(input.as_ptr(), to_copy);
                    let simd_input = SimdInput::new(&tmpbuf.0);
                    simd_input.check_utf8(&mut state);
                    idx += to_copy;
                }
            }
            while idx < lenminus64 {
                let input = SimdInput::new(input.get_unchecked(idx as usize..));
                input.check_utf8(&mut state);
                idx += SIMDINPUT_LENGTH;
            }

            if idx < len {
                tmpbuf
                    .1
                    .as_mut_ptr()
                    .copy_from(input.as_ptr().add(idx), len as usize - idx);
                let input = SimdInput::new(&tmpbuf.1);

                input.check_utf8(&mut state);
            }
            SimdInput::check_eof(&mut state);
            if unlikely!(SimdInput::check_utf8_errors(&state)) {
                Err(crate::pure::Utf8Error {})
            } else {
                Ok(())
            }
        }
    };
}

/// validate_utf8_compat_simd() strategy and wrapper
macro_rules! validate_utf8_compat_simd {
    ($feat:expr) => {
        #[target_feature(enable = $feat)]
        #[inline]
        pub(crate) unsafe fn validate_utf8_compat(
            input: &[u8],
        ) -> core::result::Result<(), crate::compat::Utf8Error> {
            validate_utf8_compat_simd0(input)
                .map_err(|idx| crate::implementation::get_compat_error(input, idx))
        }

        #[target_feature(enable = $feat)]
        #[inline]
        unsafe fn validate_utf8_compat_simd0(input: &[u8]) -> core::result::Result<(), usize> {
            const SIMDINPUT_LENGTH: usize = 64;
            let len = input.len();
            let mut state = SimdInput::new_utf8_checking_state();
            let lenminus64: usize = if len < 64 { 0 } else { len as usize - 64 };
            let mut idx: usize = 0;
            let mut tmpbuf = crate::implementation::AlignToSixtyFour([0; 64], [0; 64]);

            if lenminus64 >= 4096 {
                let off = ((input.as_ptr() as usize) & (SIMDINPUT_LENGTH - 1));
                if off != 0 {
                    tmpbuf.0[off..]
                        .as_mut_ptr()
                        .copy_from(input.as_ptr(), 64 - off);
                    let simd_input = SimdInput::new(&tmpbuf.0);
                    simd_input.check_utf8(&mut state);
                    if SimdInput::check_utf8_errors(&state) {
                        return Err(idx);
                    }
                    idx += 64 - off;
                }
            }

            while idx < lenminus64 {
                let simd_input = SimdInput::new(input.get_unchecked(idx as usize..));
                simd_input.check_utf8(&mut state);
                if SimdInput::check_utf8_errors(&state) {
                    return Err(idx);
                }
                idx += SIMDINPUT_LENGTH;
            }
            if idx < len {
                tmpbuf
                    .1
                    .as_mut_ptr()
                    .copy_from(input.as_ptr().add(idx), len as usize - idx);
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
