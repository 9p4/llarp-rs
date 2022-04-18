extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .enable_cxx_namespaces()
        .raw_line("pub use self::root::*;")
        .header("wrapper.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++20")
        .clang_arg("-stdlib=libc++")
        .allowlist_recursively(false)
        .allowlist_type("Test")
        .opaque_type("std::.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
