#[cfg(all(
    feature = "armv7_neon",
    any(feature = "std", feature = "public_imp", target_feature = "neon")
))]
pub(crate) mod neon;

// validate_utf8_basic() std: implementation auto-selection

#[cfg(all(feature = "armv7_neon", feature = "std", not(target_feature = "neon")))]
#[inline]
pub(crate) unsafe fn validate_utf8_basic(
    input: &[u8],
) -> core::result::Result<(), crate::basic::Utf8Error> {
    use super::helpers::SIMD_CHUNK_SIZE;
    use core::mem;
    use std::sync::atomic::{AtomicPtr, Ordering};

    type FnRaw = *mut ();
    type ValidateUtf8Fn = unsafe fn(input: &[u8]) -> Result<(), crate::basic::Utf8Error>;

    #[inline]
    fn get_fastest_available_implementation_basic() -> ValidateUtf8Fn {
        if std::arch::is_arm_feature_detected!("neon") {
            neon::validate_utf8_basic
        } else {
            super::validate_utf8_basic_fallback
        }
    }

    static FN: AtomicPtr<()> = AtomicPtr::new(get_fastest as FnRaw);

    unsafe fn get_fastest(input: &[u8]) -> core::result::Result<(), crate::basic::Utf8Error> {
        let fun = get_fastest_available_implementation_basic();
        FN.store(fun as FnRaw, Ordering::Relaxed);
        (fun)(input)
    }

    if input.len() < SIMD_CHUNK_SIZE {
        return super::validate_utf8_basic_fallback(input);
    }

    let fun = FN.load(Ordering::Relaxed);
    mem::transmute::<FnRaw, ValidateUtf8Fn>(fun)(input)
}

// validate_utf8_basic() no-std: implementation selection by config

#[cfg(all(feature = "armv7_neon", target_feature = "neon"))]
#[inline]
pub(crate) unsafe fn validate_utf8_basic(
    input: &[u8],
) -> core::result::Result<(), crate::basic::Utf8Error> {
    if input.len() < super::helpers::SIMD_CHUNK_SIZE {
        return super::validate_utf8_basic_fallback(input);
    }

    validate_utf8_basic_neon(input)
}

#[cfg(all(feature = "armv7_neon", target_feature = "neon"))]
#[inline(never)]
unsafe fn validate_utf8_basic_neon(
    input: &[u8],
) -> core::result::Result<(), crate::basic::Utf8Error> {
    neon::validate_utf8_basic(input)
}

#[cfg(any(
    not(feature = "armv7_neon"),
    all(not(feature = "std"), not(target_feature = "neon"))
))]
pub(crate) use super::validate_utf8_basic_fallback as validate_utf8_basic;

// validate_utf8_compat() std: implementation auto-selection

#[cfg(all(feature = "armv7_neon", feature = "std", not(target_feature = "neon")))]
#[inline]
pub(crate) unsafe fn validate_utf8_compat(
    input: &[u8],
) -> core::result::Result<(), crate::compat::Utf8Error> {
    use super::helpers::SIMD_CHUNK_SIZE;
    use core::mem;
    use std::sync::atomic::{AtomicPtr, Ordering};

    type FnRaw = *mut ();
    type ValidateUtf8CompatFn = unsafe fn(input: &[u8]) -> Result<(), crate::compat::Utf8Error>;

    #[inline]
    fn get_fastest_available_implementation_compat() -> ValidateUtf8CompatFn {
        if std::arch::is_arm_feature_detected!("neon") {
            neon::validate_utf8_compat
        } else {
            super::validate_utf8_compat_fallback
        }
    }

    static FN: AtomicPtr<()> = AtomicPtr::new(get_fastest as FnRaw);

    unsafe fn get_fastest(input: &[u8]) -> core::result::Result<(), crate::compat::Utf8Error> {
        let fun = get_fastest_available_implementation_compat();
        FN.store(fun as FnRaw, Ordering::Relaxed);
        (fun)(input)
    }

    if input.len() < SIMD_CHUNK_SIZE {
        return super::validate_utf8_compat_fallback(input);
    }

    let fun = FN.load(Ordering::Relaxed);
    mem::transmute::<FnRaw, ValidateUtf8CompatFn>(fun)(input)
}

// validate_utf8_basic() no-std: implementation selection by config

#[cfg(all(feature = "armv7_neon", target_feature = "neon"))]
#[inline]
pub(crate) unsafe fn validate_utf8_compat(
    input: &[u8],
) -> core::result::Result<(), crate::compat::Utf8Error> {
    if input.len() < super::helpers::SIMD_CHUNK_SIZE {
        return super::validate_utf8_compat_fallback(input);
    }

    validate_utf8_compat_neon(input)
}

#[cfg(all(feature = "armv7_neon", target_feature = "neon"))]
#[inline(never)]
unsafe fn validate_utf8_compat_neon(
    input: &[u8],
) -> core::result::Result<(), crate::compat::Utf8Error> {
    neon::validate_utf8_compat(input)
}

#[cfg(any(
    not(feature = "armv7_neon"),
    all(not(feature = "std"), not(target_feature = "neon"))
))]
pub(crate) use super::validate_utf8_compat_fallback as validate_utf8_compat;
