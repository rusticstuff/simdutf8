use std::time::Duration;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench_group(min_time = Duration::from_secs(2))]
mod bench {
    use divan::counter::BytesCount;
    use divanbench::{Alignment, get_valid_slice_of_len_or_more_aligned, scale_to_one_mib};

    static LATIN: &[u8] = include_bytes!("../../bench/data/Latin-Lipsum.txt");
    static CYRILLIC: &[u8] = include_bytes!("../../bench/data/Russian-Lipsum.txt");
    static CHINESE: &[u8] = include_bytes!("../../bench/data/Chinese-Lipsum.txt");
    static EMOJI: &[u8] = include_bytes!("../../bench/data/Emoji-Lipsum.txt");

    #[divan::bench(name = "0-empty")]
    fn empty(bencher: divan::Bencher) {
        let bytes = &[];

        // warm up
        let start_time = std::time::Instant::now();
        while start_time.elapsed().as_secs() < 1 {
            for _ in 0..100000 {
                assert!(simdutf8::basic::from_utf8(bytes).is_ok());
            }
        }

        bencher
            .bench_local(|| simdutf8::basic::from_utf8(bytes));
    }

    #[divan::bench(name = "1-latin", args = [1, 8, 64, 512, 4096, 65536, 131072])]
    fn latin(bencher: divan::Bencher, n: usize) {
        bench_bytes(bencher, n, LATIN);
    }

    #[divan::bench(name = "2-cyrillic", args = [1, 8, 64, 512, 4096, 65536, 131072])]
    fn cyrillic(bencher: divan::Bencher, n: usize) {
        bench_bytes(bencher, n, CYRILLIC);
    }

    #[divan::bench(name = "3-chinese", args = [1, 8, 64, 512, 4096, 65536, 131072])]
    fn chinese(bencher: divan::Bencher, n: usize) {
        bench_bytes(bencher, n, CHINESE);
    }

    #[divan::bench(name = "4-emoji", args = [1, 8, 64, 512, 4096, 65536, 131072])]
    fn emoji(bencher: divan::Bencher, n: usize) {
        bench_bytes(bencher, n, EMOJI);
    }

    fn bench_bytes(bencher: divan::Bencher, n: usize, bytes: &[u8]) {
        let bytes = &scale_to_one_mib(bytes);

        let alignment = Alignment {
            boundary: 64,
            offset: 8, // 8 is the default alignment on 64-bit, so this is what can be expected worst-case
        };

        let (vec, offset) = get_valid_slice_of_len_or_more_aligned(bytes, n, alignment);
        let bytes = &vec[offset..];

        // warm up
        let start_time = std::time::Instant::now();
        while start_time.elapsed().as_secs() < 1 {
            for _ in 0..100000 {
                assert!(simdutf8::basic::from_utf8(bytes).is_ok());
            }
        }

        bencher
            .counter(BytesCount::of_slice(bytes))
            .bench_local(|| simdutf8::basic::from_utf8(bytes));
    }
}
