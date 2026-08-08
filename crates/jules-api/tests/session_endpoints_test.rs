//! Integration tests for the real `v1alpha` session/source/activity endpoints on
//! `JulesClient`, against a local mock HTTP server. Response bodies are shaped like the real
//! `v1alpha` API (verified against the live API on 2026-08-08) but use obviously-fake
//! placeholder data.
#![cfg(all(feature = "middleware", not(target_arch = "wasm32")))]

use jules_api::auth::AuthType;
use jules_api::client::{CreateSessionParams, JulesClientBuilder};
use jules_core::session::SourceContext;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// A single captured HTTP request received by the fake server.
struct CapturedRequest {
    request_line: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

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
    let mut lines = header_text.lines();
    let request_line = lines.next().unwrap_or_default().to_string();
    let mut headers = Vec::new();
    for line in lines {
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

    CapturedRequest {
        request_line,
        headers,
        body,
    }
}

/// Starts a local server that accepts exactly one connection, captures the request, and
/// responds with `status`/`body`. Returns the bound address and a handle yielding the captured
/// request.
async fn serve_once(
    status: &'static str,
    body: &'static str,
) -> (
    std::net::SocketAddr,
    tokio::task::JoinHandle<CapturedRequest>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let handle = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let captured = read_request(&mut socket).await;

        let body_bytes = body.as_bytes();
        let response = format!(
            "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body_bytes.len()
        );
        socket.write_all(response.as_bytes()).await.unwrap();
        socket.write_all(body_bytes).await.unwrap();
        socket.shutdown().await.unwrap();

        captured
    });

    (addr, handle)
}

fn find_header<'a>(req: &'a CapturedRequest, name: &str) -> Option<&'a str> {
    req.headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(name))
        .map(|(_, v)| v.as_str())
}

/// Starts a local server that accepts one connection per entry in `responses`, in order,
/// captures each request, and responds accordingly. Returns the bound address and a handle
/// yielding all captured requests in the order they were received.
async fn serve_sequence(
    responses: Vec<(&'static str, &'static str)>,
) -> (
    std::net::SocketAddr,
    tokio::task::JoinHandle<Vec<CapturedRequest>>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let handle = tokio::spawn(async move {
        let mut captured = Vec::new();
        for (status, body) in responses {
            let (mut socket, _) = listener.accept().await.unwrap();
            captured.push(read_request(&mut socket).await);

            let body_bytes = body.as_bytes();
            let response = format!(
                "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body_bytes.len()
            );
            socket.write_all(response.as_bytes()).await.unwrap();
            socket.write_all(body_bytes).await.unwrap();
            socket.shutdown().await.unwrap();
        }
        captured
    });

    (addr, handle)
}

/// Proves `JulesClient`'s internal `send_with_retry` loop (backing every real v1alpha endpoint
/// method) actually retries on a 500 and succeeds once the server recovers, re-sending auth on
/// each attempt.
#[tokio::test]
async fn test_list_sessions_retries_on_500_then_succeeds() {
    let error_body = r#"{"error": {"message": "internal error", "code": 500}}"#;
    let success_body = r#"{"sessions": [{"name": "sessions/000", "id": "000"}]}"#;
    let (addr, handle) = serve_sequence(vec![
        ("500 Internal Server Error", error_body),
        ("200 OK", success_body),
    ])
    .await;

    let client = JulesClientBuilder::new()
        .base_url(format!("http://{addr}"))
        .auth(AuthType::google_api_key("test-key"))
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let page = tokio::time::timeout(Duration::from_secs(5), client.list_sessions(None, None))
        .await
        .expect("timed out")
        .expect("list_sessions should succeed after retrying");

    assert_eq!(page.items().len(), 1);

    let requests = handle.await.unwrap();
    assert_eq!(
        requests.len(),
        2,
        "expected exactly one retry (two requests total)"
    );
    for req in &requests {
        assert_eq!(find_header(req, "X-Goog-Api-Key"), Some("test-key"));
    }
}

/// Proves `send_with_retry` gives up and propagates the final error after exhausting
/// `ExponentialBackoff::default().max_retries`, rather than retrying forever or panicking.
#[tokio::test]
async fn test_get_session_gives_up_after_max_retries() {
    let error_body = r#"{"error": {"message": "internal error", "code": 500}}"#;
    // ExponentialBackoff::default() has max_retries: 3, so 1 initial attempt + 3 retries = 4
    // total requests before send_with_retry gives up.
    let responses = vec![("500 Internal Server Error", error_body); 4];
    let (addr, handle) = serve_sequence(responses).await;

    let client = JulesClientBuilder::new()
        .base_url(format!("http://{addr}"))
        .auth(AuthType::google_api_key("test-key"))
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let result = tokio::time::timeout(Duration::from_secs(5), client.get_session("sessions/000"))
        .await
        .expect("timed out");

    assert!(
        result.is_err(),
        "expected an error after exhausting retries"
    );

    let requests = handle.await.unwrap();
    assert_eq!(
        requests.len(),
        4,
        "expected exactly 4 requests (1 initial + 3 retries)"
    );
}

#[tokio::test]
async fn test_list_sessions() {
    let body = r#"{
        "sessions": [
            {
                "name": "sessions/000",
                "title": "Example session",
                "state": "AWAITING_USER_FEEDBACK",
                "id": "000"
            }
        ],
        "nextPageToken": "token-2"
    }"#;
    let (addr, handle) = serve_once("200 OK", body).await;

    let client = JulesClientBuilder::new()
        .base_url(format!("http://{addr}"))
        .auth(AuthType::google_api_key("test-key"))
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let page = tokio::time::timeout(
        Duration::from_secs(5),
        client.list_sessions(Some(10), Some("page-token")),
    )
    .await
    .expect("timed out")
    .expect("list_sessions failed");

    assert_eq!(page.items().len(), 1);
    assert_eq!(page.items()[0].name(), Some("sessions/000"));
    assert_eq!(page.next_page_token(), Some("token-2"));

    let req = handle.await.unwrap();
    assert!(req.request_line.starts_with("GET /v1alpha/sessions?"));
    assert!(req.request_line.contains("pageSize=10"));
    assert!(req.request_line.contains("pageToken=page-token"));
    assert_eq!(find_header(&req, "X-Goog-Api-Key"), Some("test-key"));
}

