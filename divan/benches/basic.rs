use std::time::Duration;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench_group(min_time = Duration::from_secs(2))]
mod bench {
    use divan::counter::BytesCount;
    use divanbench::{Alignment, get_valid_slice_of_len_or_more_aligned, scale_to_one_mib};

    #[divan::bench(name = "1-latin", args = [1, 8, 64, 512, 4096, 65536, 131072])]
    fn latin(bencher: divan::Bencher, n: usize) {
        let alignment = Alignment {
            boundary: 64,
            offset: 8, // 8 is the default alignment on 64-bit, so this is what can be expected worst-case
        };
        let bytes = &scale_to_one_mib(include_bytes!("../../bench/data/Latin-Lipsum.txt"));

        let (vec, offset) = get_valid_slice_of_len_or_more_aligned(bytes, n, alignment);
        let slice = &vec[offset..];
        bencher
            .counter(BytesCount::of_slice(slice))
            .bench_local(|| simdutf8::basic::from_utf8(slice));
    }
}
