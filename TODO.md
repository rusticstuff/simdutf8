# TODO
* more fuzz testing
* api doc
* announce

# LATER
* cleanup lookup4 source code, make more inline with upstream impl if possible
* test on fast dedicated cloud server
* why is nightly slower on smaller inputs (both compat and basic flavors) -> benchmark again
* test nightly+likely again - any difference?
* test coverage
* Benchmark and test aligned and unaligned
* Check if std impl. benefits from AVX 2 (xargo)
* fuzz: sse42
* investigate slowness basic: why is ASCII processing comparred to GCC-compiled simdjson-utf8,
  clang-compiled is as slow
* align to 16-byte boundaries on SSE4.2


# NEXT
* publish action?
* inline tests for public_imp