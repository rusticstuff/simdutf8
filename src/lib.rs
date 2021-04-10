// #![deny(warnings)]
#![cfg_attr(target_feature = "neon", feature(stdsimd,))]
#![cfg_attr(feature = "hints", feature(core_intrinsics))]
#![deny(warnings)]
#![warn(unused_extern_crates)]
#![deny(
    clippy::all,
    clippy::unwrap_used,
    clippy::unnecessary_unwrap,
    clippy::pedantic
)]
// We might want to revisit inline_always
#![allow(clippy::module_name_repetitions, clippy::inline_always)]
#![deny(missing_docs)]

//! UTF-8 checking crate

#[macro_use]
mod macros;
mod utf8check;

#[cfg(target_feature = "avx2")]
mod avx2;
#[cfg(target_feature = "avx2")]
use crate::avx2::stage1::{SimdInput, SIMDINPUT_LENGTH};

#[cfg(all(target_feature = "sse4.2", not(target_feature = "avx2")))]
mod sse42;
#[cfg(all(target_feature = "sse4.2", not(target_feature = "avx2")))]
use crate::sse42::stage1::{SimdInput, SIMDINPUT_LENGTH};

// We import this as generics
#[cfg(all(not(any(target_feature = "sse4.2", target_feature = "avx2"))))]
mod sse42;
#[cfg(all(not(any(target_feature = "sse4.2", target_feature = "avx2"))))]
use crate::sse42::stage1::{SimdInput, SIMDINPUT_LENGTH};

#[cfg(all(
    not(feature = "allow-non-simd"),
    not(any(target_feature = "sse4.2", target_feature = "avx2"))
))]
fn please_compile_with_a_simd_compatible_cpu_setting_read_the_simdjonsrs_readme() -> ! {}

use crate::utf8check::ProcessedUtfBytes;

use std::mem;

/// Utf8Error struct
pub struct Utf8Error {}

/// Validates the UTF-8 string
/// # Errors
///
/// Will return Err(ErrorType::InvalidUTF8) on invalid UTF-8
pub fn validate_utf8(input: &[u8]) -> std::result::Result<(), Utf8Error> {
    unsafe {
        let len = input.len();
        let mut state = SimdInput::new_utf8_checking_state();
        let lenminus64: usize = if len < 64 { 0 } else { len as usize - 64 };
        let mut idx: usize = 0;

        while idx < lenminus64 {
            /*
            #ifndef _MSC_VER
              __builtin_prefetch(buf + idx + 128);
            #endif
             */
            let input = SimdInput::new(input.get_unchecked(idx as usize..));
            input.check_utf8(&mut state);
            idx += SIMDINPUT_LENGTH;
        }

        if idx < len {
            let mut tmpbuf: [u8; SIMDINPUT_LENGTH] = [0x20; SIMDINPUT_LENGTH];
            tmpbuf
                .as_mut_ptr()
                .copy_from(input.as_ptr().add(idx), len as usize - idx);
            let input = SimdInput::new(&tmpbuf);

            input.check_utf8(&mut state);
        }

        if SimdInput::check_utf8_errors(&state) {
            Err(Utf8Error {})
        } else {
            Ok(())
        }
    }
}

pub(crate) type Utf8CheckingState<T> = ProcessedUtfBytes<T>;
