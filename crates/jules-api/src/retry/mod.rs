//! Retry and Rate-Limiting handling.

use jules_core::errors::SDKError;

/// Represents a strategy for retrying failed requests.
pub trait RetryPolicy: Send + Sync {
    /// Determines whether a request should be retried based on the error.
    /// If it should be retried, returns the delay in milliseconds.
    fn should_retry(&self, attempt: u32, error: &SDKError) -> Option<u64>;
}

/// A simple backoff retry policy.
#[derive(Debug, Clone)]
pub struct ExponentialBackoff {
    /// Maximum number of retries.
    pub max_retries: u32,
    /// Base delay in milliseconds.
    pub base_delay_ms: u64,
    /// Maximum delay in milliseconds.
    pub max_delay_ms: u64,
}

impl Default for ExponentialBackoff {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 100,
            max_delay_ms: 10_000,
        }
    }
}

impl RetryPolicy for ExponentialBackoff {
    fn should_retry(&self, attempt: u32, error: &SDKError) -> Option<u64> {
        if attempt >= self.max_retries {
            return None;
        }

        let is_retryable = match error {
            SDKError::Api(api_err) => {
                // Retry on rate limits (429) and server errors (5xx)
                if let Some(status) = api_err.status_code {
                    status == 429 || (500..=599).contains(&status)
                } else {
                    false
                }
            }
            SDKError::Network(_) => true,
            _ => false,
        };

        if is_retryable {
            // Calculate backoff: base_delay * 2^attempt
            // Attempt is 0-indexed here
            let mut delay = self.base_delay_ms.saturating_mul(1 << attempt);
            if delay > self.max_delay_ms {
                delay = self.max_delay_ms;
            }
            Some(delay)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jules_core::errors::{ApiError, NetworkError};

    #[test]
    fn test_exponential_backoff_max_retries() {
        let policy = ExponentialBackoff {
            max_retries: 2,
            ..Default::default()
        };

        let error = SDKError::Network(NetworkError::new("Timeout"));

        // Attempt 0
        assert_eq!(policy.should_retry(0, &error), Some(100));
        // Attempt 1
        assert_eq!(policy.should_retry(1, &error), Some(200));
        // Attempt 2 (max retries reached)
        assert_eq!(policy.should_retry(2, &error), None);
    }

    #[test]
    fn test_exponential_backoff_rate_limit() {
        let policy = ExponentialBackoff::default();
        let error = SDKError::Api(ApiError::with_status("Rate Limit Exceeded", 429));

        assert_eq!(policy.should_retry(0, &error), Some(100));
    }

    #[test]
    fn test_exponential_backoff_server_error() {
        let policy = ExponentialBackoff::default();
        let error = SDKError::Api(ApiError::with_status("Internal Server Error", 500));

        assert_eq!(policy.should_retry(0, &error), Some(100));
    }

    #[test]
    fn test_exponential_backoff_bad_request() {
        let policy = ExponentialBackoff::default();
        let error = SDKError::Api(ApiError::with_status("Bad Request", 400));

        // Should not retry client errors (400)
        assert_eq!(policy.should_retry(0, &error), None);
    }
}
