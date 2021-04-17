#[allow(unused_macros)]
macro_rules! define_throughput_benchmark {
    ($bench_fn:expr) => {
        use std::time::Duration;

        use criterion::measurement::Measurement;
        use criterion::{criterion_group, criterion_main, Criterion};

        use common::*;

        fn benchmark_compat<M: Measurement>(c: &mut Criterion<M>) {
            criterion_benchmark(c, $bench_fn);
        }

        criterion_group!(
            name = benches;
            config = Criterion::default().measurement_time(Duration::from_secs(10)).warm_up_time(Duration::from_secs(6)).sample_size(1000);
            targets = benchmark_compat
        );

        criterion_main!(benches);
    };
}

#[allow(unused_macros)]
macro_rules! define_cpb_benchmark {
    ($bench_fn:expr) => {
        use std::time::Duration;

        use criterion::measurement::Measurement;
        use criterion::{criterion_group, criterion_main, Criterion};
        use criterion_cycles_per_byte::CyclesPerByte;

        use common::*;

        fn benchmark_fast<M: Measurement>(c: &mut Criterion<M>) {
            criterion_benchmark(c, $bench_fn);
        }

        criterion_group!(
            name = benches;
            config = Criterion::default().with_measurement(CyclesPerByte).measurement_time(Duration::from_secs(10)).warm_up_time(Duration::from_secs(6)).sample_size(1000);
            targets = benchmark_fast
        );

        criterion_main!(benches);
    };
}
