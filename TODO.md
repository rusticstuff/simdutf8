# TODO
* licensing
* finish README.md
* * thanks, licensing, features, pure vs compat, improvements, limitations etc.
* api doc
* badges
* fuzz testing: avx2, sse42
* test fuzzer against non-initialized single buffer bug (commit f0a2904c769c485e9f6524eaa5698082020a26b8)
* public repo
* crates.io
* bench: move test top-level dir

# LATER
* cleanup lookup4 source code, make more inline with upstream impl if possible
* test on fast dedicated cloud server
* test nightly again (fast locally???)
* test nightly+likely again - any difference?
* feature to make all available impls public as crate::implementation::*::from_utf8() (for non-std and
  benchmarking)?

# NEXT
* new std benchmark
* afl?