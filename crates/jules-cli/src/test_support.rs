//! Test-only helper: a minimal local HTTP server for exercising CLI subcommand handlers
//! against a real `JulesClient` without hitting the live network. Mirrors the pattern used by
//! `jules-api`'s own `tests/client_e2e_test.rs`.
#![cfg(test)]

use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

/// A single HTTP request captured by [`MockServer`].
pub struct CapturedRequest {
    /// The request path (e.g. `/v1alpha/sessions`).
    pub path: String,
    /// The request body.
    pub body: Vec<u8>,
}

/// A minimal local HTTP server that accepts one connection, responds with a fixed
/// status/JSON body, and lets the test await + inspect the request it received.
pub struct MockServer {
    addr: std::net::SocketAddr,
    received: Arc<Mutex<Option<CapturedRequest>>>,
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

impl MockServer {
    /// Starts a server that accepts exactly one connection, responds with `status`/`body`,
    /// and records the request it received.
    pub async fn respond_once(status: u16, body: &str) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let received: Arc<Mutex<Option<CapturedRequest>>> = Arc::new(Mutex::new(None));
        let received_for_server = Arc::clone(&received);
        let body = body.to_string();

        tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buf = Vec::new();
            let mut chunk = [0u8; 4096];

            let header_end = loop {
                let n = socket.read(&mut chunk).await.unwrap();
                assert!(n > 0, "connection closed before headers were received");
                buf.extend_from_slice(&chunk[..n]);
                if let Some(pos) = find_subslice(&buf, b"\r\n\r\n") {
                    break pos + 4;
                }
            };

            let header_text = String::from_utf8_lossy(&buf[..header_end]).to_string();
            let mut lines = header_text.lines();
            let path = lines
                .next()
                .unwrap_or_default()
                .split_whitespace()
                .nth(1)
                .unwrap_or_default()
                .to_string();

            let headers: Vec<(String, String)> = lines
                .filter_map(|line| line.split_once(':'))
                .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
                .collect();

            let content_length: usize = headers
                .iter()
                .find(|(k, _)| k.eq_ignore_ascii_case("content-length"))
                .and_then(|(_, v)| v.parse().ok())
                .unwrap_or(0);

            let mut req_body = buf[header_end..].to_vec();
            while req_body.len() < content_length {
                let n = socket.read(&mut chunk).await.unwrap();
                assert!(n > 0, "connection closed before body was fully received");
                req_body.extend_from_slice(&chunk[..n]);
            }
            req_body.truncate(content_length);

            *received_for_server.lock().unwrap() = Some(CapturedRequest {
                path,
                body: req_body,
            });

            let mut response = format!(
                "HTTP/1.1 {status} status\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            )
            .into_bytes();
            response.extend_from_slice(body.as_bytes());
            socket.write_all(&response).await.unwrap();
            socket.shutdown().await.unwrap();
        });

        Self { addr, received }
    }

    /// The base URL of this server (e.g. `http://127.0.0.1:54321`).
    pub fn base_url(&self) -> String {
        format!("http://{}", self.addr)
    }

    /// Waits for and returns the request the server received.
    pub async fn received(&self) -> CapturedRequest {
        for _ in 0..200 {
            if let Some(req) = self.received.lock().unwrap().take() {
                return req;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        panic!("server did not receive a request within the timeout");
    }
}
