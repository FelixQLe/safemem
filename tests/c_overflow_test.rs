use std::ffi::CString;

use crate::bindings::{
    buffer_init, buffer_append,
    buffer_capacity,
    buffer_size,
};
#[allow(unused_imports)]
use safemem::BufferR;
mod bindings;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_test_buffer_overflow() {
        unsafe {
            let buffer = buffer_init(10);
            println!("C created buffer with capacity 10");
            
            // Append data that fits
            let c_string = CString::new("Hello").unwrap(); // CString adds the null terminator auto
            buffer_append(&mut *buffer, c_string.as_ptr(), 5);
            println!("Buffer size: {}, buffer capacity: {:?}", 
            buffer_size(buffer), buffer_capacity(buffer));
            
            // Append data that exceeds capacity (will cause overflow)
            println!("C attempting to append 10 more bytes when only 5 bytes remain...");
            let append_c_string = CString::new(" Wolrd!!!").unwrap();
            buffer_append(buffer, append_c_string.as_ptr(), 10);
            
            // This might crash or show corrupted data
            println!("Buffer size: {}, buffer capacity: {:?} after overflow", 
                        buffer_size(buffer), buffer_capacity(buffer));
            
            //buffer_free(buffer);
            println!("C Test completed");
        }
}
}

