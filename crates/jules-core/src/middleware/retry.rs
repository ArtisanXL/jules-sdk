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

/// The maximum delay between retry attempts, regardless of `RetryConfig::backoff_multiplier`.
const MAX_RETRY_DELAY: Duration = Duration::from_secs(30);

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
            let mut delay = self.config.delay;
            let mut attempt = 1u32;

            loop {
                debug!(
                    attempt,
                    max_attempts, "Executing request via retry middleware"
                );

                match next(request.clone()).await {
                    Ok(response) => return Ok(response),
                    Err(e) => {
                        if attempt >= max_attempts || !Self::is_retriable(&e) {
                            return Err(e);
                        }

                        warn!(
                            "Request failed with retriable error (attempt {}/{}): {}. Retrying in {:?}.",
                            attempt, max_attempts, e, delay
                        );

                        #[cfg(not(target_arch = "wasm32"))]
                        tokio::time::sleep(delay).await;
                        #[cfg(target_arch = "wasm32")]
                        {
                            // No portable async timer dependency is pulled in for wasm32 targets
                            // by this crate, so retries proceed immediately without a real delay.
                        }

                        delay = delay
                            .mul_f32(self.config.backoff_multiplier)
                            .min(MAX_RETRY_DELAY);
                        attempt += 1;
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversation::Conversation;
    use crate::errors::NetworkError;
    use crate::message::{Message, Role};
    use crate::middleware::MiddlewarePipeline;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    fn fast_config(max_attempts: u32) -> RetryConfig {
        RetryConfig {
            max_attempts,
            delay: Duration::from_millis(1),
            backoff_multiplier: 1.0,
        }
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

    #[tokio::test]
    async fn test_retry_middleware_succeeds_after_retriable_failures() {
        let mut pipeline = MiddlewarePipeline::new();
        pipeline.add(RetryMiddleware::with_config(fast_config(5)));

        let request = ClientRequest::new(Conversation::new());
        let calls = Arc::new(AtomicU32::new(0));
        let handler_calls = Arc::clone(&calls);

        let handler = move |_req: ClientRequest| {
            let calls = Arc::clone(&handler_calls);
            async move {
                let attempt = calls.fetch_add(1, Ordering::SeqCst) + 1;
                if attempt < 3 {
                    Err(SDKError::Network(NetworkError::new("connection reset")))
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
        assert_eq!(calls.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_middleware_exhausts_max_attempts() {
        let mut pipeline = MiddlewarePipeline::new();
        pipeline.add(RetryMiddleware::with_config(fast_config(3)));

        let request = ClientRequest::new(Conversation::new());
        let calls = Arc::new(AtomicU32::new(0));
        let handler_calls = Arc::clone(&calls);

        let handler = move |_req: ClientRequest| {
            let calls = Arc::clone(&handler_calls);
            async move {
                calls.fetch_add(1, Ordering::SeqCst);
                Err::<ClientResponse, _>(SDKError::Network(NetworkError::new("connection reset")))
            }
        };

        let err = pipeline.execute(request, handler).await.unwrap_err();
        assert!(matches!(err, SDKError::Network(_)));
        assert_eq!(calls.load(Ordering::SeqCst), 3);
    }
}
