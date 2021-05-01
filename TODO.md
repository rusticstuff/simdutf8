# TODO

# LATER
* test coverage
* more fuzz testing
* armv7 support with neon runtime selection
* fuzzers: extract common code into crate/module and add honggfuzz
* streaming API
* faster/smarter error position detection

# NEXT
* clean up algorithm src.
  * document prev()
  * type -> use (mostly)
  * use imports instead of fully qualified at places
  * trait for SimdU8Value impl.
* test all available stable implementations by default as if public_imp were specified
* check remaining aarch64 perf. issues

# OTHER
* report aarch64 intrinsic issue