# Testing Strategy

> Jules-SDK follows a quality-first testing strategy that prioritizes correctness, maintainability, performance and long-term API stability.

Testing is considered a fundamental part of the development process. No feature is considered complete until it is appropriately tested.

---

## Testing Philosophy

Jules-SDK is built upon the following testing principles:

* Correctness before optimization
* Comprehensive test coverage
* Production-grade quality standards
* Regression prevention
* Continuous validation
* Long-term maintainability

The project emphasizes:

* Unit testing
* Integration testing
* API validation
* Benchmarking
* Security validation
* Workspace-wide testing

---

## Testing Pyramid

```text
                    Benchmarks
                         │
                         ▼
                  Performance Tests
                         │
                         ▼
                 Integration Tests
                         │
                         ▼
                     Unit Tests
                         │
                         ▼
                     Rust Lints
                         │
                         ▼
                  Compilation Checks
```

Every release should pass all applicable layers of the testing pyramid.

---

## Testing Requirements

The following requirements apply to all contributions:

### Required

* Unit tests
* Integration tests
* Workspace validation
* Formatting validation
* Clippy validation
* Security validation

### Optional

* Benchmarks
* Snapshot testing
* Property testing
* Performance profiling

Performance-sensitive changes SHOULD include benchmarks whenever appropriate.

---

## Testing Categories

### Unit Tests

Unit tests are responsible for validating:

* Builders
* Core abstractions
* Error handling
* Utility functions
* Serialization
* Parsing logic

Examples include:

```text
Client

↓

Session

↓

Messages

↓

Configurations

↓

Builders

↓

Errors
```

### Requirements

Unit tests SHOULD:

* Be deterministic.
* Execute quickly.
* Test a single concern.
* Avoid unnecessary external dependencies.

---

## Integration Tests

Integration tests validate interactions between multiple components.

Examples include:

```text
Authentication

↓

Streaming APIs

↓

Session Management

↓

Tool Calling

↓

Feature Flags

↓

CLI Commands
```

Integration tests SHOULD validate:

* Public APIs
* Error handling
* Async behavior
* Feature compatibility
* Workspace interactions

---

## Workspace Testing

The entire workspace MUST compile successfully.

### Validation

```bash
cargo check --workspace --all-features
```

### Build Validation

```bash
cargo build --workspace --all-features
```

Requirements:

* No compilation failures.
* No broken feature combinations.
* No dependency issues.

---

## Unit Testing

Preferred tooling:

```text
cargo test
```

Examples:

```bash
cargo test --workspace
```

```bash
cargo test --all-features
```

Unit tests SHOULD cover:

* Public APIs
* Builders
* Configuration validation
* Error propagation
* Serialization logic

---

## Integration Testing

Preferred tooling:

```text
cargo-nextest
```

Examples:

```bash
cargo nextest run
```

```bash
cargo nextest run --all-features
```

Requirements:

* All integration tests must pass.
* No ignored failures are permitted for releases.
* Feature flag combinations should be validated whenever possible.

---

## Feature Flag Testing

Feature flags are considered part of the public API.

The following combinations SHOULD be tested whenever applicable:

```text
Default Features

↓

streaming

↓

tools

↓

cli

↓

middleware

↓

wasm

↓

telemetry

↓

experimental
```

Examples:

```bash
cargo test --no-default-features
```

```bash
cargo test --all-features
```

```bash
cargo check --all-features
```

Public feature flags must not introduce incompatible behavior.

---

## Documentation Testing

Examples provided within documentation should remain valid whenever possible.

Examples include:

```text
README.md

↓

Rustdoc Examples

↓

Guides

↓

Cookbooks

↓

Migration Guides
```

Public documentation is considered part of the project's quality standards.

---

## Benchmarking

Performance-sensitive changes SHOULD include benchmarks.

Examples include:

```text
Streaming

↓

Serialization

↓

HTTP Requests

↓

Builders

↓

Tool Execution
```

Preferred tooling:

```text
criterion
```

Examples:

```bash
cargo bench
```

Performance regressions should be investigated before release.

