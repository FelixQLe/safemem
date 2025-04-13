pub struct BufferR {
    data: Vec<u8>,
}

impl BufferR {
    // Create a new buffer with initial capacity
    pub fn new(initial_capacity: usize) -> Self {
        BufferR {
            data: Vec::with_capacity(initial_capacity),
        }
    }

    // Append data to the buffer - safe by default
    pub fn append(&mut self, input_data: &[u8]) {
        self.data.extend_from_slice(input_data);
    }

    // Other methods remain the same...
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }
    
    pub fn print(&self) {
        println!("Buffer content ({} bytes): ", self.data.len());
        for byte in &self.data {
            print!("{:02x} ", byte);
        }
        println!();
    }
}