#[tokio::test]
async fn test_get_session() {
    let body = r#"{"name": "sessions/000", "state": "COMPLETED", "id": "000"}"#;
    let (addr, handle) = serve_once("200 OK", body).await;

    let client = JulesClientBuilder::new()
        .base_url(format!("http://{addr}"))
        .auth(AuthType::google_api_key("test-key"))
        .build()
        .unwrap();

    let session = client.get_session("sessions/000").await.unwrap();
    assert_eq!(session.state(), Some("COMPLETED"));

    let req = handle.await.unwrap();
    assert!(req.request_line.starts_with("GET /v1alpha/sessions/000"));
}

#[tokio::test]
async fn test_list_sources() {
    let body = r#"{
        "sources": [
            {
                "name": "sources/github/example-owner/example-repo",
                "githubRepo": {
                    "owner": "example-owner",
                    "repo": "example-repo",
                    "isPrivate": false
                },
                "id": "github/example-owner/example-repo"
            }
        ]
    }"#;
    let (addr, handle) = serve_once("200 OK", body).await;

    let client = JulesClientBuilder::new()
        .base_url(format!("http://{addr}"))
        .auth(AuthType::google_api_key("test-key"))
        .build()
        .unwrap();

    let page = client.list_sources(None, None).await.unwrap();
    assert_eq!(page.items().len(), 1);
    assert_eq!(
        page.items()[0].github_repo().and_then(|r| r.owner()),
        Some("example-owner")
    );
    assert_eq!(page.next_page_token(), None);

    let req = handle.await.unwrap();
    assert!(req.request_line.starts_with("GET /v1alpha/sources"));
}

#[tokio::test]
async fn test_list_activities() {
    let body = r#"{
        "activities": [
            {
                "name": "sessions/000/activities/001",
                "originator": "AGENT",
                "planGenerated": {
                    "plan": {
                        "id": "plan-1",
                        "steps": [{"id": "s1", "title": "Step one"}]
                    }
                },
                "id": "001"
            }
        ]
    }"#;
    let (addr, handle) = serve_once("200 OK", body).await;

    let client = JulesClientBuilder::new()
        .base_url(format!("http://{addr}"))
        .auth(AuthType::google_api_key("test-key"))
        .build()
        .unwrap();

    let page = client
        .list_activities("sessions/000", None, None)
        .await
        .unwrap();
    assert_eq!(page.items().len(), 1);
    assert_eq!(page.items()[0].originator(), Some("AGENT"));

    let req = handle.await.unwrap();
    assert!(req
        .request_line
        .starts_with("GET /v1alpha/sessions/000/activities"));
}

#[tokio::test]
async fn test_create_session() {
    let body = r#"{"name": "sessions/new", "title": "New session", "id": "new"}"#;
    let (addr, handle) = serve_once("200 OK", body).await;

    let client = JulesClientBuilder::new()
        .base_url(format!("http://{addr}"))
        .auth(AuthType::google_api_key("test-key"))
        .build()
        .unwrap();

    let session = client
        .create_session(CreateSessionParams {
            title: Some("New session".to_string()),
            prompt: Some("Do the thing".to_string()),
            source_context: Some(SourceContext::new(
                "sources/github/example-owner/example-repo",
            )),
        })
        .await
        .unwrap();

    assert_eq!(session.name(), Some("sessions/new"));

    let req = handle.await.unwrap();
    assert!(req.request_line.starts_with("POST /v1alpha/sessions"));
    let sent: serde_json::Value = serde_json::from_slice(&req.body).unwrap();
    assert_eq!(sent["title"], "New session");
    assert_eq!(sent["prompt"], "Do the thing");
    assert_eq!(
        sent["sourceContext"]["source"],
        "sources/github/example-owner/example-repo"
    );
}

#[tokio::test]
async fn test_send_message() {
    let (addr, handle) = serve_once("200 OK", "{}").await;

    let client = JulesClientBuilder::new()
        .base_url(format!("http://{addr}"))
        .auth(AuthType::google_api_key("test-key"))
        .build()
        .unwrap();

    // Note: In the live API, this endpoint may return HTTP 404 if the session
    // is in the QUEUED state. It must be in a ready state (e.g., AWAITING_USER_FEEDBACK).
    client
        .send_message("sessions/000", "hello agent")
        .await
        .unwrap();

    let req = handle.await.unwrap();
    assert!(req
        .request_line
        .starts_with("POST /v1alpha/sessions/000:sendMessage"));
    let sent: serde_json::Value = serde_json::from_slice(&req.body).unwrap();
    assert_eq!(sent["prompt"], "hello agent");
}

#[tokio::test]
async fn test_approve_plan() {
    let (addr, handle) = serve_once("200 OK", "{}").await;

    let client = JulesClientBuilder::new()
        .base_url(format!("http://{addr}"))
        .auth(AuthType::google_api_key("test-key"))
        .build()
        .unwrap();

    client.approve_plan("sessions/000").await.unwrap();

    let req = handle.await.unwrap();
    assert!(req
        .request_line
        .starts_with("POST /v1alpha/sessions/000:approvePlan"));
    assert_eq!(std::str::from_utf8(&req.body).unwrap(), "{}");
}
