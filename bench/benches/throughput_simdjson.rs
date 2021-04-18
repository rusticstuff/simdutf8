use simdutf8_bench::define_throughput_benchmark;

#[cfg(feature = "simdjson")]
define_throughput_benchmark!(BenchFn::Simdjson);
