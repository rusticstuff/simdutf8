#![allow(warnings)]

type Utf8ErrorBasic = crate::basic::Utf8Error;
type Utf8ErrorCompat = crate::compat::Utf8Error;

#[allow(dead_code)]
use core::arch::aarch64::{
    vandq_u8, vbicq_u8, vdupq_n_u8, veorq_u8, vextq_u8, vgetq_lane_u16, vmaxvq_u8, vmovq_n_u8,
    vorrq_u8, vpaddq_u8, vreinterpretq_u16_u8, vsetq_lane_s8, vsetq_lane_u8,
};

#[inline]
pub(crate) fn validate_utf8_basic(_input: &[u8]) -> Result<(), Utf8ErrorBasic> {
    todo!();
}

#[inline]
pub(crate) fn validate_utf8_compat(_input: &[u8]) -> Result<(), Utf8ErrorCompat> {
    todo!();
}
