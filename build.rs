use std::env;

fn main() {
    // Compile C code
    cc::Build::new()
        .file("c_src/buffer.c")
        .include("c_src")
        .compile("buffer");

    // Tell cargo to look for libraries in the specified directory
    println!("cargo:rustc-link-search=native={}", env::var("OUT_DIR").unwrap());
    
    // Rerun if C source files change
    println!("cargo:rerun-if-changed=c_src/buffer.c");
    println!("cargo:rerun-if-changed=c_src/buffer.h");
}