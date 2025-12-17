[![CI](https://github.com/rusticstuff/simdutf8/actions/workflows/portable.yml/badge.svg)](https://github.com/rusticstuff/simdutf8/actions/workflows/portable.yml)
[![crates.io](https://img.shields.io/crates/v/simdutf8-portable.svg)](https://crates.io/crates/simdutf8-portable)
[![docs.rs](https://docs.rs/simdutf8-portable/badge.svg)](https://docs.rs/simdutf8-portable)

# simdutf8-portable – Fast UTF-8 validation using `core::simd` (portable SIMD)

Fast API-compatible UTF-8 validation for Rust using the experimental
[`core::simd`](https://doc.rust-lang.org/core/simd/index.html) (portable SIMD) module from the
standard library. An up-to-date nightly Rust compiler is required. The API and the algorithm are the
same as in the [simdutf8](https://crates.io/crates/simdutf8) crate.

## Features

- no unsafe code (`#[forbid(unsafe_code)]`) in the implementation
- good performance (with **caveats**, see [architecture notes](#Architecture-notes))
- `auto` module which selects the best implementation for known-good targets at compile-time
  including falling back to `core::str:from_utf8`.
- new platforms need no new code as long as they are supported by `core::simd`
- `no_std` support
- supports 128-bit and 256-bit SIMD

## Limitations

The functions in `core::simd` are marked as `#[inline]`. This way they are compiled to MIR  Code generation is deferred until they are actually used in a crate.
- target-feature required
- no runtime implementation selection
- slower
  - memcpy calls
- swizzle_dyn
  - slow on non-special-cased targets
  - requires -Zbuild-std for sse4.2, avx2 support if not part of the target architecture

## Performance

- There are no unnecessary bounds checks in the compiled code, functions are properly inlined and
  loops properly unrolled (as of nightly-xx).
- uses memcpy because of forbid(unsafe), see https://github.com/llvm/llvm-project/issues/87440
- swizzle_dyn slower because of abstraction

## Architecture notes

| Architecture    | [Targets](https://doc.rust-lang.org/nightly/rustc/platform-support.html) | Performance\*   | Notes                                                                                                                                                                                                                                         |
| --------------- | ------------------------------------------------------------------------ | --------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| aarch64         | all                                                                      | xx% of simdutf8 | works out of the box                                                                                                                                                                                                                          |
| x86_64 (avx2)   | all                                                                      | xx% of simdutf8 | requires `-Zbuild-std` and `RUSTFLAGS="-C target-feature=+avx2"`                                                                                                                                                                              |
| x86_64 (sse4.2) | all                                                                      | xx% of simdutf8 | requires `-Zbuild-std` and `RUSTFLAGS="-C target-feature=+sse4.2"`                                                                                                                                                                            |
| wasm32          | all                                                                      | (not tested)    | requires `-Zbuild-std` and `RUSTFLAGS="-C target-feature=+simd128"`                                                                                                                                                                           |
| armv7           | thumbv7neon&#x2011;\*                                                    | (not tested)    | works out of the box                                                                                                                                                                                                                          |
| armv7           | others                                                                   | (not tested)    | requires `-Zbuild-std` and `RUSTFLAGS="-C target-feature=+neon"`                                                                                                                                                                              |
| other           | ...                                                                      | bad             | falls back to `core::str::from_utf8` unless `forcesimd128` or `forcesimd256` are used. Check [`swizzle_dyn` support](https://github.com/rust-lang/rust/blob/master/library/portable-simd/crates/core_simd/src/swizzle_dyn.rs) before forcing. |

\*) using `basic::from_utf8`

## Quick start

Add the dependency to your Cargo.toml file:

```toml
[dependencies]
simdutf8-portable = "0.0.1"
```

Use `simdutf8-portable::basic::from_utf8()` as a drop-in replacement for `std::str::from_utf8()`.

```rust
use simdutf8-portable::basic::from_utf8;

println!("{}", from_utf8(b"I \xE2\x9D\xA4\xEF\xB8\x8F UTF-8!").unwrap());
```

If you need detailed information on validation failures, use `simdutf8::compat::from_utf8()`
instead.

```rust
use simdutf8-portable::compat::from_utf8;

let err = from_utf8(b"I \xE2\x9D\xA4\xEF\xB8 UTF-8!").unwrap_err();
assert_eq!(err.valid_up_to(), 5);
assert_eq!(err.error_len(), Some(2));
```

## APIs

See docs or [simdutf8](https://crates.io/crates/simdutf8).

## Minimum Supported Rust Version (MSRV)

Rust nightly as of xx xx

## Thanks

- to [Heinz N. Gies](https://github.com/licenser) for the initial portable SIMD implementation,
- to the authors of `core::simd` module,
- to the authors of simdjson for coming up with the high-performance SIMD implementation,
- to the authors of the simdjson Rust port who did most of the initial heavy lifting of porting the
  C++ code to Rust.

## License

This code is dual-licensed under the
[Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0.html) and the
[MIT License](https://opensource.org/licenses/MIT).

It is based on code distributed with simd-json.rs, the Rust port of simdjson, which is dual-licensed
under the MIT license and Apache 2.0 license as well.

simdjson itself is distributed under the Apache License 2.0.

## References

John Keiser, Daniel Lemire,
[Validating UTF-8 In Less Than One Instruction Per Byte](https://arxiv.org/abs/2010.03090),
Software: Practice and Experience 51 (5), 2021
