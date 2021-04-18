use simdutf8_bench::define_cpb_benchmark;

#[cfg(feature = "simdjson")]
define_cpb_benchmark!(BenchFn::Simdjson);
