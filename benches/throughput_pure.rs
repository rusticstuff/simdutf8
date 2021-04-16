use std::time::Duration;

use criterion::measurement::Measurement;
use criterion::{criterion_group, criterion_main, Criterion};

mod common;
use common::*;

fn benchmark_fast<M: Measurement>(c: &mut Criterion<M>) {
    criterion_benchmark(c, BenchFn::Fast);
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10)).warm_up_time(Duration::from_secs(6)).sample_size(1000);
    targets = benchmark_fast
);

criterion_main!(benches);
