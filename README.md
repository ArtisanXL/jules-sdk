# Jules-SDK

> Early-stage (pre-alpha), async-first and idiomatic Rust implementation of Google's Jules SDK.

[![CI](https://github.com/ArtisanXL/jules-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/ArtisanXL/jules-sdk/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/jules-sdk)](https://crates.io/crates/jules-sdk)
[![Docs.rs](https://img.shields.io/docsrs/jules-sdk)](https://docs.rs/jules-sdk)
[![License](https://img.shields.io/crates/l/jules-sdk)](LICENSE)
[![MSRV](https://img.shields.io/badge/MSRV-1.90%2B-blue)](#minimum-supported-rust-version)

---

> **Status:** This project is in Pre-Alpha. Core types, builders and abstractions exist, and `jules-api` has a real native HTTP transport (`ReqwestTransport`) and an end-to-end `JulesClient`. All `v1alpha` session/source/activity endpoints (`list_sessions`, `get_session`, `create_session`, `send_message`, `approve_plan`, `list_sources`, `list_activities`) are implemented and live-verified against `https://jules.googleapis.com` using an API key via the `X-Goog-Api-Key` header. The Jules product API surface itself (`v1alpha`) is otherwise still limited — no OAuth/service-account auth, no CLI. See [PROJECT_STATE.md](PROJECT_STATE.md) for the authoritative, up-to-date status of every crate.

## Overview

Jules-SDK provides a modern, type-safe and async-first Rust interface for building applications powered by Jules.

The project follows Rust ecosystem best practices and is designed around:

* Type safety
* Async-first APIs
* Builder pattern abstractions
* Error handling infrastructure
* Streaming support (planned)
* Tool calling support (planned)
* WASM compatibility (planned)
* Testing infrastructure
* Multi-platform support
* Excellent developer experience

Jules-SDK aims to remain ergonomic for beginners while being powerful enough for production workloads once it reaches a stable, functionally complete release.

---

## Features

### Current Goals

* Async-first APIs
* Builder pattern interfaces
* Sessions support
* Conversations support
* Streaming APIs
* Tool calling support
* Official CLI
* WASM support
* Cloudflare Workers compatibility
* Extensive documentation

### Design Principles

* Minimal public API surface
* Zero-cost abstractions whenever possible
* Feature flag driven architecture
* Production-ready defaults (target for v1.0.0, not the current state)
* Long-term maintainability
* Semantic versioning compliance

---

## Installation

Add Jules-SDK to your project:

```toml
[dependencies]
jules-sdk = "0.1"
```

Or using Cargo:

```bash
cargo add jules-sdk
```

---

## Minimum Supported Rust Version

```text
Rust 1.90+
```

Changes to the Minimum Supported Rust Version (MSRV) are considered breaking changes and will only occur in accordance with the project's versioning policy.

---

## Quick Start

### Creating a Client

```rust
use jules_sdk::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .api_key("YOUR_API_KEY")
        .build()?;

    Ok(())
}
```

### Sessions

```rust
use jules_sdk::Session;

let session = Session::builder()
    .name("My Session")
    .build()?;
```

### Conversations

```rust
use jules_sdk::Conversation;

let conversation = Conversation::builder()
    .build()?;
```

### Streaming

```rust
let mut stream = client.stream().await?;

while let Some(event) = stream.next().await {
    println!("{:?}", event);
}
```

> **Note:** The APIs shown above are intended examples and may evolve during the `0.x` development cycle.

---

## Workspace Architecture

```text
jules-sdk/
│
├── crates/
│
│   ├── jules-sdk/
│   │   ├── -> Public facade crate
│   │   │
│   │   └── examples/
│   │
│   ├── jules-core/
│   │   -> Core abstractions
│   │
│   ├── jules-api/
│   │   -> API implementation
│   │
│   ├── jules-macros/
│   │   -> Proc macros
│   │
│   └── jules-cli/
│       -> Official CLI
```

Users should only depend on:

```toml
[dependencies]
jules-sdk = "0.1"
```

Internal crates are considered implementation details and are not intended to be used directly.

---

## Feature Flags

Current planned feature flags include:

```toml
[features]

default = [
    "streaming",
    "tools"
]

streaming=[]

tools=[]

cli=[]

middleware=[]

telemetry=[]

wasm=[]

experimental=[]
```

Future feature flags may include:

```toml
github-tools=[]

filesystem-tools=[]

browser-tools=[]

cloudflare-workers=[]
```

---

## Supported Platforms

| Platform           | Status  |
| ------------------ | ------- |
| Linux              | Supported |
| Windows            | Supported |
| macOS              | Supported |
| WASM               | Planned |
| Cloudflare Workers | Planned |
| WASI               | Planned |
| Deno               | Planned |

---

## Project Status

Jules-SDK is currently under active development.

### Release Plan

| Version | Status                   |
| ------- | ------------------------ |
| v0.1.0  | Initial Release          |
| v0.2.0  | Sessions & Conversations |
| v0.3.0  | Streaming                |
| v0.4.0  | Tool Calling             |
| v0.5.0  | Official CLI             |
| v0.6.0  | Proc Macros              |
| v0.7.0  | Middleware               |
| v0.8.0  | WASM Support             |
| v0.9.0  | API Freeze Candidate     |
| v1.0.0  | Stable Release           |

For additional details, please see:

* [PROJECT_STATE.md](PROJECT_STATE.md)
* [ROADMAP.md](ROADMAP.md)
* [ARCHITECTURE.md](ARCHITECTURE.md)
* [VERSIONING.md](VERSIONING.md)

---

## Development

Clone the repository:

```bash
git clone https://github.com/your-organization/jules-sdk.git

cd jules-sdk
```

Run the test suite:

```bash
cargo nextest run
```

Run Clippy:

```bash
cargo clippy --workspace --all-features
```

Run formatting checks:

```bash
cargo fmt --all --check
```

Run dependency auditing:

```bash
cargo audit
```

Build the entire workspace:

```bash
cargo build --workspace --all-features
```

---

## Documentation

Project documentation includes:

* [README.md](README.md)
* [PROJECT_STATE.md](PROJECT_STATE.md)
* [ROADMAP.md](ROADMAP.md)
* [ARCHITECTURE.md](ARCHITECTURE.md)
* [CHANGELOG.md](CHANGELOG.md)
* [CONTRIBUTING.md](CONTRIBUTING.md)
* [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
* [SECURITY.md](SECURITY.md)
* [VERSIONING.md](VERSIONING.md)
* [RELEASE.md](RELEASE.md)
* [TESTING.md](TESTING.md)
* [FEATURES.md](FEATURES.md)
* [SUPPORT.md](SUPPORT.md)
* [MSRV.md](MSRV.md)
* [AGENTS.md](AGENTS.md)

Additional examples and guides will be provided inside:

```text
docs/

examples/
```

---

## Contributing

Contributions are welcome.

Before opening a pull request, please read:

* [PROJECT_STATE.md](PROJECT_STATE.md)
* [CONTRIBUTING.md](CONTRIBUTING.md)
* [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
* [SECURITY.md](SECURITY.md)

Bug reports, documentation improvements and feature proposals are highly appreciated.

---

## Security

If you discover a security vulnerability, please follow the guidelines provided in:

```text
SECURITY.md
```

Please do not disclose security vulnerabilities publicly before they have been reviewed and addressed.

---

## Versioning Policy

Jules-SDK follows Semantic Versioning.

Before `v1.0.0`, breaking changes may occur as the public APIs evolve.

After `v1.0.0`, breaking changes will only be introduced through major version releases.

For more information, see:

```text
VERSIONING.md
```

---

## License

Jules-SDK is dual licensed under:

* Apache License, Version 2.0
* MIT License

You may choose either license when using this project.

---

## Acknowledgements

Jules-SDK is inspired by the design philosophy of modern Rust libraries and aims to provide an idiomatic Rust experience for developers building applications powered by Jules.

This project's API surface and Rust type shapes were developed with reference to:

* [Jules API REST reference](https://developers.google.com/jules/api/reference/rest) — the authoritative source for `v1alpha` resource shapes and endpoints
* [google-labs-code/jules-sdk](https://github.com/google-labs-code/jules-sdk) — Google's own SDK, used as a reference for naming and behavior parity

---

> Jules-SDK follows the Rust philosophy of releasing early, iterating during the `0.x` lifecycle and committing to API stability only when the project reaches `v1.0.0`.
