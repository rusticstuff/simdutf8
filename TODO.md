# TODO
* licensing
* doc
* feature to make all available impls public as crate::implementation::*::from_utf8() (for non-std and benchmarking)?
* badges
* crates.io
- BENCHMARKING.md

# LATER
* cleanup lookup4 source code, make more inline with upstream impl if possible
* test on fast dedicated cloud server
* fuzz testing of all impls

# NEXT
* save new baselines
* test nightly again (fast locally???)
* test nightly+likely again - any difference?
* remove codegen-units for bench (should not be needed) -> benchmark to be sure