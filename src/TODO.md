# Issues
* nostd
* neon support
* avx512 support
* make buildable with fallback impl

# TODO
* restructure implementations -> implementation get_available_implementations(), set_implementation(), default: autodetect, check e.g. memchr impl
* add from_utf8_mut()
* is_valid_until() implementation
* fuzz testing
* implementation traits: overhead?
* macro_export needed?
* test cpu feature detection overhead
* licensing
* doc

# LATER
* test on fast dedicated cloud server

# NEXT
* make benchmarks work again
* build test: nostd, nostd+target=+avx2, nostd+target=+sse4.2, cross-compile to arm for fallback
* clippy all features, all targets