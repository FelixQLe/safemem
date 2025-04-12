use std::ffi::{c_char, CString};

use bindings::buffer_double_free;

use crate::bindings::{
    buffer_init, buffer_append,
    buffer_free, buffer_print
};

mod bindings;

// Helper function to run a closure and catch any panics
fn C_run_test_case<F>(name: &str, test_func: F)
where
    F: FnOnce() + std::panic::UnwindSafe,
{
    println!("\n=== C Running Test: {} ===", name);
    let result = std::panic::catch_unwind(test_func);
    match result {
        Ok(_) => println!("C Test completed without crash"),
        Err(_) => println!("C Test crashed (expected for vulnerable code)"),
    }
}

// Test cases for buffer overflow
fn C_test_buffer_overflow_1() {
    unsafe {
        let buffer = buffer_init(10);
        println!("C created buffer with capacity 10");
        
        // Append data that fits
        let c_string = CString::new("Hello").unwrap(); // CString adds the null terminator auto
        buffer_append(&mut *buffer, c_string.as_ptr(), 5);
        buffer_print(buffer);
        
        // Append data that exceeds capacity (will cause overflow)
        println!("C attempting to append 10 more bytes when only 5 bytes remain...");
        let append_c_string = CString::new(" Wolrd!!!").unwrap();
        buffer_append(buffer, append_c_string.as_ptr(), 10);
        
        // This might crash or show corrupted data
        println!("C after overflow: ");
        buffer_print(buffer);
        
        //buffer_free(buffer);
        println!("C Test completed");
    }
}

fn C_test_buffer_overflow_2() {
    unsafe  {
        let buffer = buffer_init(5);
        println!("C created buffer with capacity 5");

        // Massive overflow
        println!("C attempting to append 50 bytes to a 5 bytes buffer...");
        let large_data = vec![b'X' as c_char; 50];
        //let large_string = CString::new(large_data).unwrap();
        buffer_append(&mut *buffer, large_data.as_ptr(), 50);

        // This might crash
        println!("C after massive overflow: ");
        buffer_print(buffer);

        buffer_free(buffer);
        println!("C Test completed")
    }
}


// Test cases for double-free
fn C_test_double_free_1() {
    unsafe {
        let buffer = buffer_init(10);
        println!("C created buffer");
        
        buffer_append(buffer, "Test\0".as_ptr() as *const c_char, 4);
        buffer_print(buffer);
        
        // First free (correct)
        println!("C freeing buffer first time...");
        buffer_free(buffer);
        
        // Second free (vulnerability)
        println!("C attempting to free buffer second time...");
        buffer_double_free(buffer); // This will cause a double-free error
        
        println!("C Test completed");
    }
}

fn C_test_double_free_2() {
    unsafe {
        let buffer = buffer_init(10);
        println!("C created buffer");
        
        // Free immediately
        println!("C freeing buffer first time...");
        buffer_free(buffer);
        
        // Wait a bit
        println!("Waiting briefly...");
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        // Double free after some time
        println!("C attempting to free buffer second time after delay...");
        buffer_double_free(buffer); // This will cause a double-free error
        
        println!("C Test completed");
    }
}

// Test cases for use-after-free
fn C_test_use_after_free_1() {
    unsafe {
        let buffer = buffer_init(10);
        println!("C created buffer");
        
        buffer_append(buffer, "Hello\0".as_ptr() as *const c_char, 5);
        buffer_print(buffer);
        
        // Free the buffer
        println!("C freeing buffer...");
        buffer_free(buffer);
        
        // Use after free: try to access data
        println!("C attempting to access data after free...");
        buffer_print(buffer); // Accessing freed memory
        
        println!("C Test completed");
    }
}

fn C_test_use_after_free_2() {
    unsafe {
        let buffer = buffer_init(10);
        println!("C created buffer");
        
        buffer_append(buffer, "Hello\0".as_ptr() as *const c_char, 5);
        buffer_print(buffer);
        
        // Free the buffer
        println!("C freeing buffer...");
        buffer_free(buffer);
        
        // Use after free: try to append more data
        println!("C attempting to append data after free...");
        buffer_append(buffer, " World\0".as_ptr() as *const c_char, 6); // Writing to freed memory
        
        // Try to print after use-after-free
        println!("C after use-after-free append: ");
        buffer_print(buffer); // This will likely crash
        
        println!("Test completed");
    }
}


fn main() {
    println!("Starting buffer manager vulnerability tests");
    println!("WARNING: These tests will might trigger memory errors");
    
    // Run each test in its own scope to prevent crashes from affecting other tests
    C_run_test_case("Buffer Overflow Test 1", C_test_buffer_overflow_1);
    C_run_test_case("Buffer Overflow Test 2", C_test_buffer_overflow_2);
    C_run_test_case("Double Free Test 1", C_test_double_free_1);
    C_run_test_case("Double Free Test 2", C_test_double_free_2);
    C_run_test_case("Use After Free Test 1", C_test_use_after_free_1);
    C_run_test_case("Use After Free Test 2", C_test_use_after_free_2);
    
    println!("\nAll tests completed");
    
    use safemem::Buffer;

    println!("SafeMem: Rust Memory Safety Demo");

    // Run all tests manually
    println!("\n=== Running Rust Tests ===");

}