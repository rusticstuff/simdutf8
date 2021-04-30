# TODO

# LATER
* benchmark nightly, nightly+likely
* test coverage
* more fuzz testing
* armv7 support with neon runtime selection

# NEXT
* clean up algorithm src. after neon merge
  * check_eof() -> |
  * broadcast() -> splat()
  * indirection functions
  * self first always
  * const?
  * type -> pub use (mostly)
  * use imports instead of fully qualified at places
* test all available implementations by default as if public_imp were specified
* don't inline with compile time selection at all call sites! (aarch64, avx2, no-std all)
* benchmark likely: arm, x64
* fuzz test: ascii opt, aarch64

# OTHER
* why does cargo asm not find from_utf8() on aarch64?
* test aarch64 loop unrolling issue