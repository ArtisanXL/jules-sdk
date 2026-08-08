//! End-to-end integration test proving `JulesClient` genuinely wires together the real
//! `ReqwestTransport`, the retry middleware, and the JSON request/response mapping: it spins
//! up a real local HTTP server, has it fail once (HTTP 500) and then succeed, and asserts the
//! client both retries and correctly carries auth headers / JSON body.
//!
//! Only runs when the `middleware` feature is enabled (required by `JulesClient`) on
//! non-wasm targets.
#![cfg(all(feature = "middleware", not(target_arch = "wasm32")))]

use jules_api::auth::AuthType;
use jules_api::client::JulesClientBuilder;
use jules_core::client::ClientRequest;
use jules_core::conversation::Conversation;
use jules_core::message::{Message, Role};
use jules_core::traits::Client;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// A captured HTTP request received by the fake server.
struct CapturedRequest {
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

/// Reads a single HTTP/1.1 request (headers + body) off `socket`.
async fn read_request(socket: &mut TcpStream) -> CapturedRequest {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 1024];

    let header_end = loop {
        let n = socket.read(&mut chunk).await.expect("read failed");
        assert!(n > 0, "connection closed before headers were received");
        buf.extend_from_slice(&chunk[..n]);
        if let Some(pos) = find_subslice(&buf, b"\r\n\r\n") {
            break pos + 4;
        }
    };

    let header_text = String::from_utf8_lossy(&buf[..header_end]).to_string();
    let mut headers = Vec::new();
    for line in header_text.lines().skip(1) {
        if let Some((k, v)) = line.split_once(':') {
            headers.push((k.trim().to_string(), v.trim().to_string()));
        }
    }

    let content_length: usize = headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("content-length"))
        .and_then(|(_, v)| v.parse().ok())
        .unwrap_or(0);

    let mut body = buf[header_end..].to_vec();
    while body.len() < content_length {
        let n = socket.read(&mut chunk).await.expect("read failed");
        assert!(n > 0, "connection closed before body was fully received");
        body.extend_from_slice(&chunk[..n]);
    }
    body.truncate(content_length);

    CapturedRequest { headers, body }
}

#[tokio::test]
async fn test_client_end_to_end_retries_then_succeeds() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let captured: Arc<Mutex<Vec<CapturedRequest>>> = Arc::new(Mutex::new(Vec::new()));
    let captured_for_server = Arc::clone(&captured);

    tokio::spawn(async move {
        // Handle exactly two connections: the first attempt (fails with 500), and the retry
        // (succeeds with 200).
        for call in 1..=2 {
            let (mut socket, _) = listener.accept().await.unwrap();
            let request = read_request(&mut socket).await;
            captured_for_server.lock().unwrap().push(request);

            let response = if call == 1 {
                let body = b"{\"error\":{\"message\":\"boom\"}}".to_vec();
                let mut resp = format!(
                    "HTTP/1.1 500 Internal Server Error\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                )
                .into_bytes();
                resp.extend_from_slice(&body);
                resp
            } else {
                let body = b"{\"message\":{\"role\":\"assistant\",\"content\":\"ack\"}}".to_vec();
                let mut resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                )
                .into_bytes();
                resp.extend_from_slice(&body);
                resp
            };

            socket.write_all(&response).await.unwrap();
            socket.shutdown().await.unwrap();
        }
    });

    let client = JulesClientBuilder::new()
        .base_url(format!("http://{addr}"))
        .timeout(Duration::from_secs(5))
        .auth(AuthType::Bearer("secret-token".to_string()))
        .build()
        .unwrap();

    let mut conversation = Conversation::new();
    conversation.add_message(Message::new(Role::User, "Hello, server"));
    let request = ClientRequest::new(conversation);

    let response = tokio::time::timeout(Duration::from_secs(10), client.send_request(request))
        .await
        .expect("send_request did not complete within the bounded time")
        .expect("send_request should succeed after one retry");

    // (b) the successful response was correctly parsed into a `ClientResponse`.
    assert_eq!(*response.message.role(), Role::Assistant);
    assert_eq!(response.message.content(), "ack");

    // (c) the 500-then-200 sequence resulted in exactly two attempts, proving the retry
    // middleware actually retried the request rather than failing immediately.
    let captured = captured.lock().unwrap();
    assert_eq!(
        captured.len(),
        2,
        "expected exactly two attempts (one failure + one retry)"
    );

    // (a) every attempt the fake server received carried the Authorization header and the
    // correct JSON body.
    for req in captured.iter() {
        let auth_header = req
            .headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case("authorization"))
            .map(|(_, v)| v.as_str());
        assert_eq!(auth_header, Some("Bearer secret-token"));

        let content_type = req
            .headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case("content-type"))
            .map(|(_, v)| v.as_str());
        assert_eq!(content_type, Some("application/json"));

        let body_json: serde_json::Value = serde_json::from_slice(&req.body).unwrap();
        assert_eq!(body_json["messages"][0]["role"], "user");
        assert_eq!(body_json["messages"][0]["content"], "Hello, server");
    }
}
