//! Contains UTF-8 validation implementations.

#![forbid(unsafe_code)]

pub(crate) mod fallback;

#[allow(unused)]
pub(crate) mod simd;

cfg_if::cfg_if! {
    if #[cfg(feature = "force_fallback")] {
        pub(crate) use fallback as auto;
    } else if #[cfg(feature = "force_simd128")] {
            pub(crate) use simd::v128 as auto;
    } else if #[cfg(feature = "force_simd256")] {
        pub(crate) use simd::v256 as auto;
    // known good configurations
    } else if #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "avx2"
    ))] {
        pub(crate) use simd::v256 as auto;
    } else if #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse4.2"
    ))] {
        pub(crate) use simd::v128 as auto;
    } else if #[cfg(all(
        target_arch = "aarch64",
        target_feature = "neon"
    ))] {
        pub(crate) use simd::v128 as auto;
    } else if #[cfg(all(
        target_arch = "arm",
        target_endian = "little",
        target_feature = "v7",
        target_feature = "neon"
    ))] {
        pub(crate) use simd::v128 as auto;
    } else {
        pub(crate) use fallback as auto;
    }
}

#[inline]
pub(crate) fn validate_utf8_basic(input: &[u8]) -> Result<(), crate::basic::Utf8Error> {
    if input.len() < simd::SIMD_CHUNK_SIZE {
        return fallback::validate_utf8_basic(input);
    }

    validate_utf8_basic_simd(input)
}

#[inline(never)]
#[allow(clippy::missing_const_for_fn)]
fn validate_utf8_basic_simd(input: &[u8]) -> Result<(), crate::basic::Utf8Error> {
    auto::validate_utf8_basic(input)
}

#[inline]
pub(crate) fn validate_utf8_compat(input: &[u8]) -> Result<(), crate::compat::Utf8Error> {
    if input.len() < simd::SIMD_CHUNK_SIZE {
        return fallback::validate_utf8_compat(input);
    }

    validate_utf8_compat_simd(input)
}

fn validate_utf8_compat_simd(input: &[u8]) -> Result<(), crate::compat::Utf8Error> {
    auto::validate_utf8_compat(input)
}

type Utf8ErrorCompat = crate::compat::Utf8Error;

#[inline]
#[expect(clippy::cast_possible_truncation)]
fn validate_utf8_at_offset(input: &[u8], offset: usize) -> Result<(), Utf8ErrorCompat> {
    match core::str::from_utf8(&input[offset..]) {
        Ok(_) => Ok(()),
        Err(err) => Err(Utf8ErrorCompat {
            valid_up_to: err.valid_up_to() + offset,
            error_len: err.error_len().map(|len| {
                // never truncates since std::str::err::Utf8Error::error_len() never returns value larger than 4
                len as u8
            }),
        }),
    }
}
