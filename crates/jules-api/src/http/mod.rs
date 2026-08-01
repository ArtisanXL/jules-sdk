//! Http module defining transport layer abstractions.

pub mod endpoint;

use jules_core::errors::SDKError;
use std::future::Future;

/// Represents an HTTP method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Method {
    /// GET method
    #[default]
    Get,
    /// POST method
    Post,
    /// PUT method
    Put,
    /// DELETE method
    Delete,
    /// PATCH method
    Patch,
}

/// A generic HTTP request abstraction.
#[derive(Debug, Clone, Default)]
pub struct HttpRequest {
    /// The HTTP method.
    pub method: Method,
    /// The URL to send the request to.
    pub url: String,
    /// The HTTP headers.
    pub headers: Vec<(String, String)>,
    /// The request body, if any.
    pub body: Option<Vec<u8>>,
}

impl HttpRequest {
    /// Creates a new `HttpRequest` with the given method and URL.
    #[must_use]
    pub fn new(method: Method, url: impl Into<String>) -> Self {
        Self {
            method,
            url: url.into(),
            headers: Vec::new(),
            body: None,
        }
    }

    /// Adds a header to the request.
    #[must_use]
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    /// Sets the request body.
    #[must_use]
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }
}

/// A generic HTTP response abstraction.
#[derive(Debug, Clone)]
pub struct HttpResponse {
    /// The HTTP status code.
    pub status: u16,
    /// The HTTP headers.
    pub headers: Vec<(String, String)>,
    /// The response body.
    pub body: Vec<u8>,
}

impl HttpResponse {
    /// Creates a new `HttpResponse`.
    #[must_use]
    pub fn new(status: u16, headers: Vec<(String, String)>, body: Vec<u8>) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }
}

/// The transport layer abstraction for executing HTTP requests.
pub trait Transport: Send + Sync {
    /// Sends an HTTP request and returns the response asynchronously.
    fn send(
        &self,
        request: HttpRequest,
    ) -> impl Future<Output = Result<HttpResponse, SDKError>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockTransport {
        response: HttpResponse,
    }

    impl Transport for MockTransport {
        fn send(
            &self,
            _request: HttpRequest,
        ) -> impl Future<Output = Result<HttpResponse, SDKError>> + Send {
            let response = self.response.clone();
            async move { Ok(response) }
        }
    }

    #[tokio::test]
    async fn test_mock_transport() {
        let transport = MockTransport {
            response: HttpResponse::new(
                200,
                vec![("Content-Type".into(), "application/json".into())],
                b"{}".to_vec(),
            ),
        };
        let request = HttpRequest::new(Method::Get, "https://api.example.com");
        let response = transport.send(request).await.unwrap();
        assert_eq!(response.status, 200);
        assert_eq!(response.body, b"{}");
    }
}
