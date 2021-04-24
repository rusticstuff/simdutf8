[![CI](https://github.com/rusticstuff/simdutf8/actions/workflows/ci.yml/badge.svg)](https://github.com/rusticstuff/simdutf8/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/simdutf8.svg)](https://crates.io/crates/simdutf8)
[![docs.rs](https://docs.rs/simdutf8/badge.svg)](https://docs.rs/simdutf8)

# simdutf8 – High-speed UTF-8 validation for Rust

Blazingly fast API-compatible UTF-8 validation for Rust using SIMD extensions, based on the implementation from
[simdjson](https://github.com/simdjson/simdjson). Originally ported to Rust by the developers of [simd-json.rs](https://simd-json.rs).

## Disclaimer
This software should be considered alpha quality and should not (yet) be used in production, though it has been tested
with sample data as well as a fuzzer and there are no known bugs. It will be tested more rigorously before the first
production release.

## Features
* `basic` API for the fastest validation, optimized for valid UTF-8
* `compat` API as a fully compatible replacement for `std::str::from_utf8()`
* Up to twenty times faster than the std library on non-ASCII, up to twice as fast on ASCII
* Up to 28% faster on non-ASCII input compared to the original simdjson implementation
* Supports AVX 2 and SSE 4.2 implementations on x86 and x86-64. ARMv7 and ARMv8 neon support is planned
* Selects the fastest implementation at runtime based on CPU support
* Written in pure Rust
* No dependencies
* No-std support
* Falls back to the excellent std implementation if SIMD extensions are not supported

## Quick start
Add the dependency to your Cargo.toml file:
```toml
[dependencies]
simdutf8 = { version = "0.1.0" }
```

Use `simdutf8::basic::from_utf8` as a drop-in replacement for `std::str::from_utf8()`.

```rust
use simdutf8::basic::from_utf8;

println!("{}", from_utf8(b"I \xE2\x9D\xA4\xEF\xB8\x8F UTF-8!").unwrap());
```

If you need detailed information on validation failures, use `simdutf8::compat::from_utf8`
instead.

```rust
use simdutf8::compat::from_utf8;

let err = from_utf8(b"I \xE2\x9D\xA4\xEF\xB8 UTF-8!").unwrap_err();
assert_eq!(err.valid_up_to(), 5);
assert_eq!(err.error_len(), Some(2));
```

## APIs

### Basic flavor
Use the `basic` API flavor for maximum speed. It is fastest on valid UTF-8, but only checks
for errors after processing the whole byte sequence and does not provide detailed information if the data
is not valid UTF-8. `simdutf8::basic::Utf8Error` is a zero-sized error struct.

### Compat flavor
The `compat` flavor is fully API-compatible with `std::str::from_utf8`. In particular, `simdutf8::compat::from_utf8()`
returns a `simdutf8::compat::Utf8Error`, which has `valid_up_to()` and `error_len()` methods. The first is useful for verification of streamed data. The second is useful e.g. for replacing invalid byte sequences with a replacement character.

It also fails early: errors are checked on-the-fly as the string is processed and once
an invalid UTF-8 sequence is encountered, it returns without processing the rest of the data.
This comes at a performance penality compared to the `basic` API even if the input is valid UTF-8.

## Implementation selection
The fastest implementation is selected at runtime using the `std::is_x86_feature_detected!` macro unless the CPU
targeted by the compiler supports the fastest available implementation.
So if you compile with `RUSTFLAGS="-C target-cpu=native"` on a recent x86-64 machine, the AVX 2 implementation is selected at
compile time and runtime selection is disabled.

For no-std support (compiled with `--no-default-features`) the implementation is always selected at compile time based on
the targeted CPU. Use `RUSTFLAGS="-C target-feature=+avx2"` for the AVX 2 implementation or `RUSTFLAGS="-C target-feature=+sse4.2"`
for the SSE 4.2 implementation.

If you want to be able to call A SIMD implementation directly, use the `public_imp` feature flag. The validation
implementations are then accessible via `simdutf8::(basic|compat)::imp::x86::(avx2|sse42)::validate_utf8()`.

## When not to use
If you are only processing short byte sequences (less than 64 bytes), the excellent scalar algorithm in the standard
library is likely faster. Also, this library uses unsafe code which has not been battle-tested and should not (yet)
be used in production.

## Benchmarks

The benchmarks have been done with [criterion](https://bheisler.github.io/criterion.rs/book/index.html), the tables
are created with [critcmp](https://github.com/BurntSushi/critcmp). Source code and data are in the
[bench directory](https://github.com/rusticstuff/simdutf8/tree/main/bench).

The name schema is id-charset/size. _0-empty_ is the empty byte slice, _x-error/66536_ is a 64KiB slice where the very
first character is invalid UTF-8. All benchmarks were run on a laptop with an Intel Core i7-10750H CPU (Comet Lake) on
Windows with Rust 1.51.0.

### simdutf8 basic vs std library UTF-8 validation
![critcmp stimdutf8 basic vs std lib](https://raw.githubusercontent.com/rusticstuff/simdutf8/main/img/basic-vs-std.png)
simdutf8 performs better except for inputs ≤ 64 bytes.

### simdutf8 basic vs simdjson UTF-8 validation
![critcmp st lib vs stimdutf8 basic](https://raw.githubusercontent.com/rusticstuff/simdutf8/main/img/basic-vs-simdjson.png)
simdutf8 is faster than simdjson except for some crazy optimization by clang for the pure ASCII
loop (to be investigated). simdjson is compiled using clang and gcc from MSYS.

### simdutf8 basic vs simdutf8 compat UTF-8 validation
![critcmp st lib vs stimdutf8 basic](https://raw.githubusercontent.com/rusticstuff/simdutf8/main/img/basic-vs-compat.png)
There is a small performance penalty to continuously checking the error status while processing data, but detecting
errors early provides a huge benefit for the _x-error/66536_ benchmark.

## Technical details
The implementation is similar to the one in simdjson except that it aligns reads to the block size of the
SIMD extension, which leads to better peak performance compared to the implementation in simdjson. This alignment
means that an incomplete block needs to be processed before the aligned data is read, which would lead to worse
performance on short byte sequences. Thus, aligned reads are only used with 2048 bytes of data or more. Incomplete
reads for the first unaligned and the last incomplete block are done in two aligned 64-byte buffers.

For the compat API we need to check the error buffer on each 64-byte block instead of just aggregating it. If an
error is found, the last bytes of the previous block are checked for a cross-block continuation and then
`std::str::from_utf8()` is run to find the exact location of the error.

Care is taken that all functions are properly inlined up to the public interface.

## Thanks
* to the authors of simdjson for coming up with the high-performance SIMD implementation.
* to the authors of the simdjson Rust port who did most of the heavy lifting of porting the C++ code to Rust.


## License
This code is made available under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0.html).

It is based on code distributed with simd-json.rs, the Rust port of simdjson, which is dual-licensed under
the MIT license and Apache 2.0 license.

simdjson itself is distributed under the Apache License 2.0.

## References

John Keiser, Daniel Lemire, [Validating UTF-8 In Less Than One Instruction Per Byte](https://arxiv.org/abs/2010.03090), Software: Practice and Experience 51 (5), 2021
