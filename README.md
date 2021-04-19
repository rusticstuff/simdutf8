# Blurb
* no-alloc
* no-std
* no-deps
* Two APIS
* How fast is it?
* * [Ridiculously fast](https://lemire.me/blog/2020/10/20/ridiculously-fast-unicode-utf-8-validation/)
* * But seriously, how fast is it?
* * * Benchmarks

* * 1) 'compat' API fully compatible with `Utf8Error' `core::str::from_utf8()` and `std::str::from_utf8()`
      Meant as a general purpose replacement. Works well with streams as the `Utf8Error::valid_up_to()'
      functionality is needed for proper processing.
* * 2) optimized for valid UTF-8 strings: no detailed error information, whole string is scanned even if
* *    errors are found early.
* std API uses autodetection of CPU features to select the best implementation.
* All functions are fully inlined.
* Use hints features on nightly (test if any faster) to make use of likely/unlikely intrinsics
* fallback uses the standard core/std implementation, which is quite fast for a scalar implementation, in particular on ASCII
* fuzz-tested
* 10 GiB/sec. performance on non-ASCII strings, xx times faster than stdlib
* 50+ Gib/sec. performance on ASCII strings, xx times faster than stdlib
* SIMD implementations for x86/x86-64 AVX 2 and SSE 4.2, ports of the neon SIMD implementations for aarch64
  and armv7 are planned.
* document `RUSTFLAGS="-C target-feature=+avx2"` and `RUSTFLAGS="-C target-cpu=native"` std code selection

# Limitations

# License

# Thanks