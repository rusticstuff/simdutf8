#![allow(dead_code)]
use crate::utf8check::Utf8Check;

use crate::{ProcessedUtfBytes, Utf8CheckingState};
#[cfg(target_arch = "x86")]
use std::arch::x86::{__m128i, _mm_loadu_si128};
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{__m128i, _mm_loadu_si128};

use std::mem;

pub const SIMDJSON_PADDING: usize = mem::size_of::<__m128i>() * 2;
pub const SIMDINPUT_LENGTH: usize = 64;

#[derive(Debug)]
pub(crate) struct SimdInput {
    v0: __m128i,
    v1: __m128i,
    v2: __m128i,
    v3: __m128i,
}

impl SimdInput {
    #[cfg_attr(not(feature = "no-inline"), inline)]
    #[allow(clippy::cast_ptr_alignment)]
    pub(crate) fn new(ptr: &[u8]) -> Self {
        unsafe {
            Self {
                v0: _mm_loadu_si128(ptr.as_ptr().cast::<__m128i>()),
                v1: _mm_loadu_si128(ptr.as_ptr().add(16).cast::<__m128i>()),
                v2: _mm_loadu_si128(ptr.as_ptr().add(32).cast::<__m128i>()),
                v3: _mm_loadu_si128(ptr.as_ptr().add(48).cast::<__m128i>()),
            }
        }
    }

    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    pub(crate) fn new_utf8_checking_state() -> Utf8CheckingState<__m128i> {
        ProcessedUtfBytes::<__m128i>::default()
    }

    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    pub(crate) fn check_utf8(&self, state: &mut Utf8CheckingState<__m128i>) {
        unsafe {
            ProcessedUtfBytes::<__m128i>::check_bytes(self.v0, state);
            ProcessedUtfBytes::<__m128i>::check_bytes(self.v1, state);
            ProcessedUtfBytes::<__m128i>::check_bytes(self.v2, state);
            ProcessedUtfBytes::<__m128i>::check_bytes(self.v3, state);
        }
    }

    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    pub(crate) fn check_utf8_errors(state: &Utf8CheckingState<__m128i>) -> bool {
        unsafe { ProcessedUtfBytes::<__m128i>::has_error(state.error) }
    }
}
