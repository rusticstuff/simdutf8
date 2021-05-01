# TODO

# LATER
* benchmark nightly, nightly+likely
* test coverage
* more fuzz testing
* armv7 support with neon runtime selection
* fuzzers: extract common into crate and add honggfuzz

# NEXT
* clean up algorithm src. after neon merge
  * check_eof() -> check_incomplete_pending()
  * document prev()
  * type -> pub use (mostly)
  * use imports instead of fully qualified at places
* test all available stable implementations by default as if public_imp were specified
* benchmark likely: arm, x64
* fuzz test: ascii opt, aarch64

# OTHER
* why does cargo asm not find from_utf8() on aarch64?
* test aarch64 loop unrolling issue