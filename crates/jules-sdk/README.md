# jules-sdk

> The public facade crate for Jules-SDK — an async-first, idiomatic Rust client for Google's Jules API.

[![Crates.io](https://img.shields.io/crates/v/jules-sdk)](https://crates.io/crates/jules-sdk)
[![Docs.rs](https://img.shields.io/docsrs/jules-sdk)](https://docs.rs/jules-sdk)
[![License](https://img.shields.io/crates/l/jules-sdk)](../../LICENSE)

This is the crate you should depend on. It re-exports the stable surface of [`jules-core`](../jules-core) and [`jules-api`](../jules-api), plus the procedural macros from [`jules-macros`](../jules-macros), behind a single dependency.

> **Status:** Pre-Alpha. The real `v1alpha` read endpoints (`list_sessions`, `get_session`, `list_sources`, `list_activities`) are live-verified against `https://jules.googleapis.com`. The write endpoints (`create_session`, `send_message`, `approve_plan`) exist and work against a mock server but have not been exercised against the live API. Public API shapes may still change before `1.0`. See the workspace [PROJECT_STATE.md](../../PROJECT_STATE.md) for the authoritative, up-to-date status.

## Installation

```bash
cargo add jules-sdk
```

Or manually:

```toml
[dependencies]
jules-sdk = "0.1"
```

## What's exported

| Item | From | Notes |
| --- | --- | --- |
| `Config`, `ConfigBuilder` | `jules-core` | API key / timeout configuration |
| `Session`, `SessionBuilder` | `jules-core` | The `v1alpha` `Session` resource |
| `Conversation` | `jules-core` | In-memory message history |
| `Client` (trait) | `jules-core` | The generic request/response abstraction `JulesClient` implements |
| `JulesClient`, `JulesClientBuilder` | `jules-api` | The real, network-capable client. Requires the `middleware` feature and a non-`wasm32` target |
| `Tool` | `jules-core` | Tool-calling abstraction. Requires the `tools` feature |
| `Placeholder` (derive) | `jules-macros` | Not yet functional — see [jules-macros](../jules-macros) |

The `jules_core` and `jules_api` crates are also re-exported in full (`jules_sdk::jules_core`, `jules_sdk::jules_api`) so you can reach anything not re-exported at the top level without adding them as direct dependencies.

## Usage

### Talking to the real Jules API

This is the only end-to-end path that is live-verified today. It requires the `middleware` feature (enabled below via `features = ["middleware"]`):

```rust,no_run
use jules_sdk::jules_api::auth::AuthType;
use jules_sdk::{JulesClient, JulesClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client: JulesClient = JulesClientBuilder::new()
        .auth(AuthType::google_api_key(std::env::var("JULES_API_KEY")?))
        .build()?;

    let page = client.list_sessions(Some(10), None).await?;
    for session in page.items() {
        println!("{:?} — {:?}", session.name(), session.state());
    }

    Ok(())
}
```

```toml
[dependencies]
jules-sdk = { version = "0.1", features = ["middleware"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

### Building configuration and models directly

The `Config` and `Session` builders from `jules-core` are usable independently of any network client, e.g. for tests or for constructing request payloads:

```rust
use jules_sdk::Config;

let config = Config::builder()
    .api_key("YOUR_API_KEY")
    .timeout(30)
    .build()?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Examples

The [`examples/`](examples) directory has runnable, self-contained programs. All commands are run from the repo root.

| Example | Demonstrates | Run |
| --- | --- | --- |
| `basic_config` | Building a `Config` with an API key and timeout via `ConfigBuilder`. | `cargo run --example basic_config -p jules-sdk` |
| `basic_session` | Building a `Session` with a name via `SessionBuilder`. | `cargo run --example basic_session -p jules-sdk` |
| `cli_usage` | Loading a `Config` from the `JULES_API_KEY` env var and building an (unused) `Session`, mimicking how a CLI front-end would consume the SDK. | `cargo run --example cli_usage -p jules-sdk --features cli` |
| `middleware` | Building a `MiddlewarePipeline`, running a request through a custom `Middleware` that counts invocations and rewrites the response message, and driving the resulting future to completion by hand. | `cargo run --example middleware -p jules-sdk --features middleware` |
| `streaming` | Implementing the `Stream` trait to emit `StreamEvent::TextChunk`/`StreamEvent::Done` values and consuming them in a loop. | `cargo run --example streaming -p jules-sdk` |
| `tools` | Implementing the `Tool` trait, registering it in a `ToolRegistry`, and invoking it dynamically by name via `call_dyn`. | `cargo run --example tools -p jules-sdk` |
| `v1alpha_client` | Using the real `JulesClient` to call the live `list_sessions`/`list_sources` endpoints against `https://jules.googleapis.com` (requires a real `JULES_API_KEY`; prints instructions and exits if unset). | `cargo run --example v1alpha_client -p jules-sdk --features middleware` |

## Feature Flags

| Flag | Default | Enables |
| --- | --- | --- |
| `streaming` | ✅ | Streaming response abstractions in `jules-core`/`jules-api` |
| `tools` | ✅ | Tool-calling types (`Tool`) |
| `middleware` | — | The middleware pipeline and the real `JulesClient` (non-`wasm32` only) |
| `telemetry` | — | Tracing/metrics instrumentation |
| `cli` | — | CLI-oriented support code shared with `jules-cli` |
| `wasm` | — | WebAssembly-targeted code paths |
| `experimental` | — | Unstable, no-notice-required-to-break APIs |

Enable what you need explicitly, e.g. `features = ["middleware", "tools"]`. See each underlying crate's `Cargo.toml` for how these flags compose.

## Where this fits in the workspace

```text
jules-sdk    <- you depend on this
├── jules-core    (traits, models, builders — no network I/O)
├── jules-api     (HTTP transport, auth, the real v1alpha endpoints)
└── jules-macros  (proc macros; currently a no-op placeholder)
```

`jules-core`, `jules-api`, and `jules-macros` are implementation details of this facade and are not intended to be depended on directly — see their own READMEs if you need to understand internals.

## References

This SDK's design and its Rust type shapes were cross-checked against:

* [Jules API REST reference](https://developers.google.com/jules/api/reference/rest) — the authoritative source for `v1alpha` resource shapes and endpoints
* [google-labs-code/jules-sdk](https://github.com/google-labs-code/jules-sdk) — Google's own SDK, used as a reference for naming and behavior parity

## More

* [Root README](../../README.md) — full project overview, roadmap, and platform support
* [PROJECT_STATE.md](../../PROJECT_STATE.md) — authoritative crate-by-crate status
* [ARCHITECTURE.md](../../ARCHITECTURE.md) — workspace design

## License

Dual-licensed under [MIT](../../LICENSE-MIT) or [Apache-2.0](../../LICENSE-APACHE), at your option.
