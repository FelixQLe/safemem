/// A safe buffer with a fixed capacity, designed to prevent memory safety issues.
#[derive(Debug)]
pub struct Buffer {
    data: Vec<u8>,
    capacity: usize,
}

impl Buffer {
    /// Creates a new buffer with the given capacity.
    pub fn new(capacity: usize) -> Result<Self, String> {
        if capacity == 0 {
            return Err(format!("Invalid capacity {} in Buffer::new", capacity));
        }
        Ok(Buffer {
            data: Vec::with_capacity(capacity),
            capacity,
        })
    }

    /// Appends data to the buffer, respecting capacity.
    pub fn append(&mut self, data: &[u8]) -> Result<(), String> {
        let new_len = self.data.len() + data.len();
        if new_len > self.capacity {
            return Err(format!(
                "Buffer overflow prevented (tried to append {} bytes, {} available)",
                data.len(),
                self.capacity - self.data.len()
            ));
        }
        self.data.extend_from_slice(data);
        Ok(())
    }

    /// Prints the buffer's contents as a string, if valid UTF-8.
    pub fn print(&self) {
        match std::str::from_utf8(&self.data) {
            Ok(s) => println!("Buffer: {}", s),
            Err(_) => println!("Buffer: <invalid UTF-8, {} bytes>", self.data.len()),
        }
    }

    /// Returns a reference to the buffer's data.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Returns the current length of the buffer.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}