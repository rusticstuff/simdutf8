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
* missed inlining plays a huge role of course
* alignment plays a huge role
* * up to 20% better performance on long inputs
* * the added code causes a litte slowdown in compat on shorter inputs
    and unfortunately a large slowdown on pure ASCII for all input sizes (to be investigated)

# Tuning criterion

# Laptops
* Generally more noisy due to power and temperature constraints
* Beware of BD PROCHOT on aged machines, can cause severe throttling