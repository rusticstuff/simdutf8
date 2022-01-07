//! Provides basic benchmark integration with WASM runtimes
//! (currently [Wasmer](https://wasmer.io/) and [Wasmtime](https://wasmtime.dev/)).

include!(concat!(env!("OUT_DIR"), "/wasm_shim.rs"));

pub(crate) const PAGE_SIZE: u32 = 0x10000;

/// Defines the API surface of the partial function that binds the SIMD UTF-8 validator
/// to a WASM runtime.  The partial application is the compilation state of the WASM module
/// plus the input to be validated (copied into the instance's linear memory).
pub trait WasmValidator {
    /// Compiles and binds some input into the webassembly module.
    fn new(input: &[u8]) -> Self;
    fn std_from_utf8(&mut self) -> bool;
    fn compat_from_utf8(&mut self) -> bool;
    fn basic_from_utf8(&mut self) -> bool;
}

#[cfg(feature = "simdutf8_wasmer")]
pub mod wasmer;

#[cfg(feature = "simdutf8_wasmtime")]
pub mod wasmtime;
