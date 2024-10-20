#[cfg(any(not(feature = "simd256"), feature = "public_imp"))]
pub(crate) mod simd128;
#[cfg(any(feature = "simd256", feature = "public_imp"))]
pub(crate) mod simd256;
