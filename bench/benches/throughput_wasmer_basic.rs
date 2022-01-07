use simdutf8_bench::define_throughput_benchmark;

#[cfg(feature = "simdutf8_wasmer")]
define_throughput_benchmark!(BenchFn::Wasmer(WasmFn::Basic));
