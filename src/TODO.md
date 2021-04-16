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
