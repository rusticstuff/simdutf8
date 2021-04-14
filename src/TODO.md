# Issues
* nostd
* neon support
* avx512 support
* make buildable with fallback impl

# TODO
* add from_utf8_mut()
* is_valid_until() implementation
* fuzz testing
* licensing
* doc

# LATER
* test on fast dedicated cloud server

# NEXT
* investigate fn ptr call overhead
* make benchmarks work again
* run test against: sse42, avx2, fallback
* ci build: nostd, nostd+target=+avx2, nostd+target=+sse4.2, cross-compile to arm for fallback
* clippy all features, all targets
* critcmp issue