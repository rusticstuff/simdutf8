use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    if env::var("CARGO_FEATURE_SIMDUTF8_WASM").is_ok() {
        // for WASM benchmarking we need to cross-compile the shim crate to a WASM
        // module we can link in on the host platform
        let shim_dir = Path::new("wasm-shim")
            .canonicalize()
            .expect("Could not find WASM shim");

        println!("cargo:rerun-if-changed={}", shim_dir.display());
        let mut cmd = Command::new("cargo");
        cmd.current_dir(shim_dir.as_path()).args([
            "build",
            "--release",
            "--all-targets",
            "--verbose",
        ]);
        // we need to remove any environment variables starting with RUST/CARGO for the child process
        for (key, _value) in env::vars() {
            if key.starts_with("CARGO") || key.starts_with("RUST") {
                cmd.env_remove(key);
            }
        }

        cmd.spawn()
            .expect("Could not build WASM shim")
            .wait()
            .unwrap();

        let mut module_path = shim_dir.clone();
        module_path.extend([
            "target",
            "wasm32-unknown-unknown",
            "release",
            "simdutf8_wasm_shim.wasm",
        ]);
        if !module_path.is_file() {
            panic!("Expected the WASM shim module at {:?}", module_path);
        }
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        let mut out_file = File::create(out_path.join("wasm_shim.rs"))
            .expect("Could not create WASM shim Rust file");
        writeln!(
            &mut out_file,
            "const WASM_SHIM_CODE: &[u8] = include_bytes!({:?});",
            module_path
        )
        .expect("Could not write to WASM shim module");
    }
}
