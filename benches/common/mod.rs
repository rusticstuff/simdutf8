use criterion::{measurement::Measurement, BenchmarkGroup, BenchmarkId, Criterion, Throughput};
use simdutf8::compat::from_utf8 as compat_from_utf8;
use simdutf8::pure::from_utf8 as pure_from_utf8;
use std::time::Duration;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(super) enum BenchFn {
    Fast,
    Compat,
}

fn scale_to_one_mib(input: &[u8]) -> Vec<u8> {
    input.repeat((1024 * 1024 + input.len() - 1) / input.len())
}

pub(super) fn criterion_benchmark<M: Measurement>(c: &mut Criterion<M>, bench_fn: BenchFn) {
    let core_ids = core_affinity::get_core_ids().unwrap();
    core_affinity::set_for_current(*core_ids.get(2).unwrap_or(&core_ids[0]));

    let mut group = c.benchmark_group("0-empty");
    bench_input(&mut group, b"", false, true, bench_fn);
    group.finish();

    bench(
        c,
        "1-latin",
        &scale_to_one_mib(include_bytes!("../text/Latin-Lipsum.txt")),
        bench_fn,
    );
    bench(
        c,
        "2-cyrillic",
        &scale_to_one_mib(include_bytes!("../text/Russian-Lipsum.txt")),
        bench_fn,
    );
    bench(
        c,
        "3-chinese",
        &scale_to_one_mib(include_bytes!("../text/Chinese-Lipsum.txt")),
        bench_fn,
    );
    bench(
        c,
        "4-emoji",
        &scale_to_one_mib(include_bytes!("../text/Emoji-Lipsum.txt")),
        bench_fn,
    );

    let mut group = c.benchmark_group("x-error");
    group.warm_up_time(Duration::from_secs(6));
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(1000);
    bench_input(
        &mut group,
        b"\xFF".repeat(65536).as_slice(),
        false,
        false,
        bench_fn,
    );
    group.finish();
}

fn get_valid_slice_of_len_or_more(s: &[u8], len: usize) -> &[u8] {
    for i in 0..4 {
        let res = &s[..len + i];
        if std::str::from_utf8(res).is_ok() {
            return res;
        }
    }
    panic!("Could not get valid slice of {} bytes", len);
}

fn bench<M: Measurement>(c: &mut Criterion<M>, name: &str, bytes: &[u8], bench_fn: BenchFn) {
    let mut group = c.benchmark_group(name);
    group.warm_up_time(Duration::from_secs(6));
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(1000);
    for i in [1, 8, 64, 512, 4096, 65536, 131072].iter() {
        let slice = get_valid_slice_of_len_or_more(bytes, *i);
        bench_input(&mut group, slice, true, true, bench_fn);
    }
    group.finish();
}

fn bench_input<M: Measurement>(
    group: &mut BenchmarkGroup<M>,
    input: &[u8],
    with_throughput: bool,
    expected_ok: bool,
    bench_fn: BenchFn,
) {
    if with_throughput {
        group.throughput(Throughput::Bytes(input.len() as u64));
    }
    match bench_fn {
        BenchFn::Fast => {
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{:05}", input.len())),
                &input,
                |b, &slice| {
                    b.iter(|| assert_eq!(pure_from_utf8(slice).is_ok(), expected_ok));
                },
            );
        }
        BenchFn::Compat => {
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{:05}", input.len())),
                &input,
                |b, &slice| {
                    b.iter(|| assert_eq!(compat_from_utf8(slice).is_ok(), expected_ok));
                },
            );
        }
    }
}
