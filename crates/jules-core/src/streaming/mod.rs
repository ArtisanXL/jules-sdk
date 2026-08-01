//! Streaming abstractions for incremental responses.

use serde::{Deserialize, Serialize};
use std::future::Future;

/// Represents an event in a stream of incremental responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamEvent {
    /// A chunk of text yielded incrementally.
    TextChunk(String),
    /// The stream has finished successfully.
    Done,
}

/// A stream of incremental responses.
pub trait Stream {
    /// The type of items yielded by the stream.
    type Item;

    /// Attempts to pull out the next value of this stream, returning `None` if the stream is exhausted.
    fn next(&mut self) -> impl Future<Output = Option<Self::Item>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockStream {
        count: u32,
    }

    impl Stream for MockStream {
        type Item = StreamEvent;

        fn next(&mut self) -> impl Future<Output = Option<Self::Item>> + Send {
            let res = if self.count < 3 {
                self.count += 1;
                Some(StreamEvent::TextChunk(format!("chunk {}", self.count)))
            } else if self.count == 3 {
                self.count += 1;
                Some(StreamEvent::Done)
            } else {
                None
            };
            async move { res }
        }
    }

    #[tokio::test]
    async fn test_mock_stream() {
        let mut stream = MockStream { count: 0 };
        assert_eq!(
            stream.next().await,
            Some(StreamEvent::TextChunk("chunk 1".to_string()))
        );
        assert_eq!(
            stream.next().await,
            Some(StreamEvent::TextChunk("chunk 2".to_string()))
        );
        assert_eq!(
            stream.next().await,
            Some(StreamEvent::TextChunk("chunk 3".to_string()))
        );
        assert_eq!(stream.next().await, Some(StreamEvent::Done));
        assert_eq!(stream.next().await, None);
    }
}
