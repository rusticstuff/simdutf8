# TODO
* licensing: simdjson, simd-json.rs, simdjson-utf8-bench
* finish README.md
* * thanks, licensing, features, pure vs compat, improvements, limitations etc.
* api doc
* badges
* public repo
* crates.io

# LATER
* cleanup lookup4 source code, make more inline with upstream impl if possible
* test on fast dedicated cloud server
* why is nightly slower on smaller inputs (both compat and pure flavors)
* test nightly+likely again - any difference?
* feature to make all available impls public as crate::implementation::*::from_utf8() (for non-std and
  benchmarking)?
* test coverage
* Benchmark against simdjson
* Benchmark and test aligned and unaligned
* Check if std can be autovectorized
* Test if aligning on SIMD width is sufficient
* Table, which impl is used under which circumstances
* libfuzz testing: old testcase
* fuzz: sse42
* investigate slowness pure: why is ASCII processing comparred to GCC-compiled simdjson-utf8,
  clang-compiled is as slow
* expose implementations

# NEXT
* remove indirections via pub(crate) use?
* run test against all impls
* always benchmark with unaligned/aligned input
