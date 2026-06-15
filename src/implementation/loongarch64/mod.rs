#[cfg(all(
    feature = "loongarch64_lsx",
    any(
        feature = "public_imp",
        // std: lsx is available for auto-selection unless lasx is selected at compile-time
        all(feature = "std", not(target_feature = "lasx")),
        // no-std: no lasx -> select lsx
        all(not(feature = "std"), not(target_feature = "lasx"), target_feature = "lsx")
    )
))]
pub(crate) mod lsx;

#[cfg(all(
    feature = "loongarch64_lsx",
    any(
        feature = "public_imp",
        // always available, except if no-std and no lasx support
        feature = "std",
        target_feature = "lasx"
    )
))]
pub(crate) mod lasx;

// validate_utf8_basic() std: implementation auto-selection

#[cfg(all(
    feature = "loongarch64_lsx",
    feature = "std",
    not(target_feature = "lasx")
))]
#[inline]
pub(crate) unsafe fn validate_utf8_basic(input: &[u8]) -> Result<(), crate::basic::Utf8Error> {
    use super::helpers::SIMD_CHUNK_SIZE;
    use core::mem;
    use std::sync::atomic::{AtomicPtr, Ordering};

    type FnRaw = *mut ();
    type ValidateUtf8Fn = unsafe fn(input: &[u8]) -> Result<(), crate::basic::Utf8Error>;

    #[flexpect::e(clippy::option_if_let_else)]
    #[inline]
    fn get_fastest_available_implementation_basic() -> ValidateUtf8Fn {
        if std::arch::is_loongarch_feature_detected!("lasx") {
            lasx::validate_utf8_basic
        } else if std::arch::is_loongarch_feature_detected!("lsx") {
            lsx::validate_utf8_basic
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

#[cfg(all(feature = "loongarch64_lsx", target_feature = "lasx"))]
pub(crate) unsafe fn validate_utf8_basic(
    input: &[u8],
) -> core::result::Result<(), crate::basic::Utf8Error> {
    validate_utf8_basic_lasx(input)
}

#[cfg(all(feature = "loongarch64_lsx", target_feature = "lasx"))]
#[inline(never)]
unsafe fn validate_utf8_basic_lasx(
    input: &[u8],
) -> core::result::Result<(), crate::basic::Utf8Error> {
    lasx::validate_utf8_basic(input)
}

#[cfg(all(
    feature = "loongarch64_lsx",
    not(feature = "std"),
    not(target_feature = "lasx"),
    target_feature = "lsx"
))]
#[inline]
pub(crate) unsafe fn validate_utf8_basic(
    input: &[u8],
) -> core::result::Result<(), crate::basic::Utf8Error> {
    if input.len() < super::helpers::SIMD_CHUNK_SIZE {
        return super::validate_utf8_basic_fallback(input);
    }

    validate_utf8_basic_lsx(input)
}

#[cfg(all(
    feature = "loongarch64_lsx",
    not(feature = "std"),
    not(target_feature = "lasx"),
    target_feature = "lsx"
))]
#[inline]
pub(crate) unsafe fn validate_utf8_basic_lsx(
    input: &[u8],
) -> core::result::Result<(), crate::basic::Utf8Error> {
    lsx::validate_utf8_basic(input)
}

#[cfg(any(
    not(feature = "loongarch64_lsx"),
    all(
        not(feature = "std"),
        not(target_feature = "lasx"),
        not(target_feature = "lsx"),
    )
))]
pub(crate) use super::validate_utf8_basic_fallback as validate_utf8_basic;

// validate_utf8_compat() std: implementation auto-selection

#[cfg(all(
    feature = "loongarch64_lsx",
    feature = "std",
    not(target_feature = "lasx")
))]
#[inline]
pub(crate) unsafe fn validate_utf8_compat(input: &[u8]) -> Result<(), crate::compat::Utf8Error> {
    use super::helpers::SIMD_CHUNK_SIZE;
    use core::mem;
    use std::sync::atomic::{AtomicPtr, Ordering};

    type FnRaw = *mut ();
    type ValidateUtf8CompatFn = unsafe fn(input: &[u8]) -> Result<(), crate::compat::Utf8Error>;

    #[flexpect::e(clippy::option_if_let_else)]
    #[inline]
    fn get_fastest_available_implementation_compat() -> ValidateUtf8CompatFn {
        if std::arch::is_loongarch_feature_detected!("lasx") {
            lasx::validate_utf8_compat
        } else if std::arch::is_loongarch_feature_detected!("lsx") {
            lsx::validate_utf8_compat
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

// validate_utf8_compat() no-std: implementation selection by config

#[cfg(all(feature = "loongarch64_lsx", target_feature = "lasx"))]
pub(crate) unsafe fn validate_utf8_compat(
    input: &[u8],
) -> core::result::Result<(), crate::compat::Utf8Error> {
    validate_utf8_compat_lasx(input)
}

#[cfg(all(feature = "loongarch64_lsx", target_feature = "lasx"))]
#[inline(never)]
unsafe fn validate_utf8_compat_lasx(
    input: &[u8],
) -> core::result::Result<(), crate::compat::Utf8Error> {
    lasx::validate_utf8_compat(input)
}

#[cfg(all(
    feature = "loongarch64_lsx",
    not(feature = "std"),
    not(target_feature = "lasx"),
    target_feature = "lsx"
))]
#[inline]
pub(crate) unsafe fn validate_utf8_compat(
    input: &[u8],
) -> core::result::Result<(), crate::compat::Utf8Error> {
    if input.len() < super::helpers::SIMD_CHUNK_SIZE {
        return super::validate_utf8_compat_fallback(input);
    }

    validate_utf8_compat_lsx(input)
}

#[cfg(all(
    feature = "loongarch64_lsx",
    not(feature = "std"),
    not(target_feature = "lasx"),
    target_feature = "lsx"
))]
#[inline]
pub(crate) unsafe fn validate_utf8_compat_lsx(
    input: &[u8],
) -> core::result::Result<(), crate::compat::Utf8Error> {
    lsx::validate_utf8_compat(input)
}

#[cfg(any(
    not(feature = "loongarch64_lsx"),
    all(
        not(feature = "std"),
        not(target_feature = "lasx"),
        not(target_feature = "lsx"),
    )
))]
pub(crate) use super::validate_utf8_compat_fallback as validate_utf8_compat;
