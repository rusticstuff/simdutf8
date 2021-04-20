#![deny(warnings)]
#![warn(unused_extern_crates)]
#![deny(
    clippy::all,
    clippy::unwrap_used,
    clippy::unnecessary_unwrap,
    clippy::pedantic
)]
#![deny(missing_docs)]
#![cfg_attr(feature = "hints", feature(core_intrinsics))]
#![cfg_attr(not(feature = "std"), no_std)]

//! UTF-8 checking crate
//!
//! Quick start:
//! ```
//! use simdutf8::basic::{from_utf8, Utf8Error};
//!
//! fn main() {
//!     println!("{}", from_utf8(b"I \xEE\x80\xA2 UTF-8!").unwrap());
//! }
//! ```

pub mod basic;
pub mod compat;
mod implementation;

#[cfg(test)]
mod tests;
