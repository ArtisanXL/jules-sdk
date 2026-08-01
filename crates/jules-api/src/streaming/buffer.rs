//! Chunk buffer for streaming responses.

/// Error indicating that the `ChunkBuffer` capacity has been exceeded.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("Chunk buffer capacity exceeded")]
pub struct BufferCapacityError;

/// Buffer that accumulates chunks and enforces a maximum byte capacity.
#[derive(Debug, Default)]
pub struct ChunkBuffer {
    buffer: String,
    max_capacity: usize,
}

impl ChunkBuffer {
    /// Creates a new `ChunkBuffer` with the specified maximum capacity.
    #[must_use]
    pub fn new(max_capacity: usize) -> Self {
        Self {
            buffer: String::with_capacity(max_capacity.min(8192)),
            max_capacity,
        }
    }

    /// Pushes a string chunk into the buffer.
    ///
    /// # Errors
    ///
    /// Returns `BufferCapacityError` if pushing the chunk would exceed the maximum capacity.
    pub fn push(&mut self, chunk: &str) -> Result<(), BufferCapacityError> {
        if self.buffer.len() + chunk.len() > self.max_capacity {
            return Err(BufferCapacityError);
        }
        self.buffer.push_str(chunk);
        Ok(())
    }

    /// Drains up to `max_bytes` from the beginning of the buffer.
    pub fn drain(&mut self, max_bytes: usize) -> String {
        let drain_len = self.buffer.len().min(max_bytes);

        // Ensure we split at a char boundary.
        let mut split_idx = drain_len;
        while split_idx > 0 && !self.buffer.is_char_boundary(split_idx) {
            split_idx -= 1;
        }

        let drained = self.buffer[..split_idx].to_string();
        self.buffer = self.buffer[split_idx..].to_string();
        drained
    }

    /// Returns the current number of bytes in the buffer.
    #[must_use]
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns whether the buffer is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_drain() {
        let mut buffer = ChunkBuffer::new(100);
        buffer.push("hello").unwrap();
        buffer.push(" world").unwrap();

        assert_eq!(buffer.len(), 11);

        let drained = buffer.drain(5);
        assert_eq!(drained, "hello");
        assert_eq!(buffer.len(), 6);

        let drained2 = buffer.drain(10);
        assert_eq!(drained2, " world");
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_capacity_exceeded() {
        let mut buffer = ChunkBuffer::new(10);
        buffer.push("12345").unwrap();

        let res = buffer.push("678901");
        assert_eq!(res.unwrap_err(), BufferCapacityError);
        assert_eq!(buffer.len(), 5); // Unchanged
    }

    #[test]
    fn test_char_boundary() {
        let mut buffer = ChunkBuffer::new(100);
        // '🚀' is 4 bytes
        buffer.push("a🚀b").unwrap();

        // Try to drain 2 bytes, should split before the rocket.
        let drained = buffer.drain(2);
        assert_eq!(drained, "a");

        let drained2 = buffer.drain(10);
        assert_eq!(drained2, "🚀b");
    }
}
