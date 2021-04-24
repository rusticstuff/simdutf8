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
* investigate simdjson Win clang speed advantage on ASCII

# NEXT
* align to 16-byte boundaries on SSE4.2
* benchmark matrix: pure vs compat, v0.1.0 vs inline vs inline-small-std vs std, AMD vs Xeon
* investigate compat speedup for valid UTF-8 larget 2048 bytes