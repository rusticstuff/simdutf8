#![allow(dead_code)]
use crate::utf8check::Utf8Check;
use crate::{ProcessedUtfBytes, Utf8CheckingState};
#[cfg(target_arch = "x86")]
use std::arch::x86::{__m256i, _mm256_loadu_si256};
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{__m256i, _mm256_loadu_si256};

use std::mem;

pub const SIMDJSON_PADDING: usize = mem::size_of::<__m256i>();
pub const SIMDINPUT_LENGTH: usize = 64;

#[derive(Debug)]
pub(crate) struct SimdInput {
    v0: __m256i,
    v1: __m256i,
}

impl SimdInput {
    #[cfg_attr(not(feature = "no-inline"), inline)]
    #[allow(clippy::cast_ptr_alignment)]
    pub(crate) fn new(ptr: &[u8]) -> Self {
        unsafe {
            Self {
                v0: _mm256_loadu_si256(ptr.as_ptr().cast::<__m256i>()),
                v1: _mm256_loadu_si256(ptr.as_ptr().add(32).cast::<__m256i>()),
            }
        }
    }
}

impl SimdInput {
    #[cfg_attr(not(feature = "no-inline"), inline)]
    pub(crate) fn new_utf8_checking_state() -> Utf8CheckingState<__m256i> {
        ProcessedUtfBytes::<__m256i>::default()
    }

    #[cfg_attr(not(feature = "no-inline"), inline)]
    pub(crate) fn check_utf8(&self, state: &mut Utf8CheckingState<__m256i>) {
        unsafe {
            ProcessedUtfBytes::<__m256i>::check_bytes(self.v0, state);
            ProcessedUtfBytes::<__m256i>::check_bytes(self.v1, state);
        }
    }

    #[cfg_attr(not(feature = "no-inline"), inline)]
    pub(crate) fn check_eof(state: &mut Utf8CheckingState<__m256i>) {
        unsafe {
            state.error = ProcessedUtfBytes::<__m256i>::check_eof(state.error, state.incomplete);
        }
    }

    #[cfg_attr(not(feature = "no-inline"), inline)]
    pub(crate) fn check_utf8_errors(state: &Utf8CheckingState<__m256i>) -> bool {
        unsafe { ProcessedUtfBytes::<__m256i>::has_error(state.error) }
    }
}
