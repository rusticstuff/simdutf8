use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

mod common;

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10)).warm_up_time(Duration::from_secs(6)).sample_size(1000);
    targets = common::criterion_benchmark
);

criterion_main!(benches);
