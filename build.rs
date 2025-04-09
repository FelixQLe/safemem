fn main() {
    cc::Build::new()
        .file("c_src/buffer.c")
        .compile("buffer");  // Compiles to libbuffer.a

    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=buffer");  // Link libbuffer.a
}