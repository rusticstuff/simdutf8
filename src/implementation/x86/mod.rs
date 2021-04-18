use super::Utf8ErrorCompat;
use super::Utf8ErrorPure;

#[allow(dead_code)]
pub(crate) mod avx2;

#[allow(dead_code)]
pub(crate) mod sse42;

// validate_utf8_pure() std

#[cfg(feature = "std")]
#[inline]
pub(super) unsafe fn validate_utf8_pure(input: &[u8]) -> core::result::Result<(), Utf8ErrorPure> {
    use core::mem;
    use std::sync::atomic::{AtomicPtr, Ordering};

    type FnRaw = *mut ();

    static FN: AtomicPtr<()> = AtomicPtr::new(get_fastest as FnRaw);

    unsafe fn get_fastest(input: &[u8]) -> core::result::Result<(), Utf8ErrorPure> {
        let fun = get_fastest_available_implementation_pure();
        FN.store(fun as FnRaw, Ordering::Relaxed);
        (fun)(input)
    }

    let fun = FN.load(Ordering::Relaxed);
    mem::transmute::<FnRaw, super::ValidateUtf8Fn>(fun)(input)
}

#[cfg(feature = "std")]
#[inline]
fn get_fastest_available_implementation_pure() -> super::ValidateUtf8Fn {
    if std::is_x86_feature_detected!("avx2") {
        avx2::validate_utf8_pure_simd
    } else if std::is_x86_feature_detected!("sse4.2") {
        sse42::validate_utf8_pure_simd
    } else {
        super::validate_utf8_pure_fallback
    }
}

// alidate_utf8_pure() no-std implementations

#[cfg(all(not(feature = "std"), target_feature = "avx2"))]
#[inline]
pub(super) unsafe fn validate_utf8_pure(input: &[u8]) -> Result<(), Utf8ErrorPure> {
    avx2::validate_utf8_pure_simd(input)
}

#[cfg(all(
    not(feature = "std"),
    not(target_feature = "avx2"),
    target_feature = "sse4.2"
))]
#[inline]
pub(super) unsafe fn validate_utf8_pure(input: &[u8]) -> Result<(), Utf8ErrorPure> {
    sse42::validate_utf8_pure_simd(input)
}

#[cfg(all(
    not(feature = "std"),
    not(target_feature = "avx2"),
    not(target_feature = "sse4.2")
))]
#[inline]
pub(super) unsafe fn validate_utf8_pure(input: &[u8]) -> Result<(), Utf8ErrorPure> {
    super::validate_utf8_pure_fallback(input)
}

// validate_utf8_compat() implementations

#[cfg(feature = "std")]
#[inline]
pub(super) unsafe fn validate_utf8_compat(
    input: &[u8],
) -> core::result::Result<(), Utf8ErrorCompat> {
    use core::mem;
    use std::sync::atomic::{AtomicPtr, Ordering};

    type FnRaw = *mut ();

    static FN: AtomicPtr<()> = AtomicPtr::new(get_fastest as FnRaw);

    unsafe fn get_fastest(input: &[u8]) -> core::result::Result<(), Utf8ErrorCompat> {
        let fun = get_fastest_available_implementation_compat();
        FN.store(fun as FnRaw, Ordering::Relaxed);
        (fun)(input)
    }

    let fun = FN.load(Ordering::Relaxed);
    mem::transmute::<FnRaw, super::ValidateUtf8CompatFn>(fun)(input)
}

#[cfg(feature = "std")]
#[inline]
fn get_fastest_available_implementation_compat() -> super::ValidateUtf8CompatFn {
    if std::is_x86_feature_detected!("avx2") {
        avx2::validate_utf8_compat_simd
    } else if std::is_x86_feature_detected!("sse4.2") {
        sse42::validate_utf8_compat_simd
    } else {
        super::validate_utf8_compat_fallback
    }
}

// validate_utf8_compat() no-std implementations

#[cfg(all(not(feature = "std"), target_feature = "avx2"))]
#[inline]
pub(super) unsafe fn validate_utf8_compat(input: &[u8]) -> Result<(), Utf8ErrorCompat> {
    avx2::validate_utf8_compat_simd(input)
}

#[cfg(all(
    not(feature = "std"),
    not(target_feature = "avx2"),
    target_feature = "sse4.2"
))]
#[inline]
pub(super) unsafe fn validate_utf8_compat(input: &[u8]) -> Result<(), Utf8ErrorCompat> {
    sse42::validate_utf8_compat_simd(input)
}

#[cfg(all(
    not(feature = "std"),
    not(target_feature = "avx2"),
    not(target_feature = "sse4.2")
))]
#[inline]
pub(super) unsafe fn validate_utf8_compat(input: &[u8]) -> Result<(), Utf8ErrorCompat> {
    super::validate_utf8_compat_fallback(input)
}
