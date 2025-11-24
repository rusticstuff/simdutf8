fn main() {
    println!("cargo::rustc-check-cfg=cfg(avx512_stable)");
    // `if rustversion::cfg!(...)` is not supported in older Rust versions
    if avx512_stable() {
        println!("cargo:rustc-cfg=avx512_stable");
    }
}

#[rustversion::since(1.89)]
fn avx512_stable() -> bool {
    true
}

#[rustversion::before(1.89)]
fn avx512_stable() -> bool {
    false
}
