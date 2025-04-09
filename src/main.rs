use std::ffi::CString;

extern "C" {
    fn buffer_new(capacity: usize) -> *mut Buffer;
    fn buffer_append(buf: *mut Buffer, s: *const libc::c_char);
    fn buffer_free(buf: *mut Buffer);
}

fn test_c_vulnerabilities() {
    println!("Testing C vulnerabilities:");

    // Buffer Overflows
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

    // Use-After-Free
    println!("\nUse-After-Free Tests:");
    unsafe {
        // Test 1: Simple Reuse
        let c_buf3 = buffer_new(4);
        buffer_free(c_buf3);
        let world = CString::new("World").unwrap();
        buffer_append(c_buf3, world.as_ptr());
        println!("Simple reuse: Appended 'World' after free (heap corruption likely)");

        // Test 2: Double Append
        let c_buf4 = buffer_new(4);
        buffer_free(c_buf4);
        let foo = CString::new("Foo").unwrap();
        buffer_append(c_buf4, foo.as_ptr());
        let bar = CString::new("Bar").unwrap();
        buffer_append(c_buf4, bar.as_ptr());
        println!("Double append: Appended 'Foo' then 'Bar' after free (escalating chaos)");
    }

    // Double-Free
    println!("\nDouble-Free Tests:");
    unsafe {
        // Test 1: Basic Double-Free
        let c_buf5 = buffer_new(4);
        buffer_free(c_buf5);
        buffer_free(c_buf5);
        println!("Basic double-free: Freed twice (heap corruption or crash)");

        // Test 2: Free-and-Reuse
        let c_buf6 = buffer_new(4);
        let test = CString::new("Test").unwrap();
        buffer_free(c_buf6);
        buffer_append(c_buf6, test.as_ptr());
        buffer_free(c_buf6);
        println!("Free-and-reuse: Freed, appended 'Test', freed again (double trouble)");
    }
}

fn main() {
    test_c_vulnerabilities();
}