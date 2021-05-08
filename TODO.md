# TODO

# LATER
* armv7 support (with neon runtime selection?)
* streaming API + experimental simdjson support
* faster/smarter error position detection
* try out [multiversion](https://docs.rs/multiversion/0.6.1/multiversion/)
* test all available stable implementations by default as if public_imp were specified
* clean up algorithm src.
  * document prev()
  * newtype -> use (mostly)
  * use imports instead of fully qualified at places
  * trait for SimdU8Value impl.
  * bikeshed: SimdU8Value -> SimdU8Vector | SimdU8xNative | ...

# NEXT
* docs-rs arch building
* copy README to crate doc
* update benchmarks

# OTHER
