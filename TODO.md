# TODO

# LATER
* benchmark nightly, nightly+likely
* test coverage
* more fuzz testing
* armv7 support with neon runtime selection
* fuzzers: extract common into crate and add honggfuzz

# NEXT
* clean up algorithm src. after neon merge
  * document prev()
  * type -> use (mostly)
  * use imports instead of fully qualified at places
* test all available stable implementations by default as if public_imp were specified

# OTHER
* test aarch64 loop unrolling issue