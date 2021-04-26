[![CI](https://github.com/rusticstuff/simdutf8/actions/workflows/ci.yml/badge.svg)](https://github.com/rusticstuff/simdutf8/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/simdutf8.svg)](https://crates.io/crates/simdutf8)
[![docs.rs](https://docs.rs/simdutf8/badge.svg)](https://docs.rs/simdutf8)

# simdutf8 – High-speed UTF-8 validation for Rust

Blazingly fast API-compatible UTF-8 validation for Rust using SIMD extensions, based on the implementation from
[simdjson](https://github.com/simdjson/simdjson). Originally ported to Rust by the developers of [simd-json.rs](https://simd-json.rs).

## Disclaimer
This software should not (yet) be used in production, though it has been tested with sample data as well as
fuzzing and there are no known bugs.

## Features
* `basic` API for the fastest validation, optimized for valid UTF-8
* `compat` API as a fully compatible replacement for `std::str::from_utf8()`
* Up to 22 times faster than the std library on non-ASCII, up to three times faster on ASCII
* As fast as or faster than the original simdjson implementation
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
simdutf8 = { version = "0.1.1" }
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
returns a `simdutf8::compat::Utf8Error`, which has `valid_up_to()` and `error_len()` methods. The first is useful for
verification of streamed data. The second is useful e.g. for replacing invalid byte sequences with a replacement character.

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

If you want to be able to call a SIMD implementation directly, use the `public_imp` feature flag. The validation
implementations are then accessible via `simdutf8::(basic|compat)::imp::x86::(avx2|sse42)::validate_utf8()`.

## When not to use
This library uses unsafe code which has not been battle-tested and should not (yet) be used in production.

## Minimum Supported Rust Version (MSRV)
This crate's minimum supported Rust version is 1.38.0.

## Benchmarks
The benchmarks have been done with [criterion](https://bheisler.github.io/criterion.rs/book/index.html), the tables
are created with [critcmp](https://github.com/BurntSushi/critcmp). Source code and data are in the
[bench directory](https://github.com/rusticstuff/simdutf8/tree/main/bench).

The name schema is id-charset/size. _0-empty_ is the empty byte slice, _x-error/66536_ is a 64KiB slice where the very
first character is invalid UTF-8. All benchmarks were run on a laptop with an Intel Core i7-10750H CPU (Comet Lake) on
Windows with Rust 1.51.0 if not otherwise stated. Library versions are simdutf8 v0.1.1 and simdjson v0.9.2. When comparing
with simdjson simdutf8 is compiled with `#inline(never)`.

### simdutf8 basic vs std library UTF-8 validation
![critcmp stimdutf8 v0.1.1 basic vs std lib](https://user-images.githubusercontent.com/3736990/116121179-a8271f80-a6c0-11eb-9b2b-6233c3c824f2.png)
simdutf8 performs better or as well as the std library.

### simdutf8 basic vs simdjson UTF-8 validation on Intel Comet Lake
![critcmp stimdutf8 v0.1.1 basic vs simdjson WSL](https://user-images.githubusercontent.com/3736990/116121748-38656480-a6c1-11eb-8cb4-385c7516a46a.png)
simdutf8 beats simdjson on non-ASCII inputs and small inputs on this CPU. This benchmark is run on 
[WSL](https://docs.microsoft.com/en-us/windows/wsl/install-win10) 
since I could not get simdjson to reach maximum performance on Windows with any C++ toolchain (see simdjson issues [847](https://github.com/simdjson/simdjson/issues/847) and [848](https://github.com/simdjson/simdjson/issues/848)).

### simdutf8 basic vs simdjson UTF-8 validation on AMD Zen 2
![critcmp stimdutf8 v0.1.1 basic vs simdjson AMD Zen 2](https://user-images.githubusercontent.com/3736990/116122729-731bcc80-a6c2-11eb-82a5-6e297778a1c4.png)

On AMD Zen 2 aligning reads apparently does not matter at all. The extra step for aligning even hurts performance a bit around
an input size of 4096.

### simdutf8 basic vs simdutf8 compat UTF-8 validation
![image](https://user-images.githubusercontent.com/3736990/116122427-0dc7db80-a6c2-11eb-8434-f9879742d90d.png)
There is a small performance penalty to continuously checking the error status while processing data, but detecting
errors early provides a huge benefit for the _x-error/66536_ benchmark.

## Technical details
On X86 for inputs shorter than 64 bytes validation is delegated to `core::str::from_utf8()`.

The SIMD implementation is similar to the one in simdjson except that it aligns reads to the block size of the
SIMD extension, which leads to better peak performance compared to the implementation in simdjson on some CPUs.
This alignment means that an incomplete block needs to be processed before the aligned data is read, which
leads to worse performance on byte sequences shorter than 2048 bytes. Thus, aligned reads are only used with
2048 bytes of data or more. Incomplete reads for the first unaligned and the last incomplete block are done in
two aligned 64-byte buffers.

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
