use crate::streaming::Stream;
use std::future::Future;

/// A stream that automatically attempts to reconnect upon encountering an error.
pub struct ReconnectableStream<S, F> {
    stream: S,
    reconnect_fn: F,
    max_retries: usize,
    current_retries: usize,
}

impl<S, F, Fut, T, E> ReconnectableStream<S, F>
where
    S: Stream<Item = Result<T, E>> + Send,
    F: FnMut() -> Fut + Send,
    Fut: Future<Output = Result<S, E>> + Send,
    T: Send,
    E: Send,
{
    /// Creates a new `ReconnectableStream`.
    pub fn new(stream: S, reconnect_fn: F, max_retries: usize) -> Self {
        Self {
            stream,
            reconnect_fn,
            max_retries,
            current_retries: 0,
        }
    }
}

impl<S, F, Fut, T, E> Stream for ReconnectableStream<S, F>
where
    S: Stream<Item = Result<T, E>> + Send,
    F: FnMut() -> Fut + Send,
    Fut: Future<Output = Result<S, E>> + Send,
    T: Send,
    E: Send,
{
    type Item = Result<T, E>;

    async fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stream.next().await {
                Some(Ok(item)) => {
                    self.current_retries = 0;
                    return Some(Ok(item));
                }
                Some(Err(e)) => {
                    if self.current_retries >= self.max_retries {
                        return Some(Err(e));
                    }
                    self.current_retries += 1;
                    match (self.reconnect_fn)().await {
                        Ok(new_stream) => {
                            self.stream = new_stream;
                        }
                        Err(reconnect_err) => {
                            return Some(Err(reconnect_err));
                        }
                    }
                }
                None => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::StreamingError;

    struct MockErrorStream {
        items: Vec<Result<String, StreamingError>>,
    }

    impl Stream for MockErrorStream {
        type Item = Result<String, StreamingError>;

        async fn next(&mut self) -> Option<Self::Item> {
            if self.items.is_empty() {
                None
            } else {
                Some(self.items.remove(0))
            }
        }
    }

    #[tokio::test]
    async fn test_reconnect_success() {
        let stream = MockErrorStream {
            items: vec![
                Ok("chunk 1".to_string()),
                Err(StreamingError::new("connection lost")),
            ],
        };

        let mut reconnects = 0;
        let reconnect_fn = || {
            reconnects += 1;
            async move {
                Ok(MockErrorStream {
                    items: vec![Ok("chunk 2".to_string()), Ok("chunk 3".to_string())],
                })
            }
        };

        let mut recon_stream = ReconnectableStream::new(stream, reconnect_fn, 3);

        assert_eq!(recon_stream.next().await.unwrap().unwrap(), "chunk 1");
        assert_eq!(recon_stream.next().await.unwrap().unwrap(), "chunk 2");
        assert_eq!(recon_stream.next().await.unwrap().unwrap(), "chunk 3");
        assert!(recon_stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_reconnect_failure_max_retries() {
        let stream = MockErrorStream {
            items: vec![Err(StreamingError::new("connection lost"))],
        };

        let reconnect_fn = || async move {
            Ok(MockErrorStream {
                items: vec![Err(StreamingError::new("still broken"))],
            })
        };

        let mut recon_stream = ReconnectableStream::new(stream, reconnect_fn, 2);

        // Fails after retries exhaust
        let res = recon_stream.next().await;
        assert!(res.is_some());
        assert!(res.unwrap().is_err());
    }
}
