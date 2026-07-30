# Jules-SDK Roadmap

> Production-ready, async-first and idiomatic Rust implementation of Google's Jules SDK.
>
> Repository: `jules-sdk`
>
> License: Apache-2.0 OR MIT
>
> MSRV: Rust 1.90+
>
> Current Status: Planning (For the live project state and active tasks, see [PROJECT_STATE.md](PROJECT_STATE.md))

---

## Vision

Jules-SDK aims to provide an ergonomic, production-ready and extensible Rust SDK for building applications powered by Jules.

### Goals

* Async-first APIs
* Type-safe abstractions
* Production-ready SDK
* Streaming support
* Tool calling support
* WASM compatibility
* Multi-platform support
* Excellent developer experience
* Comprehensive documentation
* Zero-cost abstractions whenever possible

---

## Repository Structure

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
│   │   -> Public facade crate
│   │
│   ├── jules-core/
│   │   -> Core types and traits
│   │
│   ├── jules-api/
│   │   -> API client implementation
│   │
│   ├── jules-macros/
│   │   -> Proc macros
│   │
│   └── jules-cli/
│       -> Official CLI
│
├── examples/
│
└── .github/
    └── workflows/
```

---

## Public API Design

Users should only interact with:

```rust
use jules_sdk::Client;
use jules_sdk::Session;
use jules_sdk::Conversation;
use jules_sdk::Tool;
```

Internal crates are implementation details and may evolve independently without affecting the public API.

---

## Development Principles

* Async-first design
* Builder pattern APIs
* Feature flag driven architecture
* Comprehensive testing
* Excellent documentation
* Backwards compatibility after v1.0.0
* Minimal dependencies
* Production-grade error handling

---

## Feature Flags

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

Future features:

```toml
github-tools=[]

filesystem-tools=[]

browser-tools=[]

cloudflare-workers=[]

tracing=[]
```

---

# Release Strategy

## Semantic Versioning Policy

### Before v1.0.0

Breaking changes are allowed when necessary.

```text
v0.1.0
↓

Initial Release

↓

v0.2.0
↓

Breaking changes allowed

↓

v0.3.0
↓

Breaking changes allowed

↓

...

↓

v0.9.0
↓

API Freeze Candidate

↓

v1.0.0
↓

Stable API
```

### After v1.0.0

```text
1.0.0
↓

Stable Release

↓

1.1.0
↓

New Features

↓

1.2.0
↓

New Features

↓

1.2.1
↓

Bug Fixes
```

---

# v0.1.0

## Initial Release

### Goals

Provide a usable and publishable SDK.

### Included Crates

```text
jules-sdk

jules-core

jules-api
```

### Features

#### Core

* Client Builder
* Configuration System
* Core Traits
* Core Models
* Error Handling

#### API

* Authentication
* HTTP Client
* Basic Requests
* Timeouts
* Retry Support

#### Infrastructure

* Cargo Workspace
* CI/CD
* GitHub Actions
* Documentation
* Security Workflows

### Quality

* Unit Tests
* Clippy
* Rustfmt
* Cargo Audit
* Cargo Deny

### Deliverables

```text
READY FOR USE
```

---

# v0.2.0

## Sessions & Conversations

### Features

* Sessions
* Conversations
* Messages
* Metadata
* Pagination Support

### APIs

```rust
Client

Session

Conversation

Message

Response
```

### Tasks

* Session Management
* Conversation Management
* Builders
* Serialization Support

---

# v0.3.0

## Streaming APIs

### Features

* Text Streaming
* Event Streaming
* Response Streaming
* Cancellation Support

### Tasks

```rust
stream()

stream_text()

stream_events()

cancel()
```

### Improvements

* Async Optimizations
* Streaming Error Handling
* Progress Events

---

# v0.4.0

## Tool Calling

### Features

* Tool Registration
* Tool Execution
* Async Tools
* Tool Validation

### Examples

```text
Filesystem

GitHub

Browser

Search

Custom Tools
```

### Tasks

* Tool Registry
* Tool Metadata
* Validation Support
* Error Handling

---

# v0.5.0

## Official CLI

### Commands

```text
jules auth

jules chat

jules config

jules tools

jules doctor

jules version
```

### Features

* Interactive Mode
* Streaming Support
* Diagnostics
* Configuration Management

---

# v0.6.0

## Proc Macros

### Features

```rust
#[derive(Tool)]

#[derive(Session)]

#[jules_tool]

#[jules_builder]
```

### Tasks

* Proc Macros
* Validation Macros
* Documentation Support
* Testing

---

# v0.7.0

## Middleware System

### Features

```text
Logging

Retry

Caching

Rate Limiting

Telemetry

Authentication
```

### APIs

```rust
before_request()

after_response()

on_error()

on_retry()
```

### Tasks

* Middleware Pipeline
* Request Hooks
* Response Hooks
* Extensibility Improvements

---

# v0.8.0

## WASM Support

### Targets

```text
WASM

Cloudflare Workers

WASI

Deno
```

### Features

* Fetch API Support
* Lightweight Client
* Streaming Compatibility
* Feature Flag Support

---

# v0.9.0

## API Freeze Candidate

### Goals

Prepare the SDK for v1.0.0.

### Tasks

#### Performance

* Benchmarking
* Allocation Reduction
* Async Optimizations
* Memory Improvements

#### Security

* Dependency Auditing
* Security Scanning
* Secret Detection

#### Testing

* Integration Tests
* Contract Tests
* Snapshot Tests
* Performance Tests

#### Documentation

* API Reference
* Tutorials
* Examples
* Migration Guides

#### API Review

* Public API Review
* Naming Review
* Breaking Change Review
* Final Deprecations

### Deliverables

```text
API FREEZE CANDIDATE
```

---

# v1.0.0

## Stable Release

### Requirements

#### Core

* Stable Public APIs
* Complete Session Support
* Complete Conversation Support
* Tool Calling Support
* Streaming Support

#### Platform Support

* Linux
* Windows
* macOS
* WASM

#### Developer Experience

* Official CLI
* Comprehensive Documentation
* Production-grade Error Handling
* Extensive Examples

#### Quality Assurance

* Comprehensive Testing
* Security Auditing
* CI/CD Automation
* Performance Benchmarks

#### Documentation

* Rustdoc
* Tutorials
* Migration Guides
* API Reference
* Cookbook Examples

### Deliverables

```text
STABLE RELEASE

API STABILITY GUARANTEE

PRODUCTION READY

FULL DOCUMENTATION

LONG TERM MAINTAINABILITY
```

---

## Non Goals

Jules-SDK will NOT:

* Implement custom LLM models.
* Become a full agent framework.
* Include database implementations.
* Provide Web UI components.
* Replace Jules services.

---

## Success Criteria

The v1.0.0 release must provide:

* Stable Public APIs
* Production-ready SDK
* Streaming Support
* Tool Calling Support
* Official CLI
* WASM Compatibility
* Extensive Testing
* Comprehensive Documentation
* Multi-platform Compatibility
* Excellent Developer Experience

---

> Jules-SDK follows the Rust ecosystem philosophy of releasing early and iterating during the 0.x lifecycle. The project will prioritize API ergonomics, correctness, performance and long-term maintainability before committing to API stability in v1.0.0.
