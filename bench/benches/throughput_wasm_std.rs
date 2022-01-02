use simdutf8_bench::define_throughput_benchmark;

#[cfg(feature = "simdutf8_wasm")]
define_throughput_benchmark!(BenchFn::Wasm(WasmFn::Std));
