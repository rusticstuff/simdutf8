use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use simdutf8::*;

fn get_valid_slice_of_len_or_more(s: &[u8], len: usize) -> &[u8] {
    for i in 0..4 {
        let res = &s[..len + i];
        if std::str::from_utf8(res).is_ok() {
            return res;
        }
    }
    panic!("Could not get valid slice of {} bytes", len);
}

fn bench(c: &mut Criterion, name: &str, bytes: &[u8]) {
    let mut group = c.benchmark_group(name);
    for i in [1, 8, 64, 512, 4096].iter() {
        let slice = get_valid_slice_of_len_or_more(bytes, *i);
        group.throughput(Throughput::Bytes(slice.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("simd", format!("{:05}", slice.len())),
            &slice,
            |b, &slice| {
                b.iter(|| validate_utf8(slice).unwrap());
            },
        );
        group.bench_with_input(
            BenchmarkId::new("std", format!("{:05}", slice.len())),
            &slice,
            |b, &slice| {
                b.iter(|| std::str::from_utf8(slice).unwrap());
            },
        );
    }
    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    let core_ids = core_affinity::get_core_ids().unwrap();
    core_affinity::set_for_current(*core_ids.get(2).unwrap_or(&core_ids[0]));
    bench(
        c,
        "1-latin",
        include_str!("text/Latin-Lipsum.txt").as_bytes(),
    );
    bench(
        c,
        "2-cyrillic",
        include_str!("text/Russian-Lipsum.txt").as_bytes(),
    );
    bench(
        c,
        "3-chinese",
        include_str!("text/Chinese-Lipsum.txt").as_bytes(),
    );
    bench(
        c,
        "4-emoij",
        include_str!("text/Emoji-Lipsum.txt").as_bytes(),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
