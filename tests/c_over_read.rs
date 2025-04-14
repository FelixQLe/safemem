use std::ffi::CString;

use crate::bindings::{
    buffer_init, buffer_append,
    buffer_free,
};
#[allow(unused_imports)]
use safemem::BufferR;
mod bindings;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_over_read() {
        println!("\n=== Buffer Over-Read Test 1 ===\n");
        unsafe {
            let buffer = buffer_init(10);
            println!("C created buffer");

            // Append data that fits
            let c_string = CString::new("Hello").unwrap(); // CString adds the null terminator auto
            buffer_append(&mut *buffer, c_string.as_ptr(), 5);
            
            // Try to read beyond the valid data
            println!("Attempting to read beyond buffer size...\n");
            println!("Valid buffer capacity: {} bytes\n", (*buffer).capacity);
            println!("Attempting to read 5 bytes over buffer capacity...\n");
            
            // Intentional buffer over-read
            for i in 0..15 {
                let byte = *(*buffer).data.add(i);
                let c = byte as u8; // cast i8 to u8
                // attempt to access raw data over the
                println!("Byte at position {}: {} ({})", 
                        i, if c >= 32 && c <= 126 {c as char} else {'.'},
                    c);
            }

        buffer_free(buffer);
        println!("Test completed\n");
        }
    }
}