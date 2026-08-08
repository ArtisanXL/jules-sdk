//! A [`Transport`] implementation backed by [`reqwest`], for non-wasm targets.

use super::{HttpRequest, HttpResponse, Method, Transport};
use jules_core::errors::{NetworkError, SDKError};
use std::future::Future;
use std::time::Duration;

/// A [`Transport`] implementation that performs real network requests using
/// [`reqwest`].
///
/// This is the only place in the SDK where a request timeout is actually
/// enforced.
#[derive(Debug, Clone)]
pub struct ReqwestTransport {
    client: reqwest::Client,
}

impl ReqwestTransport {
    /// Creates a new `ReqwestTransport` whose underlying HTTP client enforces
    /// the given request `timeout`.
    ///
    /// # Panics
    ///
    /// Panics if the underlying `reqwest::Client` fails to build (e.g. if the
    /// TLS backend cannot be initialized).
    #[must_use]
    pub fn new(timeout: Duration) -> Self {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .expect("failed to build reqwest client");
        Self { client }
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

fn map_reqwest_error(err: &reqwest::Error) -> SDKError {
    let message = if err.is_timeout() {
        format!("request timed out: {err}")
    } else {
        format!("request failed: {err}")
    };
    SDKError::Network(NetworkError::new(message))
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

            let response = builder.send().await.map_err(|e| map_reqwest_error(&e))?;

            let status = response.status().as_u16();
            let headers = response
                .headers()
                .iter()
                .map(|(name, value)| {
                    (
                        name.to_string(),
                        value.to_str().unwrap_or_default().to_string(),
                    )
                })
                .collect();
            let body = response
                .bytes()
                .await
                .map_err(|e| map_reqwest_error(&e))?
                .to_vec();

            Ok(HttpResponse::new(status, headers, body))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn test_send_real_http_response() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            // Drain the request so the client isn't blocked writing it.
            let mut buf = [0u8; 1024];
            let _ = socket.read(&mut buf).await;

            let body = b"hello world";
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            socket.write_all(response.as_bytes()).await.unwrap();
            socket.write_all(body).await.unwrap();
            socket.shutdown().await.unwrap();
        });

        let transport = ReqwestTransport::new(Duration::from_secs(5));
        let request = HttpRequest::new(Method::Get, format!("http://{addr}/test"));
        let response = transport.send(request).await.unwrap();

        assert_eq!(response.status, 200);
        assert_eq!(response.body, b"hello world");
        assert!(response
            .headers
            .iter()
            .any(|(k, v)| k.eq_ignore_ascii_case("content-type") && v == "text/plain"));
    }

    #[tokio::test]
    async fn test_send_enforces_timeout() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            // Accept the connection but never respond, holding it open.
            let (_socket, _) = listener.accept().await.unwrap();
            tokio::time::sleep(Duration::from_secs(10)).await;
        });

        let transport = ReqwestTransport::new(Duration::from_millis(100));
        let request = HttpRequest::new(Method::Get, format!("http://{addr}/slow"));

        let result = tokio::time::timeout(Duration::from_secs(2), transport.send(request))
            .await
            .expect("send() did not return within the bounded time");

        match result {
            Err(SDKError::Network(err)) => {
                assert!(
                    err.message.to_lowercase().contains("timed out")
                        || err.message.to_lowercase().contains("timeout"),
                    "expected timeout message, got: {}",
                    err.message
                );
            }
            other => panic!("expected SDKError::Network due to timeout, got: {other:?}"),
        }
    }
}
