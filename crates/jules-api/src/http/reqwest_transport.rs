//! Native HTTP transport implementation backed by [`reqwest`].
//!
//! This module is only available on non-`wasm32` targets with the `cli`
//! feature enabled. [`ReqwestTransport`] implements [`Transport`], so it can
//! be used directly with [`crate::client::JulesClient`]:
//!
//! ```ignore
//! use jules_api::client::JulesClient;
//! use jules_api::auth::AuthType;
//! use jules_api::http::reqwest_transport::ReqwestTransport;
//!
//! let transport = ReqwestTransport::new();
//! let client = JulesClient::new(transport, AuthType::jules_api_key("my-key"));
//! ```

use crate::http::{HttpRequest, HttpResponse, Method, Transport};
use jules_core::errors::{NetworkError, SDKError};
use std::future::Future;
use std::time::Duration;

/// A [`Transport`] implementation backed by [`reqwest::Client`], for native
/// (non-`wasm32`) targets.
#[derive(Debug, Clone)]
pub struct ReqwestTransport {
    client: reqwest::Client,
}

impl ReqwestTransport {
    /// Creates a new `ReqwestTransport` with the default `reqwest` client
    /// configuration (no timeout).
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Creates a new `ReqwestTransport` that applies the given timeout to
    /// every request it sends.
    ///
    /// # Panics
    ///
    /// Panics if the underlying `reqwest::Client` fails to build (e.g. TLS
    /// backend initialization failure).
    #[must_use]
    pub fn with_timeout(timeout: Duration) -> Self {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .expect("failed to build reqwest client");
        Self { client }
    }
}

impl Default for ReqwestTransport {
    fn default() -> Self {
        Self::new()
    }
}

fn method_to_reqwest(method: Method) -> reqwest::Method {
    match method {
        Method::Get => reqwest::Method::GET,
        Method::Post => reqwest::Method::POST,
        Method::Put => reqwest::Method::PUT,
        Method::Delete => reqwest::Method::DELETE,
        Method::Patch => reqwest::Method::PATCH,
    }
}

fn network_error(context: &str, err: &reqwest::Error) -> SDKError {
    SDKError::from(NetworkError::new(format!("{context}: {err}")))
}

impl Transport for ReqwestTransport {
    fn send(
        &self,
        request: HttpRequest,
    ) -> impl Future<Output = Result<HttpResponse, SDKError>> + Send {
        let client = self.client.clone();
        async move {
            let mut builder = client.request(method_to_reqwest(request.method), &request.url);
            for (key, value) in &request.headers {
                builder = builder.header(key, value);
            }
            if let Some(body) = request.body {
                builder = builder.body(body);
            }

            let response = builder
                .send()
                .await
                .map_err(|e| network_error("request failed", &e))?;

            let status = response.status().as_u16();
            let headers = response
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or_default().to_string()))
                .collect();
            let body = response
                .bytes()
                .await
                .map_err(|e| network_error("failed to read response body", &e))?
                .to_vec();

            Ok(HttpResponse::new(status, headers, body))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::sync::mpsc;
    use std::time::Duration;

    /// Spawns a single-connection mock HTTP server on a local ephemeral
    /// port. It accepts one connection, sends back `response_bytes`
    /// verbatim, and reports the raw bytes it received on `rx`.
    fn spawn_mock_server(response_bytes: Vec<u8>) -> (String, mpsc::Receiver<Vec<u8>>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, rx) = mpsc::channel();

        std::thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = vec![0u8; 8192];
                let n = stream.read(&mut buf).unwrap_or(0);
                let _ = tx.send(buf[..n].to_vec());
                let _ = stream.write_all(&response_bytes);
                let _ = stream.flush();
            }
        });

        (format!("http://{addr}"), rx)
    }

    #[tokio::test]
    async fn test_get_request_success() {
        let (url, _rx) = spawn_mock_server(
            b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 2\r\n\r\nok".to_vec(),
        );

        let transport = ReqwestTransport::new();
        let request = HttpRequest::new(Method::Get, url);
        let response = transport.send(request).await.unwrap();

        assert_eq!(response.status, 200);
        assert_eq!(response.body, b"ok");
        assert!(response
            .headers
            .iter()
            .any(|(k, v)| k.eq_ignore_ascii_case("content-type") && v == "text/plain"));
    }

    #[tokio::test]
    async fn test_post_request_sends_headers_and_body() {
        let (url, rx) =
            spawn_mock_server(b"HTTP/1.1 201 Created\r\nContent-Length: 0\r\n\r\n".to_vec());

        let transport = ReqwestTransport::new();
        let request = HttpRequest::new(Method::Post, url)
            .with_header("X-Test-Header", "test-value")
            .with_body(b"payload".to_vec());
        let response = transport.send(request).await.unwrap();

        assert_eq!(response.status, 201);

        let received = rx.recv_timeout(Duration::from_secs(5)).unwrap();
        let received_str = String::from_utf8_lossy(&received);
        assert!(received_str.starts_with("POST "));
        assert!(received_str.contains("x-test-header: test-value"));
        assert!(received_str.contains("payload"));
    }

    #[tokio::test]
    async fn test_connection_error_maps_to_network_error() {
        // Bind then immediately drop a listener to obtain a port that is
        // very likely to refuse connections.
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        drop(listener);

        let transport = ReqwestTransport::new();
        let request = HttpRequest::new(Method::Get, format!("http://{addr}"));
        let result = transport.send(request).await;

        assert!(matches!(result, Err(SDKError::Network(_))));
    }

    #[tokio::test]
    async fn test_timeout_maps_to_network_error() {
        // Server accepts the connection but never writes a response.
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        std::thread::spawn(move || {
            let _ = listener.accept();
            std::thread::sleep(Duration::from_secs(5));
        });

        let transport = ReqwestTransport::with_timeout(Duration::from_millis(100));
        let request = HttpRequest::new(Method::Get, format!("http://{addr}"));
        let result = transport.send(request).await;

        assert!(matches!(result, Err(SDKError::Network(_))));
    }
}
