use std::ffi::CString;

use crate::bindings::{
    Buffer, buffer_init, buffer_append,
    buffer_data, buffer_free, buffer_print
};

mod bindings;

// Helper function to run a closure and catch any panics
fn run_test_case<F>(name: &str, test_func: F)
where
    F: FnOnce() + std::panic::UnwindSafe,
{
    println!("\n=== Running Test: {} ===", name);
    let result = std::panic::catch_unwind(test_func);
    match result {
        Ok(_) => println!("Test completed without crash"),
        Err(_) => println!("Test crashed (expected for vulnerable code)"),
    }
}

// Test cases for buffer overflow
fn test_buffer_overflow_1() {
    unsafe {
        let buffer = buffer_init(10);
        println!("Created buffer with capacity 10");
        
        // Append data that fits
        let c_string = CString::new("Hello").unwrap(); // CString adds the null terminator auto
        buffer_append(&mut *buffer, c_string.as_ptr(), 5);
        buffer_print(buffer);
        
        // Append data that exceeds capacity (will cause overflow)
        println!("Attempting to append 10 more bytes when only 5 bytes remain...");
        let append_c_string = CString::new(" Wolrd!!!").unwrap();
        buffer_append(buffer, append_c_string.as_ptr(), 10);
        
        // This might crash or show corrupted data
        println!("After overflow: ");
        buffer_print(buffer);
        
        //buffer_free(buffer);
        println!("Test completed");
    }
}

fn main() {
    println!("Starting buffer manager vulnerability tests");
    println!("WARNING: These tests will might trigger memory errors");
    
    // Run each test in its own scope to prevent crashes from affecting other tests
    run_test_case("Buffer Overflow Test 1", test_buffer_overflow_1);
    
    println!("\nAll tests completed");
}