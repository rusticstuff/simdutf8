use std::time::Duration;

use criterion::measurement::Measurement;
use criterion::{criterion_group, criterion_main, Criterion};
use criterion_cycles_per_byte::CyclesPerByte;

mod common;
use common::*;

fn benchmark_exact<M: Measurement>(c: &mut Criterion<M>) {
    criterion_benchmark(c, BenchFn::Exact);
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(CyclesPerByte).measurement_time(Duration::from_secs(10)).warm_up_time(Duration::from_secs(6)).sample_size(1000);
    targets = benchmark_exact
);

criterion_main!(benches);
