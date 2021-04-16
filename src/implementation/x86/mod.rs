use crate::{Utf8Error, Utf8ErrorExact};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(dead_code)]
pub mod avx2;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(dead_code)]
pub mod sse42;

#[cfg(all(
    not(feature = "std"),
    not(target_feature = "avx2"),
    not(target_feature = "sse4.2")
))]
#[cfg_attr(not(feature = "no-inline"), inline)]
pub(crate) unsafe fn validate_utf8(input: &[u8]) -> Result<(), Utf8Error> {
    super::validate_utf8_fallback(input)
}

#[cfg(any(
    all(
        not(feature = "std"),
        not(target_feature = "avx2"),
        target_feature = "sse4.2"
    ),
    forcesse42
))]
#[cfg_attr(not(feature = "no-inline"), inline)]
pub(crate) unsafe fn validate_utf8(input: &[u8]) -> Result<(), Utf8Error> {
    sse42::validate_utf8_simd(input)
}

#[cfg(target_feature = "avx2")]
pub(crate) unsafe fn validate_utf8(input: &[u8]) -> Result<(), Utf8Error> {
    avx2::validate_utf8_simd(input)
}

#[cfg(all(
    feature = "std",
    any(target_arch = "x86", target_arch = "x86_64"),
    not(target_feature = "avx2"),
    not(forcesse42)
))]
#[cfg_attr(not(feature = "no-inline"), inline)]
fn get_fastest_available_implementation() -> super::ValidateUtf8Fn {
    if std::is_x86_feature_detected!("avx2") {
        avx2::validate_utf8_simd
    } else if std::is_x86_feature_detected!("sse4.2") {
        sse42::validate_utf8_simd
    } else {
        super::validate_utf8_fallback
    }
}

#[cfg(all(feature = "std", not(target_feature = "avx2"), not(forcesse42)))]
#[cfg_attr(not(feature = "no-inline"), inline)]
pub(crate) fn validate_utf8(input: &[u8]) -> core::result::Result<(), Utf8Error> {
    use core::mem;
    use std::sync::atomic::{AtomicPtr, Ordering};

    type FnRaw = *mut ();

    static FN: AtomicPtr<()> = AtomicPtr::new(get_fastest as FnRaw);

    fn get_fastest(input: &[u8]) -> core::result::Result<(), Utf8Error> {
        let fun = get_fastest_available_implementation();
        FN.store(fun as FnRaw, Ordering::Relaxed);
        unsafe { (fun)(input) }
    }

    unsafe {
        let fun = FN.load(Ordering::Relaxed);
        mem::transmute::<FnRaw, super::ValidateUtf8Fn>(fun)(input)
    }
}

#[cfg(target_feature = "avx2")]
pub(crate) unsafe fn validate_utf8_exact(input: &[u8]) -> Result<(), Utf8ErrorExact> {
    avx2::validate_utf8_exact_simd(input)
}

#[cfg(all(
    feature = "std",
    any(target_arch = "x86", target_arch = "x86_64"),
    not(target_feature = "avx2"),
    not(forcesse42)
))]
#[cfg_attr(not(feature = "no-inline"), inline)]
fn get_fastest_available_implementation_exact() -> super::ValidateUtf8ExactFn {
    if std::is_x86_feature_detected!("avx2") {
        avx2::validate_utf8_exact_simd
    } else if std::is_x86_feature_detected!("sse4.2") {
        sse42::validate_utf8_exact_simd
    } else {
        super::validate_utf8_exact_fallback
    }
}

#[cfg(all(feature = "std", not(target_feature = "avx2"), not(forcesse42)))]
#[cfg_attr(not(feature = "no-inline"), inline)]
pub(crate) fn validate_utf8_exact(input: &[u8]) -> core::result::Result<(), Utf8ErrorExact> {
    use core::mem;
    use std::sync::atomic::{AtomicPtr, Ordering};

    type FnRaw = *mut ();

    static FN: AtomicPtr<()> = AtomicPtr::new(get_fastest as FnRaw);

    fn get_fastest(input: &[u8]) -> core::result::Result<(), Utf8ErrorExact> {
        let fun = get_fastest_available_implementation_exact();
        FN.store(fun as FnRaw, Ordering::Relaxed);
        unsafe { (fun)(input) }
    }

    unsafe {
        let fun = FN.load(Ordering::Relaxed);
        mem::transmute::<FnRaw, super::ValidateUtf8ExactFn>(fun)(input)
    }
}
