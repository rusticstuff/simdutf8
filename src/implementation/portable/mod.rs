#[cfg(any(
    feature = "portable_public_imp",
    all(
        feature = "portable_override",
        any(
            not(any(target_arch = "x86", target_arch = "x86_64")),
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                not(target_feature = "avx2")
            )
        ),
    ),
))]
pub(crate) mod simd128;

#[cfg(any(
    feature = "portable_public_imp",
    all(
        feature = "portable_override",
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "avx2"
    ),
))]
pub(crate) mod simd256;

#[cfg(feature = "portable_override")]
#[inline]
pub(crate) unsafe fn validate_utf8_basic(input: &[u8]) -> Result<(), crate::basic::Utf8Error> {
    if input.len() < super::helpers::SIMD_CHUNK_SIZE {
        return super::validate_utf8_basic_fallback(input);
    }

    validate_utf8_basic_portable(input)
}

/// This function definition is only needed to make sure that it is never inlined.
#[cfg(all(
    feature = "portable_override",
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2"
))]
#[inline(never)]
unsafe fn validate_utf8_basic_portable(input: &[u8]) -> Result<(), crate::basic::Utf8Error> {
    simd256::validate_utf8_basic(input)
}

/// This function definition is only needed to make sure that it is never inlined.
#[cfg(all(
    feature = "portable_override",
    not(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "avx2"
    ))
))]
#[inline(never)]
unsafe fn validate_utf8_basic_portable(input: &[u8]) -> Result<(), crate::basic::Utf8Error> {
    simd128::validate_utf8_basic(input)
}

#[cfg(feature = "portable_override")]
#[inline]
pub(crate) unsafe fn validate_utf8_compat(input: &[u8]) -> Result<(), crate::compat::Utf8Error> {
    if input.len() < super::helpers::SIMD_CHUNK_SIZE {
        return super::validate_utf8_compat_fallback(input);
    }

    validate_utf8_compat_portable(input)
}

/// This function definition is only needed to make sure that it is never inlined.
#[cfg(all(
    feature = "portable_override",
    not(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "avx2"
    ))
))]
#[inline(never)]
unsafe fn validate_utf8_compat_portable(input: &[u8]) -> Result<(), crate::compat::Utf8Error> {
    simd128::validate_utf8_compat(input)
}

/// This function definition is only needed to make sure that it is never inlined.
#[cfg(all(
    feature = "portable_override",
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2"
))]
#[inline(never)]
unsafe fn validate_utf8_compat_portable(input: &[u8]) -> Result<(), crate::compat::Utf8Error> {
    simd256::validate_utf8_compat(input)
}
