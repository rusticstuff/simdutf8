# TODO
* licensing
* doc
* test likely
* benchmark > 64KiB
* feature to make all available impls public as crate::implementation::*::from_utf8() (for non-std and benchmarking)?
* move err to extra module
* cleanup lookup4 source code, make more inline with upstream impl if possible
* naming: fast vs. exact
* also: rename _exact -> no suffix, no suffix -> assume_ok (?)

# LATER
* test on fast dedicated cloud server
* fuzz testing of all impls
* sse42 benchmark
* neon support
* avx512 support

# NEXT
* save new baselines
* exact errors
** ? `#[!cold]` or #inline[] or  nothing ?
* test nightly again (fast locally???)
* remove codegen-units for bench (should not be needed) -> benchmark to be sure

# Blurb
* no-alloc
* no-std
* Two APIS
** 1) 'compat' API fully compatible with `Utf8Error' `core::str::from_utf8()` and `std::str::from_utf8()`
      Meant as a general purpose replacement. Works well with streams as the `Utf8Error::valid_up_to()'
      functionality is needed for proper processing.
** 2) optimized for valid UTF-8 strings: no detailed error information, whole string is scanned even if
**    errors are found early.
* std API uses autodetection of CPU features to select the best implementation.
* Compilation with `RUSTFLAGS="-C target-feature=+avx2"` or  `RUSTFLAGS="-C target-cpu=native"` on a
* x86/x86-64 CPUAwith AVX 2 support uses the AVX 2 implementation directly for maximum performance.
* Autodetection costs up to xx% on small strings due to the function ptr call.
* All functions are fully inlined.
* Use hints features on nightly (test if any faster) to make use of likely/unlikely intrinsics
* fallback uses the standard core/std implementation, which is quite fast for a scalar implementation, in particular on ASCII
* fuzz-tested
* 10 GiB/sec. performance on non-ASCII strings, xx times faster than stdlib
* 50+ Gib/sec. performance on ASCII strings, xx times faster than stdlib
* SIMD implementations for x86/x86-64 AVX 2 and SSE 4.2, ports of the neon SIMD implementations for aarch64 and armv7 are planned.