//! Contains the arm/aarch64  UTF-8 validation implementation.
//! See Validating UTF-8 In Less Than One Instruction Per Byte, Software: Practice and Experience 51 (5), 2021
//! <https://arxiv.org/abs/2010.03090
//! https://github.com/simdjson/simdjson/blob/master/include/simdjson/arm64/simd.h
//! https://github.com/rust-lang/stdarch/blob/master/crates/core_arch/src/aarch64/neon/generated.rs
//! https://github.com/rust-lang/stdarch/blob/master/crates/core_arch/src/aarch64/neon/mod.rs
//! Godbolt.org  test template
//! -emit-llvm -O2 -target aarch64-apple-darwin 
//! #include <arm_neon.h>
//! int test(){
//!  return (int) vaba_s16;
//! }
//!


// For intrinsics not supported in core::arch::aarch64 they will need to be added here


//#[allow(dead_code)]
//#[cfg(target_arch = "aarch64")]
//use core::arch::aarch64::{vsetq_lane_u8, vsetq_lane_s8, vorrq_u8, vandq_u8,veorq_u8,vbicq_u8, vextq_u8, vmovq_n_u8, vpaddq_u8, vgetq_lane_u16,vreinterpretq_u16_u8, vmaxvq_u8, vdupq_n_u8   };


