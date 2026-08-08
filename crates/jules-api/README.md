# jules-api

> The real HTTP client, authentication, and `v1alpha` endpoint implementations for Jules-SDK.

[![Crates.io](https://img.shields.io/crates/v/jules-api)](https://crates.io/crates/jules-api)
[![Docs.rs](https://img.shields.io/docsrs/jules-api)](https://docs.rs/jules-api)
[![License](https://img.shields.io/crates/l/jules-api)](../../LICENSE)

> **This crate is an internal implementation detail of [`jules-sdk`](../jules-sdk).** Most users should depend on `jules-sdk` (with the `middleware` feature) instead. Depend on `jules-api` directly only if you need transport-level control `jules-sdk` doesn't expose.

> **Status:** Pre-Alpha. Read endpoints are live-verified; write endpoints are not. See [Endpoint verification status](#endpoint-verification-status) below and the workspace [PROJECT_STATE.md](../../PROJECT_STATE.md) for authoritative status.

## What's in here

`jules-api` implements [`jules-core`](../jules-core)'s abstractions against a real network transport:

| Module | Purpose |
| --- | --- |
| `client` | `JulesClient` / `JulesClientBuilder` — the real, network-capable client, plus the `v1alpha` endpoint methods |
| `auth` | `AuthType` — `Bearer`, `ApiKey`, `Custom`, or `None`; `AuthType::google_api_key` for the real API's `X-Goog-Api-Key` header |
| `http` | `Transport`, `HttpRequest`/`HttpResponse`, `Method`, `Endpoint` (URL builder); `reqwest_transport::ReqwestTransport` on non-`wasm32` targets |
| `retry` | `ExponentialBackoff`, `RetryPolicy` — used internally by the endpoint methods |
| `response` | Response deserialization and Jules API error-response mapping |
| `session`, `conversation` | Session/conversation-shaped request helpers |
| `timeouts` | Timeout configuration |
| `errors` | `jules-api`-specific error types |
| `wasm` *(feature `wasm`)* | Browser `fetch`-backed transport for `wasm32` targets |

## Installation

```toml
[dependencies]
jules-api = { version = "0.1", features = ["middleware"] }
```

`JulesClient` requires the `middleware` feature (it relies on `jules-core`'s middleware pipeline for retries) and is only available on non-`wasm32` targets (it uses `reqwest` for network I/O). On `wasm32`, enable the `wasm` feature instead to get a browser `fetch`-backed transport.

## Usage

```rust,no_run
use jules_api::auth::AuthType;
use jules_api::client::{JulesClient, JulesClientBuilder};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client: JulesClient = JulesClientBuilder::new()
        // Defaults to https://jules.googleapis.com — override only for local testing.
        .timeout(Duration::from_secs(30))
        .auth(AuthType::google_api_key(std::env::var("JULES_API_KEY")?))
        .build()?;

    let sessions = client.list_sessions(Some(20), None).await?;
    for session in sessions.items() {
        println!("{:?}: {:?}", session.name(), session.state());
    }

    let sources = client.list_sources(None, None).await?;
    println!("{} connected source(s)", sources.items().len());

    Ok(())
}
```

### Endpoints implemented on `JulesClient`

| Method | Maps to | Verification |
| --- | --- | --- |
| `list_sessions(page_size, page_token)` | `GET /v1alpha/sessions` | ✅ Verified against the live API |
| `get_session(name)` | `GET /v1alpha/{name}` | ✅ Verified against the live API |
| `list_sources(page_size, page_token)` | `GET /v1alpha/sources` | ✅ Verified against the live API |
| `list_activities(session_name, page_size, page_token)` | `GET /v1alpha/{session}/activities` | ✅ Verified against the live API |
| `create_session(params)` | `POST /v1alpha/sessions` | ⚠️ Unverified — best-effort request shape, only tested against a mock server |
| `send_message(session_name, message)` | `POST /v1alpha/{session}:sendMessage` | ⚠️ Unverified against the live API. Note: the session must be past `QUEUED` (e.g. `AWAITING_USER_FEEDBACK`) or the API returns 404 |
| `approve_plan(session_name)` | `POST /v1alpha/{session}:approvePlan` | ⚠️ Unverified against the live API |

All list methods return `jules_core::pagination::Page<T>`, which exposes `.items()` and `.next_page_token()`.

### Authentication

```rust
use jules_api::auth::AuthType;

// The real Jules v1alpha API — verified header.
let auth = AuthType::google_api_key("YOUR_API_KEY");

// Generic alternatives, for other deployments or test servers:
let _ = AuthType::Bearer("token".to_string());
let _ = AuthType::ApiKey { header: "x-api-key".to_string(), key: "key".to_string() };
let _ = AuthType::None;
```

`AuthType`'s `Debug` impl redacts credential values, so it's safe to include in logs.

### Pointing at a test server

`JulesClientBuilder::base_url` overrides the default `https://jules.googleapis.com`, which is how this crate's own test suite exercises the endpoint methods against a local mock server without touching the real API.

## Endpoint verification status

Read endpoints (`list_sessions`, `get_session`, `list_sources`, `list_activities`) were built directly from real response payloads captured against the live Jules API. Write endpoints (`create_session`, `send_message`, `approve_plan`) were **not** exercised against the live API — doing so would create or mutate a real session — so their request shapes are a best-effort reading of the API docs and are only verified against local mock servers. Treat them as unverified until confirmed against the real API.

## Feature Flags

| Flag | Default | Enables |
| --- | --- | --- |
| `streaming` | ✅ | The `streaming` module (Server-Sent Events buffering/parsing) |
| `tools` | ✅ | Tool-calling support (via `jules-core`) |
| `middleware` | — | `JulesClient` / `JulesClientBuilder` and the retry pipeline (non-`wasm32` only) |
| `telemetry` | — | Tracing/metrics instrumentation |
| `cli` | — | Support code shared with `jules-cli` |
| `wasm` | — | The `wasm` module (browser `fetch` transport) |
| `experimental` | — | Unstable, no-notice-required-to-break APIs |

## More

* [jules-sdk](../jules-sdk) — the facade crate most users should depend on instead
* [jules-core](../jules-core) — the transport-agnostic types this crate implements against
* [Root README](../../README.md) · [PROJECT_STATE.md](../../PROJECT_STATE.md) · [ARCHITECTURE.md](../../ARCHITECTURE.md)

## References

The `v1alpha` endpoint paths, request/response shapes, and the `X-Goog-Api-Key` auth header were derived from the [Jules API REST reference](https://developers.google.com/jules/api/reference/rest) and cross-checked against [google-labs-code/jules-sdk](https://github.com/google-labs-code/jules-sdk).

## License

Dual-licensed under [MIT](../../LICENSE-MIT) or [Apache-2.0](../../LICENSE-APACHE), at your option.
