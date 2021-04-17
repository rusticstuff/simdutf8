# TODO
* licensing
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

# NEXT
* fuzz testing: check results, sse42
* bench: move test top-level, usr src/ for common