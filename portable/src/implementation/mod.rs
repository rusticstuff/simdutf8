//! Contains UTF-8 validation implementations.

pub(crate) mod helpers;

// UTF-8 validation function types
pub(crate) mod portable;

#[inline]
pub(crate) unsafe fn validate_utf8_basic(input: &[u8]) -> Result<(), crate::basic::Utf8Error> {
    if input.len() < helpers::SIMD_CHUNK_SIZE {
        return validate_utf8_basic_fallback(input);
    }

    validate_utf8_basic_simd(input)
}

#[inline(never)]
unsafe fn validate_utf8_basic_simd(input: &[u8]) -> Result<(), crate::basic::Utf8Error> {
    #[cfg(not(feature = "simd256"))]
    return portable::algorithm_safe::validate_utf8_basic(input);
    #[cfg(feature = "simd256")]
    return portable::simd256::validate_utf8_basic(input);
}

#[inline]
pub(crate) unsafe fn validate_utf8_compat(input: &[u8]) -> Result<(), crate::compat::Utf8Error> {
    if input.len() < helpers::SIMD_CHUNK_SIZE {
        return validate_utf8_compat_fallback(input);
    }

    validate_utf8_compat_simd(input)
}

unsafe fn validate_utf8_compat_simd(input: &[u8]) -> Result<(), crate::compat::Utf8Error> {
    #[cfg(not(feature = "simd256"))]
    return portable::algorithm_safe::validate_utf8_compat(input);
    #[cfg(feature = "simd256")]
    return portable::simd256::validate_utf8_compat(input);
}

// fallback method implementations
#[inline]
pub(crate) fn validate_utf8_basic_fallback(input: &[u8]) -> Result<(), crate::basic::Utf8Error> {
    match core::str::from_utf8(input) {
        Ok(_) => Ok(()),
        Err(_) => Err(crate::basic::Utf8Error {}),
    }
}

#[inline]
pub(crate) fn validate_utf8_compat_fallback(input: &[u8]) -> Result<(), crate::compat::Utf8Error> {
    helpers::validate_utf8_at_offset(input, 0)
}
