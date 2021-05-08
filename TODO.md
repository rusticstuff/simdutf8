# TODO

# LATER
* test coverage
* more fuzz testing
* armv7 support (with neon runtime selection?)
* fuzzers: extract common code into crate/module and add honggfuzz
* streaming API + experimental simdjson support
* faster/smarter error position detection
* try out [multiversion](https://docs.rs/multiversion/0.6.1/multiversion/)

# NEXT
* clean up algorithm src.
  * document prev()
  * newtype -> use (mostly)
  * use imports instead of fully qualified at places
  * trait for SimdU8Value impl.
  * bikeshed: SimdU8Value -> SimdU8Vector | SimdU8xNative | ...

* test all available stable implementations by default as if public_imp were specified
* document aarch64; docs-rs arch building
* discourage -Oz
* std handling: no-std + extern crate std if std
* Doc: remove for Rust from README header but not from description
* proptests?

# OTHER
