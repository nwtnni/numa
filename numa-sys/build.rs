use std::env;
use std::path::PathBuf;

fn main() {
    let config = pkg_config::Config::new()
        .probe("numa")
        .expect("Failed to find libnuma");

    let out = env::var("OUT_DIR")
        .map(PathBuf::from)
        .expect("Failed to read Cargo OUT_DIR environment variable");

    let mut builder = bindgen::builder().header("bindgen.h");

    for include in &config.include_paths {
        builder = builder.clang_arg(format!("-I{}", include.display()));
    }

    if cfg!(feature = "static") {
        println!("cargo:rustc-link-lib=static=numa");
    } else if cfg!(feature = "dynamic") {
        println!("cargo:rustc-link-lib=numa");
    } else {
        panic!("Must set either static or dynamic feature to select linking.");
    }

    builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Failed to generate bindings for libnuma")
        .write_to_file(out.join("bindgen.rs"))
        .expect("Failed to write bindings");
}
