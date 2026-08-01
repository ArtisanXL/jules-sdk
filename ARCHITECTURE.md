# Jules-SDK Architecture

> Production-ready, async-first and idiomatic Rust implementation of Google's Jules SDK.
>
> Repository: `jules-sdk`
>
> Version: Draft v1 (For live crate status and current priorities, see [PROJECT_STATE.md](PROJECT_STATE.md))
>
> MSRV: Rust 1.90+

---

# Architecture Goals

Jules-SDK is designed around five fundamental principles:

* Type Safety
* Async-first APIs
* Extensibility
* Performance
* Developer Experience

The architecture must provide:

* Production-grade abstractions
* Long-term maintainability
* Minimal public API surface
* Comprehensive testing support
* Multi-platform compatibility
* WASM compatibility
* Zero-cost abstractions whenever possible

---

# High Level Architecture

```text
                           User Application
                                   │
                                   │
                            use jules_sdk::*
                                   │
                                   ▼
                             jules-sdk
                          (Facade Crate)
                                   │
                   ┌───────────────┼───────────────┐
                   │                               │
                   ▼                               ▼
              jules-core                       jules-api
                   │                               │
                   │                               │
                   ▼                               ▼
             Core Models                     HTTP Client
                Traits                       Authentication
               Builders                      Sessions
                Errors                       Conversations
            Configurations                   Streaming
                                               Responses
                                                   │
                                                   ▼
                                               Reqwest
                                                   │
                                                   ▼
                                                Jules API
                   │
                   │
                   ▼
              jules-macros
                   │
                   ▼
               Proc Macros
                   │
                   ▼
                User APIs
                   │
                   ▼
                jules-cli
                   │
                   ▼
              Official CLI
```

---

# Repository Structure

```text
jules-sdk/
│
├── Cargo.toml
├── README.md
├── ROADMAP.md
├── ARCHITECTURE.md
├── CHANGELOG.md
│
├── crates/
│
│   ├── jules-sdk/
│   │
│   ├── jules-core/
│   │
│   ├── jules-api/
│   │
│   ├── jules-macros/
│   │
│   └── jules-cli/
│
├── examples/
│
├── benches/
│
├── tests/
│
└── .github/
    └── workflows/
```

---

# Dependency Graph

```text
                     User Application
                              │
                              ▼
                        jules-sdk
                              │
                 ┌────────────┴────────────┐
                 │                         │
                 ▼                         ▼
             jules-core               jules-api
                 │                         │
                 └──────────┬──────────────┘
                            │
                            ▼
                       jules-macros
                            │
                            ▼
                         proc-macro
                            │
                            ▼
                         User APIs


                    jules-cli
                         │
                         ▼
                    jules-sdk
```

### Dependency Rules

```text
Allowed

jules-sdk
↓

jules-core

jules-api

jules-macros


-------------------------


Allowed

jules-api
↓

jules-core


-------------------------


Allowed

jules-cli
↓

jules-sdk


-------------------------


NOT ALLOWED

jules-core
↓

jules-api


-------------------------


NOT ALLOWED

jules-core
↓

jules-cli


-------------------------


NOT ALLOWED

Circular Dependencies
```

---

# Public API Philosophy

Users should never need to know about internal crates.

### Preferred

```rust
use jules_sdk::Client;
use jules_sdk::Session;
use jules_sdk::Conversation;
use jules_sdk::Tool;
```

### Not Preferred

```rust
use jules_api::*;
use jules_core::*;
```

The facade crate is responsible for exposing the entire public API.

---

# Crate Ownership & Responsibilities

## Ownership Boundaries

To maintain a clean facade-crate architecture, strictly adhere to the following ownership boundaries:

* **`jules-core`** owns all primitive abstractions. It defines the core types, builder structures, domain models (Messages, Conversations, Tools), configuration objects, and standard traits. It must not depend on any network layers or API specifics.
* **`jules-api`** owns the communication boundary. It implements the actual HTTP clients, request/response parsing, session management, streaming protocols, retry policies, and authentication handling.
* **`jules-sdk`** owns the public API surface. It is responsible for orchestrating the feature flags, providing ergonomic re-exports of `jules-core` and `jules-api`, and ensuring the external developer experience remains unified.
* **`jules-macros`** owns all code-generation and procedural macros, minimizing boilerplate for the end user.
* **`jules-cli`** owns the terminal-based user interface, orchestrating the SDK for command-line interactions.


## jules-sdk

### Ownership

* **Primary Maintainers:** Core SDK Team
* **Domain:** Public-facing API surface, feature orchestration, and user experience.

### Responsibilities

* Public APIs
* Re-exports
* Feature Flags
* User Experience Layer

### Example

```rust
pub use jules_core::*;

pub use jules_api::*;

pub use jules_macros::*;
```

Users should only depend on:

```toml
[dependencies]

jules-sdk = "0.1"
```

---

## jules-core

Responsible for all core abstractions.

### Ownership

* **Primary Maintainers:** Core SDK Team
* **Domain:** Foundation abstractions, data models, traits, and error definitions.

### Responsibilities

```text
Client

Session

Conversation

Message

Response

Tool

Metadata

Builders

Traits

Configurations

Errors

Pagination
```

### Folder Structure

```text
src/

client/

conversation/

message/

response/

session/

tool/

builder/

config/

errors/

traits/

pagination/

lib.rs
```

### Core Traits

```rust
ClientTrait

SessionTrait

ToolTrait

ConversationTrait

BuilderTrait
```

---

## jules-api

Responsible for communicating with the Jules API.

### Ownership

