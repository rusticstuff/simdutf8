use criterion::{measurement::Measurement, BenchmarkGroup, BenchmarkId, Criterion, Throughput};
use simdutf8::basic::from_utf8 as basic_from_utf8;
use simdutf8::compat::from_utf8 as compat_from_utf8;

use std::str::from_utf8 as std_from_utf8;

#[cfg(feature = "simdjson")]
use simdjson_utf8::validate as simdjson_validate;

#[macro_use]
mod macros;

#[cfg(feature = "simdutf8_wasm")]
mod wasm;

#[derive(Clone, Copy)]
pub enum WasmFn {
    Basic,
    Compat,
    Std,
}

#[derive(Clone, Copy)]
pub enum BenchFn {
    Basic,
    BasicNoInline,
    Compat,
    Std,

    #[cfg(feature = "simdjson")]
    Simdjson,

    #[cfg(feature = "simdutf8_wasm")]
    Wasm(WasmFn),
}

#[derive(Clone, Copy)]
struct Alignment {
    boundary: usize,
    offset: usize,
}

pub fn criterion_benchmark<M: Measurement>(c: &mut Criterion<M>, bench_fn: BenchFn) {
    let core_ids = core_affinity::get_core_ids().unwrap();
    core_affinity::set_for_current(*core_ids.get(2).unwrap_or(&core_ids[0]));

    bench_empty(c, bench_fn);

    bench(
        c,
        "1-latin",
        &scale_to_one_mib(include_bytes!("../data/Latin-Lipsum.txt")),
        bench_fn,
    );

    bench(
        c,
        "2-cyrillic",
        &scale_to_one_mib(include_bytes!("../data/Russian-Lipsum.txt")),
        bench_fn,
    );
    bench(
        c,
        "3-chinese",
        &scale_to_one_mib(include_bytes!("../data/Chinese-Lipsum.txt")),
        bench_fn,
    );
    bench(
        c,
        "4-emoji",
        &scale_to_one_mib(include_bytes!("../data/Emoji-Lipsum.txt")),
        bench_fn,
    );

    bench_late_error(c, bench_fn);
}

fn bench_empty<M: Measurement>(c: &mut Criterion<M>, bench_fn: BenchFn) {
    let mut group = c.benchmark_group("0-empty");
    bench_input(&mut group, b"", false, true, bench_fn);
    group.finish();
}

fn bench_late_error<M: Measurement>(c: &mut Criterion<M>, bench_fn: BenchFn) {
    let mut group = c.benchmark_group("x-error");
    bench_input(
        &mut group,
        b"\xFF".repeat(65536).as_slice(),
        false,
        false,
        bench_fn,
    );
    group.finish();
}

fn scale_to_one_mib(input: &[u8]) -> Vec<u8> {
    input.repeat((1024 * 1024 + input.len() - 1) / input.len())
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
fn get_valid_slice_of_len_or_more_aligned(
    s: &[u8],
    len: usize,
    alignment: Alignment,
) -> (Vec<u8>, usize) {
    let valid_utf8 = get_valid_slice_of_len_or_more(s, len);
    let mut vec = Vec::with_capacity(len + alignment.boundary);
    let cur_off = (vec.as_ptr() as usize) % alignment.boundary;
    let padding = if cur_off == alignment.offset {
        0
    } else {
        (alignment.offset + alignment.boundary - cur_off) % alignment.boundary
    };
    vec.resize(padding, 0);
    vec.extend_from_slice(valid_utf8);
    (vec, padding)
}

fn bench<M: Measurement>(c: &mut Criterion<M>, name: &str, bytes: &[u8], bench_fn: BenchFn) {
    let mut group = c.benchmark_group(name);
    for i in [1, 8, 64, 512, 4096, 65536, 131072].iter() {
        let alignment = Alignment {
            boundary: 64,
            offset: 8, // 8 is the default alignment on 64-bit, so this is what can be expected worst-case
        };
        let (vec, offset) = get_valid_slice_of_len_or_more_aligned(bytes, *i, alignment);
        let slice = &vec[offset..];
        assert_eq!(
            (slice.as_ptr() as usize) % alignment.boundary,
            alignment.offset
        );
        bench_input(&mut group, slice, true, true, bench_fn);
    }
    group.finish();
}

#[inline(never)]
fn basic_from_utf8_no_inline(v: &[u8]) -> bool {
    basic_from_utf8(v).is_ok()
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
        BenchFn::Basic => {
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{:06}", input.len())),
                &input,
                |b, &slice| {
                    b.iter(|| assert_eq!(basic_from_utf8(slice).is_ok(), expected_ok));
                },
            );
        }
        BenchFn::BasicNoInline => {
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{:06}", input.len())),
                &input,
                |b, &slice| {
                    b.iter(|| assert_eq!(basic_from_utf8_no_inline(slice), expected_ok));
                },
            );
        }
        BenchFn::Compat => {
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{:06}", input.len())),
                &input,
                |b, &slice| {
                    b.iter(|| assert_eq!(compat_from_utf8(slice).is_ok(), expected_ok));
                },
            );
        }
        BenchFn::Std => {
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{:06}", input.len())),
                &input,
                |b, &slice| {
                    b.iter(|| assert_eq!(std_from_utf8(slice).is_ok(), expected_ok));
                },
            );
        }
        #[cfg(feature = "simdjson")]
        BenchFn::Simdjson => {
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{:06}", input.len())),
                &input,
                |b, &slice| {
                    b.iter(|| assert_eq!(simdjson_validate(slice), expected_ok));
                },
            );
        }
        #[cfg(feature = "simdutf8_wasm")]
        BenchFn::Wasm(wasm_fn) => {
            let validator = wasm::Validator::new(input);
            match wasm_fn {
                WasmFn::Basic => {
                    group.bench_with_input(
                        BenchmarkId::from_parameter(format!("{:06}", input.len())),
                        &input,
                        |b, &_slice| {
                            b.iter(|| assert_eq!(validator.basic_from_utf8(), expected_ok));
                        },
                    );
                }
                WasmFn::Compat => {
                    group.bench_with_input(
                        BenchmarkId::from_parameter(format!("{:06}", input.len())),
                        &input,
                        |b, &_slice| {
                            b.iter(|| assert_eq!(validator.compat_from_utf8(), expected_ok));
                        },
                    );
                }
                WasmFn::Std => {
                    group.bench_with_input(
                        BenchmarkId::from_parameter(format!("{:06}", input.len())),
                        &input,
                        |b, &_slice| {
                            b.iter(|| assert_eq!(validator.std_from_utf8(), expected_ok));
                        },
                    );
                }
            }
        }
    }
}
