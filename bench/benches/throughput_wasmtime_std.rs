use simdutf8_bench::define_throughput_benchmark;

#[cfg(feature = "simdutf8_wasmtime")]
define_throughput_benchmark!(BenchFn::Wasmtime(WasmFn::Std));
