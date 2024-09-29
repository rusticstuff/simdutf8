#[cfg(any(feature = "std", feature = "public_imp", target_feature = "avx2"))]
pub(crate) mod avx2;

#[cfg(any(
    feature = "public_imp",
    all(feature = "std", not(target_feature = "avx2")),
    all(
        not(feature = "std"),
        not(target_feature = "avx2"),
        target_feature = "sse4.2"
    )
))]
pub(crate) mod sse42;

// validate_utf8_basic() std: implementation auto-selection

#[cfg(all(
    feature = "std",
    not(target_feature = "avx2"),
    not(feature = "portable_override")
))]
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
        if std::is_x86_feature_detected!("avx2") {
            avx2::validate_utf8_basic
        } else if std::is_x86_feature_detected!("sse4.2") {
            sse42::validate_utf8_basic
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

#[cfg(all(target_feature = "avx2", not(feature = "portable_override")))]
pub(crate) unsafe fn validate_utf8_basic(
    input: &[u8],
) -> core::result::Result<(), crate::basic::Utf8Error> {
    if input.len() < super::helpers::SIMD_CHUNK_SIZE {
        return super::validate_utf8_basic_fallback(input);
    }

    validate_utf8_basic_avx2(input)
}

/// This function definition is only needed to make sure that it is never inlined.
#[cfg(all(target_feature = "avx2", not(feature = "portable_override")))]
#[inline(never)]
unsafe fn validate_utf8_basic_avx2(
    input: &[u8],
) -> core::result::Result<(), crate::basic::Utf8Error> {
    avx2::validate_utf8_basic(input)
}

#[cfg(all(
    not(feature = "std"),
    not(target_feature = "avx2"),
    target_feature = "sse4.2",
    not(feature = "portable_override")
))]
pub(crate) unsafe fn validate_utf8_basic(
    input: &[u8],
) -> core::result::Result<(), crate::basic::Utf8Error> {
    if input.len() < super::helpers::SIMD_CHUNK_SIZE {
        return super::validate_utf8_basic_fallback(input);
    }

    validate_utf8_basic_sse42(input)
}

/// This function definition is only needed to make sure that it is never inlined.
#[cfg(all(
    not(feature = "std"),
    not(target_feature = "avx2"),
    target_feature = "sse4.2",
    not(feature = "portable_override")
))]
#[inline(never)]
unsafe fn validate_utf8_basic_sse42(
    input: &[u8],
) -> core::result::Result<(), crate::basic::Utf8Error> {
    sse42::validate_utf8_basic(input)
}

#[cfg(all(
    not(feature = "std"),
    not(target_feature = "avx2"),
    not(target_feature = "sse4.2")
))]
pub(crate) use super::validate_utf8_basic_fallback as validate_utf8_basic;

// validate_utf8_compat() std: implementation auto-selection

#[cfg(all(
    feature = "std",
    not(target_feature = "avx2"),
    not(feature = "portable_override")
))]
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
        if std::is_x86_feature_detected!("avx2") {
            avx2::validate_utf8_compat
        } else if std::is_x86_feature_detected!("sse4.2") {
            sse42::validate_utf8_compat
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

#[cfg(target_feature = "avx2")]
pub(crate) unsafe fn validate_utf8_compat(
    input: &[u8],
) -> core::result::Result<(), crate::compat::Utf8Error> {
    if input.len() < super::helpers::SIMD_CHUNK_SIZE {
        return super::validate_utf8_compat_fallback(input);
    }

    validate_utf8_compat_avx2(input)
}

/// This function definition is only needed to make sure that it is never inlined.
#[cfg(target_feature = "avx2")]
#[inline(never)]
unsafe fn validate_utf8_compat_avx2(
    input: &[u8],
) -> core::result::Result<(), crate::compat::Utf8Error> {
    avx2::validate_utf8_compat(input)
}

#[cfg(all(
    not(feature = "std"),
    not(target_feature = "avx2"),
    target_feature = "sse4.2"
))]
pub(crate) unsafe fn validate_utf8_compat(
    input: &[u8],
) -> core::result::Result<(), crate::compat::Utf8Error> {
    if input.len() < super::helpers::SIMD_CHUNK_SIZE {
        return super::validate_utf8_compat_fallback(input);
    }

    validate_utf8_compat_sse42(input)
}

/// This function definition is only needed to make sure that it is never inlined.
#[cfg(all(
    not(feature = "std"),
    not(target_feature = "avx2"),
    target_feature = "sse4.2"
))]
#[inline(never)]
pub(crate) unsafe fn validate_utf8_compat_sse42(
    input: &[u8],
) -> core::result::Result<(), crate::compat::Utf8Error> {
    sse42::validate_utf8_compat(input)
}

#[cfg(all(
    not(feature = "std"),
    not(target_feature = "avx2"),
    not(target_feature = "sse4.2")
))]
pub(crate) use super::validate_utf8_compat_fallback as validate_utf8_compat;
