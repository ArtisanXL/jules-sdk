# jules-core

> Core models, traits, builders, and error types for Jules-SDK — no network I/O.

[![Crates.io](https://img.shields.io/crates/v/jules-core)](https://crates.io/crates/jules-core)
[![Docs.rs](https://img.shields.io/docsrs/jules-core)](https://docs.rs/jules-core)
[![License](https://img.shields.io/crates/l/jules-core)](../../LICENSE)

> **This crate is an internal implementation detail of [`jules-sdk`](../jules-sdk).** Most users should depend on `jules-sdk` instead — it re-exports the stable parts of this crate's public API. Depend on `jules-core` directly only if you're implementing an alternative transport or need types this facade doesn't re-export yet.

> **Status:** Pre-Alpha. Types here are stable enough to build and test against, but the public shape may still change before `1.0`. See the workspace [PROJECT_STATE.md](../../PROJECT_STATE.md) for authoritative status.

## What's in here

`jules-core` defines the transport-agnostic building blocks that [`jules-api`](../jules-api) implements against:

| Module | Purpose |
| --- | --- |
| `config` | `Config` / `ConfigBuilder` — API key and timeout configuration |
| `session` | `Session` / `SessionBuilder`, `SourceContext`, `GithubRepoContext` — mirrors the real `v1alpha` `Session` resource |
| `source` | `Source` — the `v1alpha` `Source` resource (e.g. connected GitHub repos) |
| `activity` | `Activity` — the `v1alpha` `sessions.activities` resource |
| `conversation` | `Conversation` — an in-memory, ordered list of `Message`s |
| `message` | `Message`, `Role` — a single conversation turn |
| `client` | `ClientRequest` — the generic request wrapper passed to `Client::send_request` |
| `response` | `ClientResponse` — the generic response wrapper |
| `traits` | `Client` — the trait `jules-api`'s `JulesClient` implements |
| `errors` | `SDKError` and its variants (`Authentication`, `Api`, `Network`, `Streaming`, `Tool`, `Validation`) |
| `pagination` | `Page<T>` — items plus an optional `next_page_token` |
| `builder` | Internal builder helpers used by the higher-level model builders |
| `middleware` *(feature `middleware`)* | `MiddlewarePipeline`, `RetryMiddleware`, logging middleware |
| `tool` *(feature `tools`)* | `Tool` / `DynTool` traits, `ToolParameters` for tool-calling |
| `wasm` *(feature `wasm`)* | WebAssembly-specific glue |

None of these types perform network I/O — that's `jules-api`'s job. `jules-core` is safe to depend on from `wasm32` targets and from anything that just needs to build or parse Jules API data shapes.

## Installation

```toml
[dependencies]
jules-core = "0.1"
```

## Usage

### Building a `Session` from a real API response

`Session` deserializes directly from the real `v1alpha` API's `camelCase` JSON:

```rust
use jules_core::session::Session;

let json = r#"{
    "name": "sessions/11413719004378428992",
    "title": "Example session",
    "state": "AWAITING_USER_FEEDBACK",
    "sourceContext": {
        "source": "sources/github/example-owner/example-repo",
        "githubRepoContext": { "startingBranch": "main" }
    }
}"#;

let session: Session = serde_json::from_str(json)?;
assert_eq!(session.state(), Some("AWAITING_USER_FEEDBACK"));
# Ok::<(), serde_json::Error>(())
```

### Implementing the `Client` trait

Anything that can turn a `ClientRequest` into a `ClientResponse` can act as a Jules-SDK client — this is how `jules-api`'s `JulesClient` plugs in, and how you'd write a test double:

```rust
use jules_core::client::ClientRequest;
use jules_core::conversation::Conversation;
use jules_core::errors::SDKError;
use jules_core::message::{Message, Role};
use jules_core::response::ClientResponse;
use jules_core::traits::Client;

struct EchoClient;

impl Client for EchoClient {
    async fn send_request(&self, _request: ClientRequest) -> Result<ClientResponse, SDKError> {
        Ok(ClientResponse::new(Message::new(Role::Assistant, "ok")))
    }
}

# async fn run() -> Result<(), SDKError> {
let mut conversation = Conversation::new();
conversation.add_message(Message::new(Role::User, "Hello"));

let response = EchoClient.send_request(ClientRequest::new(conversation)).await?;
assert_eq!(response.message, Message::new(Role::Assistant, "ok"));
# Ok(())
# }
```

## Feature Flags

| Flag | Default | Enables |
| --- | --- | --- |
| `streaming` | ✅ | The `streaming` module |
| `tools` | ✅ | The `tool` module and `Message`'s `tool_calls`/`tool_call_id` fields |
| `middleware` | — | The `middleware` module (pulls in `tokio`'s `time` feature) |
| `telemetry` | — | Tracing/metrics instrumentation points |
| `cli` | — | Support code shared with `jules-cli` |
| `wasm` | — | The `wasm` module |
| `experimental` | — | Unstable, no-notice-required-to-break APIs |

## More

* [jules-sdk](../jules-sdk) — the facade crate most users should depend on instead
* [jules-api](../jules-api) — the real HTTP client built on top of these types
* [Root README](../../README.md) · [PROJECT_STATE.md](../../PROJECT_STATE.md) · [ARCHITECTURE.md](../../ARCHITECTURE.md)

## References

Type shapes (in particular `Session`, `SourceContext`, and `GithubRepoContext`) were modeled directly from the [Jules API REST reference](https://developers.google.com/jules/api/reference/rest) and cross-checked against [google-labs-code/jules-sdk](https://github.com/google-labs-code/jules-sdk).

## License

Dual-licensed under [MIT](../../LICENSE-MIT) or [Apache-2.0](../../LICENSE-APACHE), at your option.
