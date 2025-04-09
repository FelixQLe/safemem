use std::ffi::CString;

// Define the Buffer struct to match C's definition
#[repr(C)]
struct Buffer {
    data: *mut u8,    // char* in C becomes *mut u8 in Rust
    size: usize,      // size_t maps to usize
    capacity: usize,  // size_t maps to usize
}

// Mark extern "C" block as unsafe
unsafe extern "C" {
    fn buffer_new(capacity: usize) -> *mut Buffer;
    fn buffer_append(buf: *mut Buffer, s: *const libc::c_char);
    fn buffer_free(buf: *mut Buffer);
}

fn test_c_vulnerabilities() {
    println!("Testing C vulnerabilities:");

    // Buffer Overflow Tests
    println!("\nBuffer Overflow Tests:");
    unsafe {
        // Test 1: Small Overflow
        let c_buf1 = buffer_new(4);
        let hello = CString::new("Hello").unwrap();  // 5 chars + null > 4
        buffer_append(c_buf1, hello.as_ptr());
        println!("Small overflow: Appended 'Hello' to capacity=4 (may corrupt memory)");
        buffer_free(c_buf1);

        // Test 2: Large Overflow
        let c_buf2 = buffer_new(4);
        let big = CString::new("A".repeat(1000)).unwrap();
        buffer_append(c_buf2, big.as_ptr());
        println!("Large overflow: Appended 1000 'A's to capacity=4 (wild overwrite)");
        buffer_free(c_buf2);
    }
}

fn main() {
    test_c_vulnerabilities();
}