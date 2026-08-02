//! Builder module.

use crate::client::ClientRequest;
use crate::errors::{SDKError, ValidationError};
use crate::response::ClientResponse;
use crate::traits::Client;
use std::future::Future;
use std::time::Duration;

/// A builder for creating a client.
#[derive(Debug, Clone)]
pub struct ClientBuilder {
    base_url: Option<String>,
    timeout: Duration,
    auth_token: Option<String>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            base_url: None,
            timeout: Duration::from_secs(30),
            auth_token: None,
        }
    }
}

impl ClientBuilder {
    /// Creates a new `ClientBuilder`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the base URL.
    #[must_use]
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Sets the timeout.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the auth token.
    #[must_use]
    pub fn auth_token(mut self, auth_token: impl Into<String>) -> Self {
        self.auth_token = Some(auth_token.into());
        self
    }

    /// Validates and builds the configuration.
    ///
    /// # Errors
    /// Returns `SDKError::Validation` if required fields are missing.
    pub fn build(self) -> Result<BuiltClient, SDKError> {
        let base_url = self
            .base_url
            .ok_or_else(|| SDKError::Validation(ValidationError::new("base_url is required")))?;

        let auth_token = self
            .auth_token
            .ok_or_else(|| SDKError::Validation(ValidationError::new("auth_token is required")))?;

        Ok(BuiltClient {
            base_url,
            timeout: self.timeout,
            auth_token,
        })
    }
}

/// A concrete client built by `ClientBuilder`.
#[derive(Debug, Clone)]
pub struct BuiltClient {
    base_url: String,
    timeout: Duration,
    auth_token: String,
}

impl BuiltClient {
    /// Returns the base URL.
    #[must_use]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Returns the timeout.
    #[must_use]
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    /// Returns the auth token.
    #[must_use]
    pub fn auth_token(&self) -> &str {
        &self.auth_token
    }
}

impl Client for BuiltClient {
    #[allow(clippy::manual_async_fn)]
    fn send_request(
        &self,
        request: ClientRequest,
    ) -> impl Future<Output = Result<ClientResponse, SDKError>> + Send {
        async move {
            let _ = request;
            Err(SDKError::Validation(ValidationError::new(
                "Not implemented",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_builder_defaults() {
        let builder = ClientBuilder::new();
        assert_eq!(builder.timeout, Duration::from_secs(30));
        assert!(builder.base_url.is_none());
        assert!(builder.auth_token.is_none());
    }

    #[test]
    fn test_client_builder_validation_error_url() {
        let builder = ClientBuilder::new().auth_token("secret");
        let result = builder.build();
        assert!(result.is_err());
        if let Err(SDKError::Validation(e)) = result {
            assert_eq!(e.message, "base_url is required");
        } else {
            panic!("Expected validation error");
        }
    }

    #[test]
    fn test_client_builder_validation_error_auth() {
        let builder = ClientBuilder::new().base_url("https://api.example.com");
        let result = builder.build();
        assert!(result.is_err());
        if let Err(SDKError::Validation(e)) = result {
            assert_eq!(e.message, "auth_token is required");
        } else {
            panic!("Expected validation error");
        }
    }

    #[test]
    fn test_client_builder_success() {
        let builder = ClientBuilder::new()
            .base_url("https://api.example.com")
            .auth_token("secret")
            .timeout(Duration::from_secs(10));

        let client = builder.build().unwrap();
        assert_eq!(client.base_url, "https://api.example.com");
        assert_eq!(client.auth_token, "secret");
        assert_eq!(client.timeout, Duration::from_secs(10));
    }

    #[tokio::test]
    async fn test_built_client_send_request() {
        let builder = ClientBuilder::new()
            .base_url("https://api.example.com")
            .auth_token("secret");
        let client = builder.build().unwrap();
        let request = ClientRequest::default();
        let result = client.send_request(request).await;

        assert!(result.is_err());
        if let Err(SDKError::Validation(e)) = result {
            assert_eq!(e.message, "Not implemented");
        } else {
            panic!("Expected validation error with message 'Not implemented'");
        }
    }
}
/// Request builder module.
pub mod request;
pub use request::RequestBuilder;

/// Conversation builder module.
pub mod conversation;
pub use conversation::ConversationBuilder;

/// Message builder module.
pub mod message;
pub use message::MessageBuilder;
