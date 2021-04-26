# Benchmarking

## How-to

By default `cargo bench` runs the benchmarks for basic, compat and the std library.

To benchmark just an individual implementation use:
* basic API
  `cargo bench --bench=throughput_basic`
* compat API
  `cargo bench --bench=throughput_compat`
* std library
  `cargo bench --bench=throughput_std`
* simdjson library (C++)
  `cargo bench --features=simdjson --bench=throughput_simdjson`

Adding `-- --save-baseline some_name` to the bench commandline and then using [critcmp](https://github.com/BurntSushi/critcmp) to compare benchmarks is handy as well.


## Various hints and observations

### Measures for reproducible results
* idle machine
* cpufreq performance governor
* pin benchmark to one of the shielded cores (done in code)
* Optional: disable turbo mode
* Optional: [cset shield](https://documentation.suse.com/sle-rt/12-SP4/html/SLE-RT-all/cha-shielding-model.html)
* Optional: disable hyper-threading (not sure if necessary, my test machine has no hyper threading)

### Factors affecting performance
* missed inlining plays a huge role of course, unfortunately one can not use `#[inline(always)] on
* functions with `#[target_feature(enable = "...")]` and even though that would only be strong a hint.
  What is needed is an error on non-inlining. Simulating that using [cargo asm](https://github.com/gnzlbg/cargo-asm) to make sure that methods supposed to be inlined do not exist in the rlib.
* alignment plays a huge role on some machines (modern Intel, modern AMD not so much)
  * up to 20% better performance on long but unaligned slices (which are apparently likely at least on Linux)

### Lessons learned
* 0-initialized temp buf instead of 0x20-initialized (less instructions) -> ✔️ improved perf.
* single 0-initialized temp buf -> ❌ not faster
* single aligned buffer -> ❌ not faster
* two buffers, aligned -> ✔️ improved perf.
* Aligment on `Utf8CheckingState<T>` and `SimdInput` -> ❌ not faster, but seems like a good idea nevertheless

### Laptops
* Generally more noisy due to power and temperature constraints
* Beware of BD PROCHOT on aged machines, can cause severe throttling

### Test machines
* Intel(R) Core(TM) i7-10750H CPU @ 2.60GHz (Comet Lake)
* AMD Ryzen 7 PRO 3700 8-Core Processor @ 3.60 GHz (Zen 2)