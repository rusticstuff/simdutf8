use criterion::{measurement::Measurement, BenchmarkGroup, BenchmarkId, Criterion, Throughput};
use simdutf8::*;
use std::time::Duration;

fn get_valid_slice_of_len_or_more(s: &[u8], len: usize) -> &[u8] {
    for i in 0..4 {
        let res = &s[..len + i];
        if std::str::from_utf8(res).is_ok() {
            return res;
        }
    }
    panic!("Could not get valid slice of {} bytes", len);
}

fn bench<M: Measurement>(c: &mut Criterion<M>, name: &str, bytes: &[u8]) {
    let mut group = c.benchmark_group(name);
    group.warm_up_time(Duration::from_secs(6));
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(1000);
    for i in [1, 8, 64, 512, 4096, 65536].iter() {
        let slice = get_valid_slice_of_len_or_more(bytes, *i);
        bench_input(&mut group, slice, true, true);
    }
    group.finish();
}

fn bench_input<M: Measurement>(
    group: &mut BenchmarkGroup<M>,
    input: &[u8],
    with_throughput: bool,
    expected_ok: bool,
) {
    if with_throughput {
        group.throughput(Throughput::Bytes(input.len() as u64));
    }
    group.bench_with_input(
        BenchmarkId::from_parameter(format!("{:05}", input.len())),
        &input,
        |b, &slice| {
            b.iter(|| assert_eq!(from_utf8_exact(slice).is_ok(), expected_ok));
        },
    );
}

pub(crate) fn criterion_benchmark<M: Measurement>(c: &mut Criterion<M>) {
    let core_ids = core_affinity::get_core_ids().unwrap();
    core_affinity::set_for_current(*core_ids.get(2).unwrap_or(&core_ids[0]));

    let mut group = c.benchmark_group("0-empty");
    group.warm_up_time(Duration::from_secs(6));
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(1000);
    bench_input(&mut group, b"", false, true);
    group.finish();

    bench(
        c,
        "1-latin",
        include_str!("../text/Latin-Lipsum.txt").as_bytes(),
    );
    bench(
        c,
        "2-cyrillic",
        include_str!("../text/Russian-Lipsum.txt").as_bytes(),
    );
    bench(
        c,
        "3-chinese",
        include_str!("../text/Chinese-Lipsum.txt").as_bytes(),
    );
    bench(
        c,
        "4-emoji",
        include_str!("../text/Emoji-Lipsum.txt").as_bytes(),
    );

    let mut group = c.benchmark_group("x-error");
    group.warm_up_time(Duration::from_secs(6));
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(1000);
    bench_input(&mut group, b"\xFF".repeat(65536).as_slice(), false, false);
    group.finish();
}
