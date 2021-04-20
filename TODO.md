# TODO
* licensing: simdjson, simd-json.rs, simdjson-utf8-bench
* finish README.md
  * thanks, licensing, features, pure vs compat, improvements, limitations, which impl is used under whic
    circumstances, benchmark against simdjson, etc.
* api doc
* badges
* github repo -> public
* crates.io
* announce

# LATER
* cleanup lookup4 source code, make more inline with upstream impl if possible
* test on fast dedicated cloud server
* why is nightly slower on smaller inputs (both compat and pure flavors) -> benchmark again
* test nightly+likely again - any difference?
* test coverage
* Benchmark and test aligned and unaligned
* Check if std impl. benefits from AVX 2 (xargo)
* fuzz: sse42
* investigate slowness pure: why is ASCII processing comparred to GCC-compiled simdjson-utf8,
  clang-compiled is as slow
* align 16-byte on SSE4.2


# NEXT
* rename pure?
* cargo doc in CI
* rename public_imp
* inline tests for public_imp