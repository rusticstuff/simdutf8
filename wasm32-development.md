# Developing/Testing the `wasm32` Target

Since there is no native host platform for WebAssembly, developing/targeting requires a bit more setup than a vanilla
Rust toolchain environment.  To build/target this library outside a `wasm-pack` context, you can do the following:

* Install toolchain with `wasm32-wasip1` or `wasm32-unknown-unknown` (e.g. `rustup target add wasm32-wasip1`).
  * `wasm32-wasip1` is a nice target because it gives us the capability to run the tests as-is with a WASM runtime.
* Install a WASM runtime (e.g. [Wasmer]/[Wasmtime]/[WAVM]).
* Install `wasm-runner` a simple runner wrapper to run WASM targeted code with a WASM runtime:

```
$ cargo install wasm-runner
```

* Add a Cargo configuration file to target `wasm` and allow the unit tests to be run with a WASM VM *by default*:

```
[build]
target = "wasm32-wasip1"
rustflags = "-C target-feature=+simd128"

[target.'cfg(target_arch="wasm32")']
runner = ["wasm-runner", "wasmer"]
```

* Run the build/tests:

```
$ cargo test
$ cargo test --all-features
```

You can do this without configuration as well:

```
$ RUSTFLAGS="-C target-feature=+simd128" \
    CARGO_TARGET_WASM32_WASI_RUNNER="wasm-runner wasmer" \
    cargo test --target wasm32-wasip1
$ RUSTFLAGS="-C target-feature=+simd128" \
    CARGO_TARGET_WASM32_WASI_RUNNER="wasm-runner wasmer" \
    cargo test --target wasm32-wasip1 --all-features
```

[wasmer]: https://wasmer.io/
[wasmtime]: https://wasmtime.dev/
[wavm]: https://wavm.github.io/
