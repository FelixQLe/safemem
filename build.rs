fn main() {
    cc::Build::new()
        .file("c_src/buffer.c")
        .compile("buffer");
}