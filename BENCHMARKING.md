# Measures for reproducible results
* idle machine
* performance governor
* disable turbo mode
* [cset shield](https://documentation.suse.com/sle-rt/12-SP4/html/SLE-RT-all/cha-shielding-model.html)
* pin benchmark to one of the shielded cores
* LTO makes a difference for some reason
* codegen-units = 1 (most likely not needed)
* disable hyper-threading (not sure if needed, my test machine has no hyper threading)

# Factors affecting performance
* missed inlining plays a huge role of course, unfortunately one can not use `#[inline(always)] on
* functions with `#[target_feature(enable = "...")]` and even that would only be a hint. What is needed
* is an error on non-inlining. Simulating that using [cargo asm](https://github.com/gnzlbg/cargo-asm) to
* make sure that methods supposed to be inlined do not exist in the rlib.
* alignment plays a huge role
* * up to 20% better performance on long but unaligned slices (which are apparently likely at least on Linux)
* * the added code causes a litte slowdown in compat on shorter inputs
    and unfortunately a large slowdown on pure ASCII for all input sizes (to be investigated)

# Lessons learned
* Stack-allocated 64-byte arrays are 64-byte-aligned automatically on x86-64 (same code as struct
  containing the array with
* 0-initialized buffers are faster (less instructions)

# Tuning criterion

# Laptops
* Generally more noisy due to power and temperature constraints
* Beware of BD PROCHOT on aged machines, can cause severe throttling


# ideas
* in test: 0-initialized temp buf instead of 0x20-initialized (less instructions)
* in test: single 0-initialized temp buf (less instructions)
* in test: with aligned buffer (same inst count, effect confirmed in assembly)
* align simdinput, utf8 state
* use one temporary buffer
* benchmark against aligned and unaligned
* test limit when to start alignment