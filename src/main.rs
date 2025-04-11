use std::ffi::c_void;
use std::os::raw::c_char;
use core::ffi::c_size_t;

// FFI declarations to call C buffer manager functions
#[link(name = "buffer")]
unsafe extern "C" {
    fn buffer_init(initial_capacity: c_size_t) -> *mut c_void;
    fn buffer_append(buffer: *mut c_void, data: *const c_char, length: c_size_t);
    fn buffer_free(buffer: *mut c_void);
    fn buffer_print(buffer: *mut c_void);
    fn buffer_size(buffer: *mut c_void) -> c_size_t;
    fn buffer_data(buffer: *mut c_void) -> *mut c_char;
}

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
        buffer_append(buffer, "Hello\0".as_ptr() as *const c_char, 5);
        buffer_print(buffer);
        
        // Append data that exceeds capacity (will cause overflow)
        println!("Attempting to append 10 more bytes when only 5 bytes remain...");
        buffer_append(buffer, " World!!!\0".as_ptr() as *const c_char, 10);
        
        // This might crash or show corrupted data
        println!("After overflow: ");
        buffer_print(buffer);
        
        buffer_free(buffer);
        println!("Test completed");
    }
}

fn test_buffer_overflow_2() {
    unsafe {
        let buffer = buffer_init(5);
        println!("Created buffer with capacity 5");
        
        // Massive overflow
        println!("Attempting to append 50 bytes to a 5-byte buffer...");
        let large_data = vec![b'X' as c_char; 50];
        buffer_append(buffer, large_data.as_ptr(), 50);
        
        // This will likely crash
        println!("After massive overflow: ");
        buffer_print(buffer);
        
        buffer_free(buffer);
        println!("Test completed");
    }
}

// Test cases for double-free
fn test_double_free_1() {
    unsafe {
        let buffer = buffer_init(10);
        println!("Created buffer");
        
        buffer_append(buffer, "Test\0".as_ptr() as *const c_char, 4);
        buffer_print(buffer);
        
        // First free (correct)
        println!("Freeing buffer first time...");
        buffer_free(buffer);
        
        // Second free (vulnerability)
        println!("Attempting to free buffer second time...");
        buffer_free(buffer); // This will cause a double-free error
        
        println!("Test completed");
    }
}

fn test_double_free_2() {
    unsafe {
        let buffer = buffer_init(10);
        println!("Created buffer");
        
        // Free immediately
        println!("Freeing buffer first time...");
        buffer_free(buffer);
        
        // Wait a bit
        println!("Waiting briefly...");
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        // Double free after some time
        println!("Attempting to free buffer second time after delay...");
        buffer_free(buffer); // This will cause a double-free error
        
        println!("Test completed");
    }
}

// Test cases for use-after-free
fn test_use_after_free_1() {
    unsafe {
        let buffer = buffer_init(10);
        println!("Created buffer");
        
        buffer_append(buffer, "Hello\0".as_ptr() as *const c_char, 5);
        buffer_print(buffer);
        
        // Free the buffer
        println!("Freeing buffer...");
        buffer_free(buffer);
        
        // Use after free: try to access data
        println!("Attempting to access data after free...");
        buffer_print(buffer); // Accessing freed memory
        
        println!("Test completed");
    }
}

fn test_use_after_free_2() {
    unsafe {
        let buffer = buffer_init(10);
        println!("Created buffer");
        
        buffer_append(buffer, "Hello\0".as_ptr() as *const c_char, 5);
        buffer_print(buffer);
        
        // Free the buffer
        println!("Freeing buffer...");
        buffer_free(buffer);
        
        // Use after free: try to append more data
        println!("Attempting to append data after free...");
        buffer_append(buffer, " World\0".as_ptr() as *const c_char, 6); // Writing to freed memory
        
        // Try to print after use-after-free
        println!("After use-after-free append: ");
        buffer_print(buffer); // This will likely crash
        
        println!("Test completed");
    }
}

fn main() {
    println!("Starting buffer manager vulnerability tests");
    println!("WARNING: These tests will intentionally trigger memory errors");
    
    // Run each test in its own scope to prevent crashes from affecting other tests
    run_test_case("Buffer Overflow Test 1", test_buffer_overflow_1);
    run_test_case("Buffer Overflow Test 2", test_buffer_overflow_2);
    run_test_case("Double Free Test 1", test_double_free_1);
    run_test_case("Double Free Test 2", test_double_free_2);
    run_test_case("Use After Free Test 1", test_use_after_free_1);
    run_test_case("Use After Free Test 2", test_use_after_free_2);
    
    println!("\nAll tests completed");
}