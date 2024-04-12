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

    builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Failed to generate bindings for libnuma")
        .write_to_file(out.join("bindgen.rs"))
        .expect("Failed to write bindings");
}
