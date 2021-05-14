# Changelog
## [Unreleased]

## [0.1.2] - 2021-05-09
### New features
* Aarch64 support (e.g. Apple Silicon, Raspberry Pi 4, ...) with nightly Rust and crate feature `aarch64_neon`

### Performance
* Another speedup on pure ASCII data
* Aligned reads have been removed as the performance was worse overall.
* Prefetch is used selectively on AVX 2, where it provides a slight benefit on some Intel CPUs.

[Comparison vs v0.1.1 on x86-64](https://user-images.githubusercontent.com/3736990/117568946-7a2fdb00-b0c3-11eb-936e-358850f0a9ad.png)

### Other
* Refactored SIMD integration to allow easy implementation for new architectures
* Full test coverage
* Thoroughly fuzz-tested

## [0.1.1] - 2021-04-26
### Performance
* Large speedup on small inputs from delegation to std lib
* Up to 50% better peak throughput on ASCII
* `#[inline]` main entry points for a small general speedup.

[Benchmark against v0.1.0](https://user-images.githubusercontent.com/3736990/116128298-12dc5900-a6c9-11eb-8c23-a117b3e57edb.png)

### Other
* Make both Utf8Error variants implement `std::error::Error`
* Make `basic::Utf8Error` implement `core::fmt::Display`
* Document Minimum Supported Rust Version (1.38.0).
* Reduce package size.
* Documentation updates

## [0.1.0] - 2021-04-21
- Documentation updates only.

0.1.x releases will have API compatibility.

## [0.0.3] - 2021-04-21
- Documentation update only.

## [0.0.2] - 2021-04-20
- Documentation update only.

## [0.0.1] - 2021-04-20
- Initial release.

[Unreleased]: https://github.com/rusticstuff/simdutf8/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/rusticstuff/simdutf8/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/rusticstuff/simdutf8/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/rusticstuff/simdutf8/compare/v0.0.3...v0.1.0
[0.0.3]: https://github.com/rusticstuff/simdutf8/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/rusticstuff/simdutf8/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/rusticstuff/simdutf8/releases/tag/v0.0.1
