use criterion::{criterion_group, criterion_main, Criterion};

mod common;

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = common::criterion_benchmark
);
criterion_main!(benches);