---

## Property Testing

Property testing is encouraged for:

* Parsers
* Serialization
* Builders
* Validation logic
* Utility functions

Preferred tooling:

```text
proptest
```

Property testing is particularly valuable when validating edge cases and unexpected inputs.

---

## Snapshot Testing

Snapshot testing MAY be used for:

* CLI outputs
* Error messages
* Documentation examples
* Serialization formats

Preferred tooling:

```text
insta
```

Examples include:

```text
CLI Output

↓

JSON Responses

↓

Streaming Events

↓

Public Error Messages
```

---

## Performance Testing

Performance-sensitive changes SHOULD be evaluated for:

* Allocation counts
* Memory usage
* Streaming performance
* Async performance
* Serialization performance

Preferred tooling includes:

```text
criterion

cargo-bloat

cargo-flamegraph
```

Performance regressions SHOULD be addressed before publication whenever practical.

---

## Security Testing

Security validation is required for every release.

### Dependency Auditing

```bash
cargo audit
```

### Dependency Validation

```bash
cargo deny check
```

Requirements include:

* No known critical vulnerabilities.
* Approved dependency licenses.
* Valid dependency graphs.

---

## Formatting Validation

Formatting checks are mandatory.

```bash
cargo fmt --all --check
```

Requirements:

* No formatting failures.
* Consistent code style throughout the workspace.

---

## Clippy Validation

Clippy checks are mandatory.

```bash
cargo clippy --workspace --all-features -- -D warnings
```

Requirements:

* No warnings.
* No denied lints.
* No obvious anti-patterns.

Public APIs should favor readability and maintainability over unnecessary complexity.

---

## Continuous Integration

The CI pipeline SHOULD validate:

```text
Formatting Checks
        ↓
Compilation Checks
        ↓
Clippy Validation
        ↓
Unit Tests
        ↓
Integration Tests
        ↓
Security Validation
        ↓
Workspace Validation
        ↓
Documentation Validation
        ↓
Release Validation
```

No release may proceed if required CI checks fail.

---

## Release Testing Requirements

Before publication, the following commands MUST succeed:

### Compilation

```bash
cargo check --workspace --all-features
```

### Build Validation

```bash
cargo build --workspace --all-features
```

### Tests

```bash
cargo nextest run
```

### Formatting

```bash
cargo fmt --all --check
```

### Clippy

```bash
cargo clippy --workspace --all-features -- -D warnings
```

### Security

```bash
cargo audit
```

```bash
cargo deny check
```

### Package Validation

```bash
cargo publish --dry-run
```

---

## Crate Testing Responsibilities

### jules-sdk

Responsible for testing:

* Public APIs
* Re-exports
* Feature flags

### jules-core

Responsible for testing:

* Core abstractions
* Traits
* Builders
* Errors
* Serialization

### jules-api

Responsible for testing:

* Authentication
* HTTP interactions
* Streaming
* Sessions
* Conversations

### jules-macros

Responsible for testing:

* Proc macros
* Validation logic
* Generated code

### jules-cli

Responsible for testing:

* Commands
* Interactive mode
* Configuration handling
* Diagnostics

---

## Testing Policy Before v1.0.0

During the `0.x` lifecycle:

* APIs may evolve.
* Tests should evolve alongside public APIs.
* Breaking changes require updated tests.
* Documentation examples should remain accurate.

Testing requirements are never relaxed for breaking changes.

---

## Testing Policy After v1.0.0

Beginning with `v1.0.0`:

* Public APIs are considered stable.
* Regression prevention becomes mandatory.
* Compatibility testing becomes increasingly important.
* New functionality must preserve backwards compatibility.

All public API changes should include appropriate test coverage.

---

## Final Notes

Jules-SDK considers testing to be an essential component of every contribution. Code that compiles successfully is not necessarily correct, and functionality is not considered complete until it has been tested, reviewed and documented appropriately.

Contributors are encouraged to prioritize:

* Correctness
* Maintainability
* Performance
* Security
* Developer experience
* Long-term API stability

> If it isn't tested, it isn't finished.
