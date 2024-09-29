pub(crate) mod portable;

#[cfg(any(
    feature = "portable_override",
    all(
        feature = "portable_fallback",
        not(any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "wasm32"
        )),
    )
))]
#[inline]
pub(crate) unsafe fn validate_utf8_basic(input: &[u8]) -> Result<(), crate::basic::Utf8Error> {
    if input.len() < super::helpers::SIMD_CHUNK_SIZE {
        return super::validate_utf8_basic_fallback(input);
    }

    validate_utf8_basic_portable(input)
}

/// This function definition is only needed to make sure that it is never inlined.
#[cfg(any(
    feature = "portable_override",
    all(
        feature = "portable_fallback",
        not(any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "wasm32"
        )),
    )
))]
#[inline(never)]
unsafe fn validate_utf8_basic_portable(input: &[u8]) -> Result<(), crate::basic::Utf8Error> {
    portable::validate_utf8_basic(input)
}

#[cfg(any(
    feature = "portable_override",
    all(
        feature = "portable_fallback",
        not(any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "wasm32"
        )),
    )
))]
#[inline]
pub(crate) unsafe fn validate_utf8_compat(input: &[u8]) -> Result<(), crate::compat::Utf8Error> {
    if input.len() < super::helpers::SIMD_CHUNK_SIZE {
        return super::validate_utf8_compat_fallback(input);
    }

    validate_utf8_compat_portable(input)
}

/// This function definition is only needed to make sure that it is never inlined.
#[cfg(any(
    feature = "portable_override",
    all(
        feature = "portable_fallback",
        not(any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "wasm32"
        )),
    )
))]
#[inline(never)]
unsafe fn validate_utf8_compat_portable(input: &[u8]) -> Result<(), crate::compat::Utf8Error> {
    portable::validate_utf8_compat(input)
}
