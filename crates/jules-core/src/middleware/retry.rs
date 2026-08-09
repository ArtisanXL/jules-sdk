//! Retry middleware for Jules-SDK.
//!
//! Intercepts requests and retries them if they fail with a retriable error.

use super::{BoxFuture, Middleware, NextFn};
use crate::client::ClientRequest;
use crate::errors::SDKError;
use crate::response::ClientResponse;
use std::time::Duration;
use tracing::{debug, warn};

/// Configuration for the retry middleware.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts.
    pub max_attempts: u32,
    /// Delay between retries.
    pub delay: Duration,
    /// Backoff multiplier (e.g., 2.0 for exponential backoff).
    pub backoff_multiplier: f32,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            delay: Duration::from_millis(500),
            backoff_multiplier: 1.5,
        }
    }
}

/// A middleware that retries failed requests.
#[derive(Debug, Clone, Default)]
pub struct RetryMiddleware {
    config: RetryConfig,
}

impl RetryMiddleware {
    /// Creates a new `RetryMiddleware` with the default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: RetryConfig::default(),
        }
    }

    /// Creates a new `RetryMiddleware` with the specified configuration.
    #[must_use]
    pub fn with_config(config: RetryConfig) -> Self {
        Self { config }
    }

    /// Determines if an error is retriable.
    fn is_retriable(error: &SDKError) -> bool {
        match error {
            SDKError::Network(_) => true,
            SDKError::Api(api_err) => {
                if let Some(code) = api_err.status_code {
                    // Retry on rate limit (429) or server errors (5xx)
                    code == 429 || (500..=599).contains(&code)
                } else {
                    let msg = api_err.message.to_lowercase();
                    msg.contains("rate limit") || msg.contains("50")
                }
            }
            _ => false,
        }
    }
}

