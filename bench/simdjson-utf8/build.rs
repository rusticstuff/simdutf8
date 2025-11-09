use std::{env, path::PathBuf, process::Command};

fn get_cpp_link_stdlib() -> Option<String> {
    if let Ok(stdlib) = env::var("CXXSTDLIB") {
        if stdlib.is_empty() {
            None
        } else {
            Some(stdlib)
        }
    } else {
        let target = env::var("TARGET").unwrap();
        if target.contains("msvc") {
            None
        } else if target.contains("apple")
            || target.contains("freebsd")
            || target.contains("openbsd")
        {
            Some("c++".to_string())
        } else {
            Some("stdc++".to_string())
        }
    }
}

fn main() {
    let version = "0.9.2";
    let download_url =
        "https://github.com/simdjson/simdjson/archive/refs/tags/v".to_owned() + version + ".tar.gz";
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    eprintln!("download");
    assert!(Command::new("curl")
        .args(["-L", "-o", "simdjson.tar.gz", &download_url])
        .current_dir(&out_path)
        .status()
        .unwrap()
        .success());
    eprintln!("download done");

    eprintln!("extracting");
    Command::new("tar")
        .args(["xzf", "simdjson.tar.gz"])
        .current_dir(&out_path)
        .status()
        .unwrap();
    eprintln!("extracting done");

    eprintln!("building");
    let simdjson_dir = out_path.as_os_str().to_str().unwrap().to_owned() + "/simdjson-" + version;
    let dst = cmake::Config::new(&simdjson_dir)
        .define("SIMDJSON_JUST_LIBRARY", "ON")
        .define("SIMDJSON_BUILD_STATIC", "ON")
        // .define("CMAKE_CXX_COMPILER", "clang")
        // .define("CMAKE_C_COMPILER", "clang")
        // .define("CMAKE_CXX_COMPILER", "gcc")
        // .define("CMAKE_C_COMPILER", "gcc")
        .uses_cxx11()
        .build();
    eprintln!("building done");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=simdjson");

    if let Some(stdlib) = get_cpp_link_stdlib() {
        println!("cargo:rustc-link-lib={}", stdlib);
    }

    eprintln!("bindings");
    let bindings = bindgen::Builder::default()
        .clang_args(&[
            "-xc++",
            "-std=c++17",
            &("-I".to_owned() + &simdjson_dir + "/include"),
        ])
        .header(&(simdjson_dir.to_owned() + "/include/simdjson/implementation.h"))
        .whitelist_function("simdjson::validate_utf8")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("unable to generate bindings");
    eprintln!("bindings done");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
