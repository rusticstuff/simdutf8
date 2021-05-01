#![deny(warnings)]
#![warn(unused_extern_crates)]
#![deny(
    clippy::all,
    clippy::unwrap_used,
    clippy::unnecessary_unwrap,
    clippy::pedantic,
    clippy::nursery
)]
#![allow(clippy::redundant_pub_crate)] // check is broken
#![allow(clippy::redundant_else)] // can make code more readable
#![deny(missing_docs)]
#![cfg_attr(feature = "hints", feature(core_intrinsics))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(
    all(feature = "aarch64_neon", target_arch = "aarch64"),
    feature(stdsimd)
)]

//! Blazingly fast API-compatible UTF-8 validation for Rust using SIMD extensions, based on the implementation from
//! [simdjson](https://github.com/simdjson/simdjson). Originally ported to Rust by the developers of
//! [simd-json.rs](https://simd-json.rs).
//!
//! ## Quick start
//! Use [`basic::from_utf8()`] as a drop-in replacement for [`std::str::from_utf8()`].
//!
//! ```rust
//! use simdutf8::basic::from_utf8;
//!
//! println!("{}", from_utf8(b"I \xE2\x9D\xA4\xEF\xB8\x8F UTF-8!").unwrap());
//! ```
//!
//! If you need detailed information on validation failures, use [`compat::from_utf8()`]
//! instead.
//!
//! ```rust
//! use simdutf8::compat::from_utf8;
//!
//! let err = from_utf8(b"I \xE2\x9D\xA4\xEF\xB8 UTF-8!").unwrap_err();
//! assert_eq!(err.valid_up_to(), 5);
//! assert_eq!(err.error_len(), Some(2));
//! ```
//!
//! ## APIs
//!
//! ### Basic flavor
//! Use the `basic` API flavor for maximum speed. It is fastest on valid UTF-8, but only checks
//! for errors after processing the whole byte sequence and does not provide detailed information if the data
//! is not valid UTF-8. [`basic::Utf8Error`] is a zero-sized error struct.
//!
//! ### Compat flavor
//! The `compat` flavor is fully API-compatible with `std::str::from_utf8`. In particular, [`compat::from_utf8()`]
//! returns a [`compat::Utf8Error`], which has [`valid_up_to()`](compat::Utf8Error#method.valid_up_to) and
//! [`error_len()`](compat::Utf8Error#method.error_len) methods. The first is useful for verification of streamed data. The
//! second is useful e.g. for replacing invalid byte sequences with a replacement character.
//!
//! It also fails early: errors are checked on-the-fly as the string is processed and once
//! an invalid UTF-8 sequence is encountered, it returns without processing the rest of the data.
//! This comes at a performance penality compared to the [`basic`] module even if the input is valid UTF-8.
//!
//! ## Implementation selection
//! The fastest implementation is selected at runtime using the `std::is_x86_feature_detected!` macro unless the CPU
//! targeted by the compiler supports the fastest available implementation.
//! So if you compile with `RUSTFLAGS="-C target-cpu=native"` on a recent x86-64 machine, the AVX 2 implementation is selected at
//! compile time and runtime selection is disabled.
//!
//! For no-std support (compiled with `--no-default-features`) the implementation is always selected at compile time based on
//! the targeted CPU. Use `RUSTFLAGS="-C target-feature=+avx2"` for the AVX 2 implementation or `RUSTFLAGS="-C target-feature=+sse4.2"`
//! for the SSE 4.2 implementation.
//!
//! If you want to be able to call A SIMD implementation directly, use the `public_imp` feature flag. The validation
//! implementations are then accessible via [`basic::imp::x86`] and [`compat::imp::x86`].
//!
//! ## Algorithm
//!
//! See Validating UTF-8 In Less Than One Instruction Per Byte, Software: Practice and Experience 51 (5), 2021
//! <https://arxiv.org/abs/2010.03090>

pub mod basic;
pub mod compat;
mod implementation;

#[cfg(test)]
mod tests;
