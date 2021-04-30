#[cfg(feature = "aarch64")]
pub(crate) mod neon;

#[inline]
#[cfg(feature = "aarch64")]
pub(crate) unsafe fn validate_utf8_basic(input: &[u8]) -> Result<(), crate::basic::Utf8Error> {
    if input.len() < super::helpers::SIMD_CHUNK_SIZE {
        return super::validate_utf8_basic_fallback(input);
    }

    neon::validate_utf8_basic(input)
}

#[cfg(not(feature = "aarch64"))]
pub(crate) use super::validate_utf8_basic_fallback as validate_utf8_basic;

#[inline]
#[cfg(feature = "aarch64")]
pub(crate) unsafe fn validate_utf8_compat(input: &[u8]) -> Result<(), crate::compat::Utf8Error> {
    if input.len() < super::helpers::SIMD_CHUNK_SIZE {
        return super::validate_utf8_compat_fallback(input);
    }

    neon::validate_utf8_compat(input)
}

#[cfg(not(feature = "aarch64"))]
pub(crate) use super::validate_utf8_compat_fallback as validate_utf8_compat;
