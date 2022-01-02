# WASM Shim for simdutf8

This is a simple static library crate to export the simd API as a simple
C ABI that can be built into a WASM module and embedded in native benchmarks
(hosting a WASM runtime).
