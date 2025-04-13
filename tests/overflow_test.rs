use std::ffi::CString;

use crate::bindings::{
    buffer_init, buffer_append,
    buffer_free, buffer_capacity,
    buffer_data, buffer_size,
};

use safemem::BufferR;
mod bindings;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_test_buffer_overflow() {
        println!("\n=== Buffer Overflow Test (Rust Implementation) ===");
        
        // Create a small buffer
        let mut buffer = BufferR::new(10);
        println!("Created buffer with capacity 10");
        
        // Add data within capacity
        buffer.append(b"Hello");
        println!("Added 5 bytes");
        buffer.print();
        
        // Try to add data exceeding capacity
        println!("Attempting to add 59 more bytes (exceeds initial capacity)...");
        buffer.append(b"This is a longer string I want to input to Rust buffer data");
        
        // Rust Vec will automatically resize - no overflow!
        println!("Rust safely handled the potential overflow by resizing the buffer");
        buffer.print();
        
        assert!(buffer.get_size() == 64);
        println!("Test passed: Buffer correctly contains all 64 bytes");
} 
    #[test]
    fn c_test_buffer_overflow_1() {
        unsafe {
            let buffer = buffer_init(10);
            println!("C created buffer with capacity 10");
            
            // Append data that fits
            let c_string = CString::new("Hello").unwrap(); // CString adds the null terminator auto
            buffer_append(&mut *buffer, c_string.as_ptr(), 5);
            println!("Buffer size: {}, buffer data: {:?}", 
            buffer_size(buffer), buffer_data(buffer));
            
            // Append data that exceeds capacity (will cause overflow)
            println!("C attempting to append 10 more bytes when only 5 bytes remain...");
            let append_c_string = CString::new(" Wolrd!!!").unwrap();
            buffer_append(buffer, append_c_string.as_ptr(), 10);
            
            // This might crash or show corrupted data
            println!("Buffer size: {}, buffer data: {:?} after overflow", 
                        buffer_size(buffer), buffer_data(buffer));
            
            //buffer_free(buffer);
            println!("C Test completed");
        }
}

    #[test]
    fn c_test_buffer_overflow_2() {
        unsafe {
            let buffer = buffer_init(5);
            println!("C created buffer with capacity 5");
            println!("Buffer size: {}, buffer data: {:?}", 
            buffer_size(buffer), buffer_data(buffer));

            // Massive overflow
            println!("C attempting to append 50 bytes to a 5 bytes buffer...");
            let large_string = b"This the very large string will be entered to buffer";
            let large_data = CString::new(large_string).unwrap();
            buffer_append(&mut *buffer, large_data.as_ptr(), 52);
            // This might crash
            println!("C after massive overflow: ");
            println!("Buffer size: {}, buffer data: {:?}", 
            buffer_size(buffer), buffer_data(buffer));
            buffer_free(buffer);
            
            println!("C Test completed")
        }
    }
}