impl Middleware for RetryMiddleware {
    fn execute<'a>(
        &'a self,
        request: ClientRequest,
        next: NextFn<'a>,
    ) -> BoxFuture<'a, Result<ClientResponse, SDKError>> {
        Box::pin(async move {
            let max_attempts = self.config.max_attempts.max(1);
            let mut attempt = 0u32;

            loop {
                attempt += 1;
                debug!(
                    attempt,
                    max_attempts, "Executing request via retry middleware"
                );

                let result = next(request.clone()).await;

                match &result {
                    Err(e) if attempt < max_attempts && Self::is_retriable(e) => {
                        let exponent = i32::try_from(attempt - 1).unwrap_or(i32::MAX);
                        let delay = self
                            .config
                            .delay
                            .mul_f32(self.config.backoff_multiplier.powi(exponent));
                        warn!(
                            attempt,
                            max_attempts,
                            error = %e,
                            delay_ms = delay.as_millis(),
                            "Request failed with retriable error; retrying after backoff"
                        );
                        tokio::time::sleep(delay).await;
                    }
                    _ => return result,
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversation::Conversation;
    use crate::errors::ApiError;
    use crate::message::{Message, Role};
    use crate::middleware::MiddlewarePipeline;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_retry_middleware_with_config() {
        let config = RetryConfig {
            max_attempts: 5,
            delay: Duration::from_secs(2),
            backoff_multiplier: 3.0,
        };
        let middleware = RetryMiddleware::with_config(config);

        assert_eq!(middleware.config.max_attempts, 5);
        assert_eq!(middleware.config.delay, Duration::from_secs(2));
        assert_eq!(middleware.config.backoff_multiplier, 3.0);
    }

    #[tokio::test]
    async fn test_retry_middleware() {
        let mut pipeline = MiddlewarePipeline::new();
        pipeline.add(RetryMiddleware::new());

        let request = ClientRequest::new(Conversation::new());

        let handler = |_req: ClientRequest| async move {
            Ok(ClientResponse::new(Message::new(
                Role::Assistant,
                "Success",
            )))
        };

        let res = pipeline.execute(request, handler).await.unwrap();
        assert_eq!(res.message.content(), "Success");
    }

    /// Proves that a retriable error actually causes the request to be retried: the handler
    /// fails with a retriable (HTTP 500) error twice, then succeeds on the third attempt.
    #[tokio::test]
    async fn test_retry_middleware_retries_on_retriable_error() {
        let mut pipeline = MiddlewarePipeline::new();
        pipeline.add(RetryMiddleware::with_config(RetryConfig {
            max_attempts: 3,
            delay: Duration::from_millis(1),
            backoff_multiplier: 1.0,
        }));

        let request = ClientRequest::new(Conversation::new());
        let calls = Arc::new(AtomicUsize::new(0));
        let calls_clone = Arc::clone(&calls);

        let handler = move |_req: ClientRequest| {
            let calls = Arc::clone(&calls_clone);
            async move {
                let attempt = calls.fetch_add(1, Ordering::SeqCst) + 1;
                if attempt < 3 {
                    Err(SDKError::Api(ApiError::with_status("server error", 500)))
                } else {
                    Ok(ClientResponse::new(Message::new(
                        Role::Assistant,
                        "Success",
                    )))
                }
            }
        };

        let res = pipeline.execute(request, handler).await.unwrap();

        assert_eq!(res.message.content(), "Success");
        let call_count = calls.load(Ordering::SeqCst);
        assert!(
            call_count > 1,
            "expected more than one attempt, got {call_count}"
        );
        assert_eq!(call_count, 3);
    }

    /// Proves that a non-retriable error (HTTP 400) is NOT retried: only a single attempt is
    /// made and the original error is propagated.
    #[tokio::test]
    async fn test_retry_middleware_does_not_retry_non_retriable_error() {
        let mut pipeline = MiddlewarePipeline::new();
        pipeline.add(RetryMiddleware::with_config(RetryConfig {
            max_attempts: 3,
            delay: Duration::from_millis(1),
            backoff_multiplier: 1.0,
        }));

        let request = ClientRequest::new(Conversation::new());
        let calls = Arc::new(AtomicUsize::new(0));
        let calls_clone = Arc::clone(&calls);

        let handler = move |_req: ClientRequest| {
            let calls = Arc::clone(&calls_clone);
            async move {
                calls.fetch_add(1, Ordering::SeqCst);
                Err::<ClientResponse, _>(SDKError::Api(ApiError::with_status("bad request", 400)))
            }
        };

        let res = pipeline.execute(request, handler).await;

        assert!(res.is_err());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    /// Proves that `SDKError::Network` errors are retried (the `is_retriable` match arm at
    /// line 57), not just `ApiError`s with a 5xx/429 status code.
    #[tokio::test]
    async fn test_retry_middleware_retries_network_error() {
        let mut pipeline = MiddlewarePipeline::new();
        pipeline.add(RetryMiddleware::with_config(RetryConfig {
            max_attempts: 3,
            delay: Duration::from_millis(1),
            backoff_multiplier: 1.0,
        }));

        let request = ClientRequest::new(Conversation::new());
        let calls = Arc::new(AtomicUsize::new(0));
        let calls_clone = Arc::clone(&calls);

        let handler = move |_req: ClientRequest| {
            let calls = Arc::clone(&calls_clone);
            async move {
                let attempt = calls.fetch_add(1, Ordering::SeqCst) + 1;
                if attempt < 2 {
                    Err(SDKError::Network(crate::errors::NetworkError::new(
                        "connection reset",
                    )))
                } else {
                    Ok(ClientResponse::new(Message::new(
                        Role::Assistant,
                        "Success",
                    )))
                }
            }
        };

        let res = pipeline.execute(request, handler).await.unwrap();

        assert_eq!(res.message.content(), "Success");
        assert_eq!(calls.load(Ordering::SeqCst), 2);
    }

    /// Proves the message-based fallback in `is_retriable` (used when an `ApiError` carries no
    /// status code): a "rate limit" message is retried, while an unrelated message is not.
    #[tokio::test]
    async fn test_retry_is_retriable_message_fallback() {
        assert!(RetryMiddleware::is_retriable(&SDKError::Api(
            ApiError::new("Rate limit exceeded, please slow down")
        )));
        assert!(!RetryMiddleware::is_retriable(&SDKError::Api(
            ApiError::new("resource not found")
        )));
    }

    /// Proves the exponential backoff delay actually grows across attempts when
    /// `backoff_multiplier > 1.0`, using a paused Tokio clock so the assertion is exact and
    /// deterministic rather than a wall-clock-timing-based approximation.
    #[tokio::test(start_paused = true)]
    async fn test_retry_middleware_backoff_grows_exponentially() {
        let mut pipeline = MiddlewarePipeline::new();
        pipeline.add(RetryMiddleware::with_config(RetryConfig {
            max_attempts: 3,
            delay: Duration::from_millis(10),
            backoff_multiplier: 2.0,
        }));

        let request = ClientRequest::new(Conversation::new());
        let timestamps = Arc::new(std::sync::Mutex::new(Vec::new()));
        let timestamps_clone = Arc::clone(&timestamps);
        let calls = Arc::new(AtomicUsize::new(0));
        let calls_clone = Arc::clone(&calls);

        let handler = move |_req: ClientRequest| {
            let calls = Arc::clone(&calls_clone);
            let timestamps = Arc::clone(&timestamps_clone);
            async move {
                timestamps.lock().unwrap().push(tokio::time::Instant::now());
                let attempt = calls.fetch_add(1, Ordering::SeqCst) + 1;
                if attempt < 3 {
                    Err(SDKError::Api(ApiError::with_status("server error", 500)))
                } else {
                    Ok(ClientResponse::new(Message::new(
                        Role::Assistant,
                        "Success",
                    )))
                }
            }
        };

        pipeline.execute(request, handler).await.unwrap();

        let ts = timestamps.lock().unwrap();
        assert_eq!(ts.len(), 3);
        let first_gap = ts[1] - ts[0];
        let second_gap = ts[2] - ts[1];
        assert_eq!(first_gap, Duration::from_millis(10));
        assert_eq!(second_gap, Duration::from_millis(20));
    }

    /// Proves `max_attempts: 0` is clamped up to 1 (via `.max(1)`), so a misconfigured retry
    /// policy still makes exactly one attempt rather than zero calls or a panic.
    #[tokio::test]
    async fn test_retry_middleware_zero_max_attempts_still_calls_once() {
        let mut pipeline = MiddlewarePipeline::new();
        pipeline.add(RetryMiddleware::with_config(RetryConfig {
            max_attempts: 0,
            delay: Duration::from_millis(1),
            backoff_multiplier: 1.0,
        }));

        let request = ClientRequest::new(Conversation::new());
        let calls = Arc::new(AtomicUsize::new(0));
        let calls_clone = Arc::clone(&calls);

        let handler = move |_req: ClientRequest| {
            let calls = Arc::clone(&calls_clone);
            async move {
                calls.fetch_add(1, Ordering::SeqCst);
                Err::<ClientResponse, _>(SDKError::Api(ApiError::with_status("server error", 500)))
            }
        };

        let res = pipeline.execute(request, handler).await;

        assert!(res.is_err());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
