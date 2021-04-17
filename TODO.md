# TODO
* licensing
* cargo.toml everything
* crates.io
* badges
* doc
* feature to make all available impls public as crate::implementation::*::from_utf8() (for non-std and
  benchmarking)?

# LATER
* cleanup lookup4 source code, make more inline with upstream impl if possible
* test on fast dedicated cloud server
* fuzz testing of all impls
* test nightly again (fast locally???)
* test nightly+likely again - any difference?
* remove codegen-units for bench (should not be needed) -> benchmark to be sure, LTO?

* test fuzzer against non-initialized single buffer bug (commit f0a2904c769c485e9f6524eaa5698082020a26b8)

# NEXT
* move benchmarks to extra package