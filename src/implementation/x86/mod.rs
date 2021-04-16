use super::Utf8ErrorCompat;
use super::Utf8ErrorPure;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(dead_code)]
mod avx2;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(dead_code)]
mod sse42;

// validate_utf8_pure() implementations

#[cfg(all(
    not(feature = "std"),
    not(target_feature = "avx2"),
    not(target_feature = "sse4.2")
))]
#[cfg_attr(not(feature = "no-inline"), inline)]
pub(super) unsafe fn validate_utf8_pure(input: &[u8]) -> Result<(), Utf8ErrorPure> {
    super::validate_utf8_pure_fallback(input)
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
pub(super) unsafe fn validate_utf8_pure(input: &[u8]) -> Result<(), Utf8ErrorPure> {
    sse42::validate_utf8_pure_simd(input)
}

#[cfg(target_feature = "avx2")]
pub(super) unsafe fn validate_utf8_pure(input: &[u8]) -> Result<(), Utf8ErrorPure> {
    avx2::validate_utf8_pure_simd(input)
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
        avx2::validate_utf8_pure_simd
    } else if std::is_x86_feature_detected!("sse4.2") {
        sse42::validate_utf8_pure_simd
    } else {
        super::validate_utf8_pure_fallback
    }
}

#[cfg(all(feature = "std", not(target_feature = "avx2"), not(forcesse42)))]
#[cfg_attr(not(feature = "no-inline"), inline)]
pub(super) unsafe fn validate_utf8_pure(input: &[u8]) -> core::result::Result<(), Utf8ErrorPure> {
    use core::mem;
    use std::sync::atomic::{AtomicPtr, Ordering};

    type FnRaw = *mut ();

    static FN: AtomicPtr<()> = AtomicPtr::new(get_fastest as FnRaw);

    unsafe fn get_fastest(input: &[u8]) -> core::result::Result<(), Utf8ErrorPure> {
        let fun = get_fastest_available_implementation();
        FN.store(fun as FnRaw, Ordering::Relaxed);
        (fun)(input)
    }

    let fun = FN.load(Ordering::Relaxed);
    mem::transmute::<FnRaw, super::ValidateUtf8Fn>(fun)(input)
}

// validate_utf8_compat() implementations

#[cfg(all(
    not(feature = "std"),
    not(target_feature = "avx2"),
    not(target_feature = "sse4.2")
))]
#[cfg_attr(not(feature = "no-inline"), inline)]
pub(super) unsafe fn validate_utf8_compat(input: &[u8]) -> Result<(), Utf8ErrorCompat> {
    super::validate_utf8_compat_fallback(input)
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
pub(super) unsafe fn validate_utf8_compat(input: &[u8]) -> Result<(), Utf8ErrorCompat> {
    sse42::validate_utf8_compat_simd(input)
}

#[cfg(target_feature = "avx2")]
pub(super) unsafe fn validate_utf8_compat(input: &[u8]) -> Result<(), Utf8ErrorCompat> {
    avx2::validate_utf8_compat_simd(input)
}

#[cfg(all(
    feature = "std",
    any(target_arch = "x86", target_arch = "x86_64"),
    not(target_feature = "avx2"),
    not(forcesse42)
))]
#[cfg_attr(not(feature = "no-inline"), inline)]
fn get_fastest_available_implementation_compat() -> super::ValidateUtf8CompatFn {
    if std::is_x86_feature_detected!("avx2") {
        avx2::validate_utf8_compat_simd
    } else if std::is_x86_feature_detected!("sse4.2") {
        sse42::validate_utf8_compat_simd
    } else {
        super::validate_utf8_compat_fallback
    }
}

#[cfg(all(feature = "std", not(target_feature = "avx2"), not(forcesse42)))]
#[cfg_attr(not(feature = "no-inline"), inline)]
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
