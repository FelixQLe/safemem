/* File: build.rs */
extern crate bindgen;
extern crate cc;
use std::path::PathBuf;

fn main() {

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=c_src/buffer.h");
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("c_src/buffer.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header file changed
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the src/bindings.rs file
    let out_path = PathBuf::from("tests");
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Build static library
    cc::Build::new()
        .file("c_src/buffer.c")
        .flag("-fsanitize=address")
        .flag("-fsanitize-recover=address")
        .compile("libbuffer.a");

    // Ensure the Rust linker includes ASan
    println!("cargo:rustc-link-lib=asan"); // Link against libasan
    println!("cargo:rustc-link-arg=-fsanitize=address"); // Pass ASan to linker
    println!("cargo:rustc-link-arg=-fsanitize-recover=address");//

}