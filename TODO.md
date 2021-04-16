# TODO
* licensing
* doc
* feature to make all available impls public as crate::implementation::*::from_utf8() (for non-std and benchmarking)?

# LATER
* cleanup lookup4 source code, make more inline with upstream impl if possible
* test on fast dedicated cloud server
* fuzz testing of all impls

# NEXT
* save new baselines
* test nightly again (fast locally???)
* test nightly+likely again - any difference?
* remove codegen-units for bench (should not be needed) -> benchmark to be sure
* cargo asm test to make sure functions are properly inlined