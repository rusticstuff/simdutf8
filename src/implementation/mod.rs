//! Contains UTF-8 validation implementations.

#[macro_use]
#[allow(unused_macros)] // only used if there is a SIMD implementation
mod algorithm;

pub(crate) mod helpers;

// UTF-8 validation function types

// x86 implementation

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    any(not(feature = "portable_override"), feature = "public_imp")
))]
pub(crate) mod x86;

/// Fn needed instead of re-import, otherwise not inlined in non-std case
#[flexpect::e(clippy::inline_always)]
#[inline(always)]
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    not(feature = "portable_override")
))]
pub(super) unsafe fn validate_utf8_basic(input: &[u8]) -> Result<(), crate::basic::Utf8Error> {
    x86::validate_utf8_basic(input)
}

/// Fn needed instead of re-import, otherwise not inlined in non-std case
#[flexpect::e(clippy::inline_always)]
#[inline(always)]
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    not(feature = "portable_override")
))]
pub(super) unsafe fn validate_utf8_compat(input: &[u8]) -> Result<(), crate::compat::Utf8Error> {
    x86::validate_utf8_compat(input)
}

// aarch64 implementation

#[cfg(all(
    target_arch = "aarch64",
    any(not(feature = "portable_override"), feature = "public_imp")
))]
pub(crate) mod aarch64;

#[cfg(all(target_arch = "aarch64", not(feature = "portable_override")))]
pub(super) use aarch64::validate_utf8_basic;

#[cfg(all(target_arch = "aarch64", not(feature = "portable_override")))]
pub(super) use aarch64::validate_utf8_compat;

// wasm32 implementation

#[cfg(all(target_arch = "wasm32", not(feature = "portable_override")))]
pub(crate) mod wasm32;

#[cfg(all(target_arch = "wasm32", not(feature = "portable_override")))]
pub(super) use wasm32::validate_utf8_basic;

#[cfg(all(target_arch = "wasm32", not(feature = "portable_override")))]
pub(super) use wasm32::validate_utf8_compat;

// fallback for unsupported architectures

#[cfg(all(
    not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "wasm32"
    )),
    not(feature = "portable_override"),
    not(feature = "portable_fallback")
))]
pub(super) use validate_utf8_basic_fallback as validate_utf8_basic;

#[cfg(all(
    not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "wasm32"
    )),
    not(feature = "portable_override"),
    not(feature = "portable_fallback")
))]
pub(super) use validate_utf8_compat_fallback as validate_utf8_compat;

// portable SIMD implementation

#[cfg(any(
    feature = "portable_public_imp",
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
pub(super) use portable::validate_utf8_basic;

#[cfg(any(
    feature = "portable_override",
    all(
        feature = "portable_fallback",
        not(any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "wasm32"
        ))
    )
))]
pub(super) use portable::validate_utf8_compat;

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
