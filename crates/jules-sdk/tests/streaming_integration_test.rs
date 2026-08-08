use jules_api::streaming::buffer::ChunkBuffer;
use jules_api::streaming::sse::SseParser;
use jules_core::errors::StreamingError;
use jules_core::streaming::{reconnect::ReconnectableStream, Stream, StreamEvent};

#[test]
fn test_buffer_and_parse_integration() {
    let mut buffer = ChunkBuffer::new(1024);
    let mut parser = SseParser::new();

    let chunks = vec!["data: hello\n", "\n", "data: world", "\n\n"];

    let mut events = Vec::new();
    for chunk in chunks {
        buffer.push(chunk).unwrap();
        let drained = buffer.drain(1024);
        let parsed = parser.push(&drained);
        events.extend(parsed);
    }

    assert_eq!(events.len(), 2);
    assert_eq!(events[0].data, "hello");
    assert_eq!(events[1].data, "world");
}

#[test]
fn test_streaming_reconnect() {
    struct MockErrorStream {
        items: Vec<Result<StreamEvent, StreamingError>>,
    }

    impl Stream for MockErrorStream {
        type Item = Result<StreamEvent, StreamingError>;

        async fn next(&mut self) -> Option<Self::Item> {
            if self.items.is_empty() {
                None
            } else {
                Some(self.items.remove(0))
            }
        }
    }

    let stream = MockErrorStream {
        items: vec![
            Ok(StreamEvent::TextChunk("chunk 1".to_string())),
            Err(StreamingError::new("connection lost")),
        ],
    };

    let reconnect_fn = || async move {
        Ok(MockErrorStream {
            items: vec![
                Ok(StreamEvent::TextChunk("chunk 2".to_string())),
                Ok(StreamEvent::Done),
            ],
        })
    };

    let mut recon_stream = ReconnectableStream::new(stream, reconnect_fn, 3);

    // Create simple block_on for the future to avoid adding tokio as a dev-dependency just for one test
    let f = async {
        let first = recon_stream.next().await.unwrap().unwrap();
        assert_eq!(first, StreamEvent::TextChunk("chunk 1".to_string()));
        let second = recon_stream.next().await.unwrap().unwrap();
        assert_eq!(second, StreamEvent::TextChunk("chunk 2".to_string()));
        let third = recon_stream.next().await.unwrap().unwrap();
        assert_eq!(third, StreamEvent::Done);
        assert!(recon_stream.next().await.is_none());
    };

    // Poor man's block_on
    let waker = std::task::Waker::noop();
    let mut cx = std::task::Context::from_waker(waker);
    let mut future = std::boxed::Box::pin(f);
    let mut iters = 0;
    while std::future::Future::poll(future.as_mut(), &mut cx).is_pending() {
        iters += 1;
        assert!(iters <= 100, "future didn't resolve");
    }
}
