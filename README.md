# simdutf8 – High-speed UTF-8 validation for Rust

## Quick start
Add the dependency to your Cargo.toml file:
```toml
[dependencies]
simdutf8 = { version = "0.0.1"}
```

Use it just like `std::str::from_utf8`:
```rs
use simdutf8::basic::{from_utf8, Utf8Error};

fn main() {
    println!("{}", from_utf8(b"I 	\xEE\x80\xA2 UTF-8!").unwrap());
}
```

Put `simdutf8 = "0.1.0"` in your Cargo.toml file and use `simdutf8::basic::from_utf8` as a drop-in replacement for
`std::str::from_utf8()`. If you need the extended information on validation failures use `simdutf8::compat::from_utf8`
instead.

## Features
* Written in purxTBD Rust
* Up to twenty times faster than the std library on non-ASCII, up to twice as fast on ASCII
* Up to 28 % faster on non-ASCII input compared to the original simdjson implementation
* Supports AVX2 and SIMD implementations on x86 and x86-64, ARMv7 and ARMv8 neon support is planned
* Selects the fastest implementation at runtime based on CPU support
* No dependencies
* No-std support
* `basic` API for the fastest validation, optimized for valid UTF-8
* `compat` API as a plug-in replacement for `std::str::from_utf8()`
* Falls back to the excellent std implementation if SIMD extensions are not supported
* Fuzz-tested

## APIs

### Basic flavor
For maximum speed on valid UTF-8 use the `basic` api flavor. It is fastest on valid UTF-8 but only checks
for errors after processing the whole byte sequence and does not provide detailed information if the data
is not valid UTF-8. `simdutf8::basic::Utf8Error` is a zero-sized error struct.

### Compat flavor
The `compat` flavor is fully API-compatible with `std::str::from_utf8`. In particular `simdutf8::compat::from_utf8()`
returns a `simdutf8::compat::Utf8Error` which has the `valid_up_to()` and `error_len()` methods. The first is useful
for verification of streamed data. Also it fails fast: Errors are checked on-the-fly as the string is processed so
if there is an invalid UTF-8 sequence at the beginning of the data it returns without processing the rest of the data.

## Implementation selection
The fastest implementation is selected at runtime using the `std::is_x86_feature_detected!` macro unless the targeted
CPU supports AVX 2. Since this is the fastest implementation it is called directly. So if you compile with
`RUSTFLAGS=-C target-cpu=native` on a recent machine the AVX 2 implementation is used automatically.

For non-std support (compiled with `--no-default-features`) the implementation is selected based on the supported
target features, use `RUSTFLAGS=-C target-cpu=avx2` to use the AVX 2 implementation or `RUSTFLAGS=-C target-cpu=sse4.2`
for the SSE 4.2 implementation.

If you want to be able to call the individual implementation directly use the `public_imp` feature flag. The validation
implementations are then accessible via `simdutf8::(basic|compat)::imp::x86::(avx2|sse42)::validate_utf8()`.

## When not to use
If you are only processing short byte sequences (less than 64 bytes) the excellent scalar algorithm in standard
library is likely faster. If there is no native implementation for your platform (yet) use the standard library
instead.

## Benchmarks

## Technical details
The implementation is similar to the one in simdjson except that it aligns reads to the block size of the
SIMD extension leading to better peak performance compared to the implementation in simdjson. Since this alignment
means that an incomplete block needs to be processed before the aligned data is read this would lead to worse
performance on short byte sequences. Thus aligned reads are only used with 2048 bytes data or more. Incomplete
reads for the first unaligned and the last incomplete block are done in two aligned 64-byte buffers.

For the compat API we need to check the error buffer on each 64-byte block instead of just aggregating it. If an
error is found the last bytes of the previous block are checked for a cross-block continuation and then
`std::str::from_utf8()` is run to find the exact location of the error.

Care is taken that all functions are properly inlined up to the public interface.

## Thanks
* to Daniel Lemire and the autors of [simdjson] for coming up with the high-performance SIMD implementation.
* to the authors of the [simdjson Rust port]() who did most of the heavy lifting of porting the C++ code to Rust.


## License
This code is made available under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0.html).

It is based on code distributed with [simd-json.rs, the Rust port of simdjson. Simdjson itself is distributed under
the Apache License 2.0.
