#[macro_export]
macro_rules! define_throughput_benchmark {
    ($bench_fn:expr) => {
        use std::time::Duration;

        use criterion::measurement::Measurement;
        use criterion::{criterion_group, criterion_main, Criterion};

        use simdutf8_bench::*;

        fn benchmark_compat<M: Measurement>(c: &mut Criterion<M>) {
            criterion_benchmark(c, $bench_fn);
        }

        criterion_group!(
            name = benches;
            config = Criterion::default().measurement_time(Duration::from_secs(1)).warm_up_time(Duration::from_secs(1)).sample_size(300);
            targets = benchmark_compat
        );

        criterion_main!(benches);
    };
}
