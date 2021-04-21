# TODO
* announce

# LATER
* cleanup lookup4 source code, make more inline with upstream impl if possible
* test on fast dedicated cloud server
* why is nightly slower on smaller inputs (both compat and basic flavors) -> benchmark again
* test nightly+likely again - any difference?
* test coverage
* Check if std impl. benefits from AVX 2 (xargo)
* more fuzz testing
* fuzz: sse42
* align to 16-byte boundaries on SSE4.2
* investigate simdjson Win clang speed advantage on ASCII

# NEXT
* publish action?
* inline tests for public_imp