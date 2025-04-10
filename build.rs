fn main() {
    cc::Build::new()
        .file("c_src/buffer.c")
        .compile("buffer");  // Compiles to libbuffer.a

    // let out_dir = std::env::var("OUT_DIR").unwrap();
    let path  = "target/debug/build/safemem_c-to-rust_memory_safety_demo-08ce96ce21c89762/out/libbuffer.a";
    println!("cargo:rustc-link-search=native={}", path);
    println!("cargo:rustc-link-lib=static=buffer");  // Link libbuffer.a
}