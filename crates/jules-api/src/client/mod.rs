//! Client module providing [`JulesClient`], a real, network-capable client.
//!
//! This module is only available when the `middleware` feature is enabled (it relies on
//! `jules_core`'s middleware pipeline for retry handling) and on non-wasm targets (it relies
//! on [`crate::http::reqwest_transport::ReqwestTransport`] for real network I/O).
#![cfg(all(feature = "middleware", not(target_arch = "wasm32")))]

use crate::auth::AuthType;
use crate::http::reqwest_transport::ReqwestTransport;
use crate::http::{HttpRequest, Method, Transport};
use crate::response::deserialize_response;
use jules_core::client::ClientRequest;
use jules_core::errors::{SDKError, ValidationError};
use jules_core::message::Message;
use jules_core::middleware::retry::RetryMiddleware;
use jules_core::middleware::MiddlewarePipeline;
use jules_core::response::ClientResponse;
use jules_core::traits::Client;
use serde::Serialize;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

mod endpoints;
pub use endpoints::CreateSessionParams;

/// The default base URL for the real Jules `v1alpha` API.
pub const JULES_API_BASE_URL: &str = "https://jules.googleapis.com";

/// A real, network-capable client for making JSON HTTP requests.
///
/// `JulesClient` wires together a real [`Transport`] (network I/O via `reqwest`), an
/// [`AuthType`] applied to every outgoing request, and a `jules_core` middleware pipeline
/// (including retry-with-backoff) that wraps the underlying transport call.
///
/// The real `v1alpha` Jules API endpoints (sessions, sources, activities) are implemented in
/// this crate's internal `endpoints` module and live-verified for reads (see that module's
/// docs for exactly what is and isn't confirmed against the live API). The [`Client`] trait
/// impl below (`send_request`) is a separate, generic conversation-shaped JSON request/response
/// path that works against any JSON HTTP API, including a local test server, via a configurable
/// `base_url` — prefer the `v1alpha`-specific methods for talking to the real Jules API.
#[derive(Clone)]
pub struct JulesClient {
    transport: ReqwestTransport,
    base_url: String,
    auth: AuthType,
    pipeline: Arc<MiddlewarePipeline>,
}

impl JulesClient {
    /// Returns the configured base URL requests are sent to.
    #[must_use]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

/// The JSON body sent to the configured `base_url`.
///
/// This is a generic, minimal wire shape (not a verified Jules API schema).
#[derive(Serialize)]
struct RequestBody<'a> {
    messages: &'a [Message],
}

/// Performs a single HTTP round-trip: serialize, apply auth, send, and deserialize.
async fn send_once(
    transport: ReqwestTransport,
    base_url: String,
    auth: AuthType,
    request: ClientRequest,
) -> Result<ClientResponse, SDKError> {
    let body = serde_json::to_vec(&RequestBody {
        messages: request.conversation.messages(),
    })
    .map_err(|e| {
        SDKError::Validation(ValidationError::new(format!(
            "failed to serialize request: {e}"
        )))
    })?;

    let http_request = HttpRequest::new(Method::Post, base_url)
        .with_header("Content-Type", "application/json")
        .with_body(body);
    let http_request = auth.apply(http_request);

    let http_response = transport.send(http_request).await?;
    deserialize_response(&http_response)
}

impl Client for JulesClient {
    fn send_request(
        &self,
        request: ClientRequest,
    ) -> impl Future<Output = Result<ClientResponse, SDKError>> + Send {
        let transport = self.transport.clone();
        let base_url = self.base_url.clone();
        let auth = self.auth.clone();
        let pipeline = Arc::clone(&self.pipeline);
        async move {
            pipeline
                .execute(request, move |req| {
                    let transport = transport.clone();
                    let base_url = base_url.clone();
                    let auth = auth.clone();
                    async move { send_once(transport, base_url, auth, req).await }
                })
                .await
        }
    }
}

/// A builder for [`JulesClient`].
///
/// Defaults to [`JULES_API_BASE_URL`] (the real Jules API) unless overridden — override it to
/// point at a local test server.
#[derive(Debug, Clone)]
pub struct JulesClientBuilder {
    base_url: String,
    timeout: Duration,
    auth: AuthType,
}

impl Default for JulesClientBuilder {
    fn default() -> Self {
        Self {
            base_url: JULES_API_BASE_URL.to_string(),
            timeout: Duration::from_secs(30),
            auth: AuthType::None,
        }
    }
}

impl JulesClientBuilder {
    /// Creates a new `JulesClientBuilder`, defaulting to [`JULES_API_BASE_URL`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Overrides the base URL requests are sent to (defaults to [`JULES_API_BASE_URL`]).
    #[must_use]
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Sets the request timeout enforced by the underlying transport.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the authentication applied to every outgoing request.
    #[must_use]
    pub fn auth(mut self, auth: AuthType) -> Self {
        self.auth = auth;
        self
    }

    /// Builds a [`JulesClient`] from the configured settings.
    ///
    /// # Errors
    ///
    /// Currently infallible in practice (kept as `Result` for forward-compatible validation),
    /// matching the fallible-builder convention used elsewhere in this codebase.
    pub fn build(self) -> Result<JulesClient, SDKError> {
        let transport = ReqwestTransport::new(self.timeout);

        let mut pipeline = MiddlewarePipeline::new();
        pipeline.add(RetryMiddleware::new());

        Ok(JulesClient {
            transport,
            base_url: self.base_url,
            auth: self.auth,
            pipeline: Arc::new(pipeline),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_defaults_to_jules_api_base_url() {
        let client = JulesClientBuilder::new().build().unwrap();
        assert_eq!(client.base_url(), JULES_API_BASE_URL);
    }

    #[test]
    fn test_builder_success() {
        let client = JulesClientBuilder::new()
            .base_url("https://api.example.com")
            .timeout(Duration::from_secs(5))
            .auth(AuthType::Bearer("token".into()))
            .build()
            .unwrap();

        assert_eq!(client.base_url(), "https://api.example.com");
    }
}
