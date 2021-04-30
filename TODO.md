# TODO

# LATER
* benchmark nightly, nightly+likely
* test coverage
* more fuzz testing
* armv7 support with neon runtime selection

# NEXT
* integrate neon support
  * behind feature flag
  * make ci build
  * fuzz test a bit
* test improved ASCII perf. idea
* add _quick benchmarks, maybe default?
* test improved memcpy vs. no memcpy
* clean up algorithm src. after neon merge
  * check_eof() -> |
  * broadcast() -> splat()
  * indirection functions
  * self first always
  * const?

# OTHER
* why does cargo asm not find from_utf8() on aarch64?
* test aarch64 loop unrolling issue