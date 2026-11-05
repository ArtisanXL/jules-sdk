//! Streaming abstractions for incremental responses.

/// Reconnection logic for streams.
pub mod reconnect;
use serde::{Deserialize, Serialize};
use std::future::Future;

/// Represents an event in a stream of incremental responses.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamEvent {
    /// A chunk of text yielded incrementally.
    TextChunk(String),
    /// The stream has finished successfully.
    Done,
}

impl std::fmt::Debug for StreamEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TextChunk(_) => f.debug_tuple("TextChunk").field(&"***REDACTED***").finish(),
            Self::Done => write!(f, "Done"),
        }
    }
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
            let res = match self.count {
                0..=2 => {
                    self.count += 1;
                    Some(StreamEvent::TextChunk(format!("chunk {}", self.count)))
                }
                3 => {
                    self.count += 1;
                    Some(StreamEvent::Done)
                }
                _ => None,
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

    #[test]
    fn test_stream_event_debug_redacts_text_chunk() {
        let chunk = StreamEvent::TextChunk("secret data".to_string());
        let debug_str = format!("{chunk:?}");
        assert!(!debug_str.contains("secret data"));
        assert!(debug_str.contains("***REDACTED***"));

        let done = StreamEvent::Done;
        assert_eq!(format!("{done:?}"), "Done");
    }
}