* **Primary Maintainers:** Integrations Team
* **Domain:** HTTP communication, authentication, streaming, and third-party interactions.

### Responsibilities

```text
HTTP Client

Authentication

Streaming

Sessions

Responses

Conversations

Retry Policies

Timeouts
```

### Folder Structure

```text
src/

auth/

client/

http/

session/

conversation/

response/

streaming/

retry/

timeouts/

errors/

lib.rs
```

### Supported Features

```text
Authentication

Retries

Timeouts

Streaming

Pagination

Compression

TLS Support
```

---

# Streaming Architecture

```text
            User Application
                    │
                    ▼
                stream()
                    │
                    ▼
                API Client
                    │
                    ▼
                Jules API
                    │
                    ▼
                 Responses
                    │
                    ▼
                Event Parser
                    │
                    ▼
              Async Stream API
                    │
                    ▼
                 User Events
```

### Supported Events

```text
Text Events

Response Events

Progress Events

Metadata Events

Completion Events

Cancellation Events

Error Events
```

---

## jules-macros

Responsible for ergonomic APIs.

### Ownership

* **Primary Maintainers:** Core SDK Team
* **Domain:** Ergonomic abstractions, code generation, and syntactic sugar.

### Responsibilities

```text
Derive Macros

Builder Macros

Validation Macros
```

### Possible Macros

```rust
#[derive(Tool)]

#[derive(Session)]

#[jules_tool]

#[jules_builder]
```

### Folder Structure

```text
src/

derive/

tool/

builder/

validation/

lib.rs
```

---

## jules-cli

Responsible for the official CLI.

### Ownership

* **Primary Maintainers:** Tooling Team
* **Domain:** Command-line tooling, diagnostics, and developer utilities.

### Commands

```text
jules auth

jules chat

jules config

jules doctor

jules tools

jules session

jules version
```

### Folder Structure

```text
src/

commands/

config/

diagnostics/

interactive/

utils/

main.rs
```

---

# Builder Pattern

Jules-SDK follows the builder pattern whenever possible.

### Example

```rust
let client = Client::builder()
    .api_key("...")
    .timeout(30)
    .build()?;
```

### Benefits

* Type Safety
* Better discoverability
* Extensibility
* Improved developer experience

---

# Error Handling Strategy

Errors should be strongly typed.

```text
SDKError

↓

AuthenticationError

↓

APIError

↓

NetworkError

↓

StreamingError

↓

ToolError

↓

ValidationError
```

### Goals

* Comprehensive error messages
* Source propagation
* Context preservation
* Minimal allocations

---

# Feature Flag Strategy

### Default

```toml
default = [
    "streaming",
    "tools"
]
```

### Optional Features

```toml
streaming=[]

tools=[]

cli=[]

middleware=[]

telemetry=[]

wasm=[]

experimental=[]
```

### Future Features

```toml
github-tools=[]

filesystem-tools=[]

browser-tools=[]

cloudflare-workers=[]

tracing=[]
```

---

# Testing Strategy

### Unit Tests

```text
jules-core

jules-api

jules-macros

jules-cli
```

### Integration Tests

```text
Authentication

Streaming

Sessions

Responses

Tool Calling
```

### Benchmarking

```text
Streaming

Builders

Serialization

HTTP Requests
```

### Tools

```text
cargo-nextest

criterion

wiremock

proptest

insta

cargo-audit

cargo-deny
```

---

# WASM Architecture

Future WASM support will be implemented through feature flags.

```text
            jules-sdk
                  │
                  ▼
              WASM API
                  │
                  ▼
             Fetch API
                  │
                  ▼
               Browser
                  │
                  ▼
         Cloudflare Workers
                  │
                  ▼
                 WASI
```

---

# Performance Principles

Jules-SDK prioritizes:

* Zero-cost abstractions
* Minimal allocations
* Async optimizations
* Efficient streaming
* Lightweight public APIs

Performance regressions must be benchmarked before release.

---

# Security Principles

The project must provide:

* Dependency auditing
* Security scanning
* Secret detection
* TLS support
* Comprehensive testing

### Required Tooling

```text
cargo-audit

cargo-deny

cargo-nextest

Miri

Clippy

Rustfmt
```

---

# Stability Policy

Before v1.0.0:

```text
Breaking Changes

↓

Allowed

↓

Public APIs may evolve.
```

After v1.0.0:

```text
Breaking Changes

↓

Not Allowed

↓

Semantic Versioning must be followed.
```

All breaking changes before v1.0.0 should be documented in the CHANGELOG.

---

# Future Architecture Plans

Potential future additions include:

```text
Middleware System

OpenTelemetry

Cloudflare Workers Support

Filesystem Tools

GitHub Tools

Browser Tools

Distributed Tracing

Plugin Ecosystem
```

New capabilities should preferably be implemented through feature flags before introducing additional crates.

---

# Architectural Decisions

The Jules-SDK project intentionally adopts a facade-crate architecture. Only `jules-sdk` is exposed as the primary public dependency, while all other crates remain implementation details. This approach minimizes the public API surface, simplifies dependency management and provides the flexibility required to evolve the internal architecture throughout the `0.x` development lifecycle.

The project favors adding modules and feature flags over introducing new crates. Additional crates should only be created when they provide clear architectural or maintenance benefits.

Before proposing architectural changes, contributors and AI coding agents should verify the repository priorities and blockers in [PROJECT_STATE.md](PROJECT_STATE.md).

> Jules-SDK follows the Rust philosophy of releasing early, iterating during the `0.x` lifecycle and committing to API stability only when the project reaches `v1.0.0`.
