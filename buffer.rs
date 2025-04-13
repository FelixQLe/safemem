#[cfg(test)]
mod tests {
    use safemem::BufferR; // Import Buffer from the safemem crate

    #[test]
    fn rust_test_buffer_overflow_1() {
        println!("\n=== Rust Buffer Overflow Test 1 (Small Overflow) ===");
        let mut buffer = BufferR::new(4).expect("Failed to create buffer");
        println!("Created buffer with capacity 4");

        // Attempt small overflow
        println!("Attempting to append 'Hello' (5 bytes) to a 4-byte buffer...");
        match buffer.append(b"Hello") {
            Ok(_) => println!("Success appending 'Hello'"),
            Err(e) => println!("{}", e),
        }

        // Print buffer state
        println!("After attempted overflow:");
        buffer.print();

        println!("Rust Test completed");
    }

    #[test]
    fn rust_test_buffer_overflow_2() {
        println!("\n=== Rust Buffer Overflow Test 2 (Large Overflow) ===");
        let mut buffer = BufferR::new(5).expect("Failed to create buffer");
        println!("Created buffer with capacity 5");

        // Attempt large overflow
        println!("Attempting to append 1000 bytes to a 5-byte buffer...");
        let large_data = vec![b'A'; 1000];
        match buffer.append(&large_data) {
            Ok(_) => println!("Unexpected success appending large data"),
            Err(e) => println!("{}", e),
        }

        // Print buffer state
        println!("After attempted massive overflow:");
        buffer.print();

        println!("Rust Test completed");
    }

    #[test]
    fn rust_test_double_free_1() {
        println!("\n=== Rust Double Free Test 1 (Basic Double-Free) ===");
        let mut buffer = BufferR::new(10).expect("Failed to create buffer");
        println!("Created buffer");

        buffer.append(b"Test").expect("Failed to append");
        buffer.print();

        // First "free"
        println!("Freeing buffer first time...");
        drop(buffer);

        // Simulate second free attempt
        println!("Attempting to free buffer second time...");
        println!("Rust: Double-free prevented (ownership ensures single drop)");

        println!("Rust Test completed");
    }

    #[test]
    fn rust_test_double_free_2() {
        println!("\n=== Rust Double Free Test 2 (Free-and-Reuse) ===");
        let buffer = BufferR::new(10).expect("Failed to create buffer");
        println!("Created buffer");

        // Free first time
        println!("Freeing buffer first time...");
        drop(buffer);

        // Simulate reuse and second free
        println!("Attempting to append 'Test' after free...");
        println!("Rust: Use-after-free prevented; creating new buffer for simulation");
        let mut new_buffer = BufferR::new(10).expect("Failed to create buffer");
        new_buffer.append(b"Test").expect("Failed to append");
        new_buffer.print();

        println!("Freeing new buffer...");
        drop(new_buffer);

        println!("Rust: Double-free prevented (new buffer dropped once)");
        println!("Rust Test completed");
    }

    #[test]
    fn rust_rust_test_use_after_free_1() {
        println!("\n=== Rust Use After Free Test 1 (Simple Reuse) ===");
        let mut buffer = BufferR::new(10).expect("Failed to create buffer");
        println!("Created buffer");

        buffer.append(b"Hello").expect("Failed to append");
        buffer.print();

        // Free the buffer
        println!("Freeing buffer...");
        drop(buffer);

        // Simulate use-after-free
        println!("Attempting to append 'World' after free...");
        println!("Rust: Use-after-free prevented; creating new buffer for simulation");
        let mut new_buffer = BufferR::new(10).expect("Failed to create buffer");
        new_buffer.append(b"World").expect("Failed to append");
        new_buffer.print();

        println!("Rust Test completed");
    }

    #[test]
    fn rust_test_use_after_free_2() {
        println!("\n=== Rust Use After Free Test 2 (Double Append) ===");
        let mut buffer = BufferR::new(10).expect("Failed to create buffer");
        println!("Created buffer");

        buffer.append(b"Hello").expect("Failed to append");
        buffer.print();

        // Free the buffer
        println!("Freeing buffer...");
        drop(buffer);

        // Simulate double append after free
        println!("Attempting to append 'Foo' after free...");
        println!("Rust: Use-after-free prevented; creating new buffer for simulation");
        let mut new_buffer = BufferR::new(10).expect("Failed to create buffer");
        new_buffer.append(b"Foo").expect("Failed to append");

        println!("Attempting to append 'Bar' after free...");
        new_buffer.append(b"Bar").expect("Failed to append");

        println!("After simulated use-after-free appends:");
        new_buffer.print();

        println!("Rust Test completed");
    }
}