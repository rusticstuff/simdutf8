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

pub mod implementation;

/// UTF-8 validation error
#[derive(Debug)]
pub struct Utf8Error {}

/// Checks if the byte sequence is valid UTF-8 and returns `Ok(str)` if it is.
///
/// # Errors
/// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
#[allow(unused_variables)]
#[cfg(not(feature = "std"))]
pub fn from_utf8(input: &[u8]) -> core::result::Result<&str, Utf8Error> {
    implementation::get_fastest_available_implementation()(input)?;
    unsafe { Ok(core::str::from_utf8_unchecked(input)) }
}

/// Checks if the byte sequence is valid UTF-8 and returns `Ok(str)` if it is.
///
/// # Errors
/// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
#[cfg(all(feature = "std", target_feature = "avx2"))]
pub fn from_utf8(input: &[u8]) -> core::result::Result<&str, Utf8Error> {
    implementation::avx2::validate_utf8_simd(input)?;
    unsafe { Ok(core::str::from_utf8_unchecked(input)) }
}

/// Checks if the byte sequence is valid UTF-8 and returns `Ok(str)` if it is.
///
/// # Errors
/// Will return `Err(Utf8Error)` on if the input contains invalid UTF-8
#[cfg(all(feature = "std", not(target_feature = "avx2")))]
pub fn from_utf8(input: &[u8]) -> core::result::Result<&str, Utf8Error> {
    use implementation::{get_fastest_available_implementation, ValidateUtf8Fn};
    use std::mem;
    use std::sync::atomic::{AtomicPtr, Ordering};

    type FnRaw = *mut ();

    static FN: AtomicPtr<()> = AtomicPtr::new(get_fastest as FnRaw);

    fn get_fastest(input: &[u8]) -> core::result::Result<(), Utf8Error> {
        let fun = get_fastest_available_implementation();
        FN.store(fun as FnRaw, Ordering::Relaxed);
        (fun)(input)
    }

    unsafe {
        let fun = FN.load(Ordering::Relaxed);
        mem::transmute::<FnRaw, ValidateUtf8Fn>(fun)(input)?;
        Ok(core::str::from_utf8_unchecked(input))
    }
}

#[cfg(test)]
mod tests;
