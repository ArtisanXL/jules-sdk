# Contributing to Jules-SDK

First of all, thank you for considering contributing to Jules-SDK.

Whether you're fixing bugs, improving documentation, writing tests or proposing new features, your contributions are greatly appreciated.

Please take a few minutes to read this document before opening an issue or submitting a pull request.

---

## Table of Contents

* Code of Conduct
* Getting Started
* Reporting Issues
* Feature Requests
* Development Setup
* Pull Request Guidelines
* Coding Standards
* Testing Requirements
* Documentation Requirements
* Commit Message Guidelines
* Review Process
* Release Policy

---

## Code of Conduct

By participating in this project, you agree to abide by the guidelines described in:

```text
CODE_OF_CONDUCT.md
```

Please be respectful and constructive in all discussions and contributions.

---

## Before Contributing

Before opening a Pull Request, please ensure that:

* The issue has not already been reported.
* Existing discussions have been reviewed.
* The repository's current status and active development priorities in [PROJECT_STATE.md](PROJECT_STATE.md) have been reviewed.
* Your changes align with the project's architecture and goals.
* Documentation has been updated when necessary.
* Appropriate tests have been added or updated.

---

## Reporting Issues

When reporting bugs, please include:

* Jules-SDK version
* Rust version
* Operating system
* Steps to reproduce
* Expected behavior
* Actual behavior
* Relevant logs or error messages

Providing minimal reproducible examples is highly encouraged.

---

## Feature Requests

Feature requests should include:

* The problem being solved
* Proposed API design
* Expected behavior
* Potential alternatives
* Backwards compatibility considerations

Large features should preferably be discussed before implementation.

---

## Development Setup

### Clone the Repository

```bash
git clone https://github.com/<organization>/jules-sdk.git

cd jules-sdk
```

### Build the Workspace

```bash
cargo build --workspace --all-features
```

### Run Tests

```bash
cargo nextest run
```

### Run Formatting Checks

```bash
cargo fmt --all --check
```

### Run Clippy

```bash
cargo clippy --workspace --all-features -- -D warnings
```

### Run Security Checks

```bash
cargo audit

cargo deny check
```

---

## Branch Naming

Preferred branch naming conventions:

```text
feature/session-support

feature/streaming-api

fix/authentication-error

fix/documentation-typo

docs/readme-updates

refactor/client-builder
```

---

## Pull Request Guidelines

Every Pull Request should:

* Be focused on a single concern.
* Include appropriate tests.
* Include documentation updates when required.
* Pass all CI checks.
* Follow the project's coding standards.

### Small Pull Requests Are Preferred

Prefer:

```text
300 lines
↓

Single feature

↓

Easy to review
```

Avoid:

```text
5000+ lines
↓

Multiple unrelated changes

↓

Difficult to review
```

Smaller and focused Pull Requests are significantly easier to review and maintain.

---

## Coding Standards

Jules-SDK follows idiomatic Rust practices.

### General Guidelines

* Prefer readability over cleverness.
* Favor composition over unnecessary abstractions.
* Keep public APIs minimal.
* Avoid premature optimizations.
* Prefer explicit code over implicit behavior.

### API Design Principles

Public APIs should be:

* Discoverable
* Type-safe
* Well documented
* Consistent
* Ergonomic

### Builder Pattern

Prefer:

```rust
Client::builder()
    .api_key("...")
    .timeout(30)
    .build()?;
```

Avoid introducing unnecessary constructors when a builder provides a better developer experience.

---

## Crate Responsibilities

Contributions should respect crate boundaries.

```text
jules-sdk
↓

Public APIs only


----------------------


jules-core
↓

Core abstractions


----------------------


jules-api
↓

API implementation


----------------------


jules-macros
↓

Proc macros


----------------------


jules-cli
↓

CLI implementation
```

### Circular Dependencies

Circular dependencies are not permitted.

```text
Allowed

jules-api
↓

jules-core


----------------------


NOT ALLOWED

jules-core
↓

jules-api
```

---

## Testing Requirements

All contributions should include appropriate testing.

### Unit Tests

Required for:

* Core functionality
* Builders
* Utility functions
* Error handling

### Integration Tests

Required for:

* API interactions
* Streaming functionality
* Session management
* Tool calling

### Benchmarking

Performance-sensitive changes should include benchmarks when appropriate.

---

## Documentation Requirements

Documentation is considered part of the implementation.

Documentation updates may include:

* Rustdoc
* README.md
* Examples
* Guides
* Migration notes

Public APIs should be documented whenever possible.

---

## Commit Message Guidelines

Preferred format:

```text
feat: add session builder

fix: improve authentication handling

docs: update roadmap

refactor: simplify streaming implementation

test: add integration tests for sessions

perf: reduce allocations in parser
```

Examples:

```text
feat(api): add streaming support

fix(core): improve builder validation

docs(readme): update installation guide

test(cli): add configuration tests
```

---

## Review Process

Pull Requests are reviewed for:

* Correctness
* API design
* Testing
* Documentation
* Performance considerations
* Maintainability
* Backwards compatibility

Reviewers may request:

* Additional tests
* Documentation updates
* API improvements
* Architectural changes

Constructive feedback is encouraged throughout the review process.

---

## Breaking Changes

Before `v1.0.0`:

```text
Breaking Changes

↓

Allowed when justified.
```

After `v1.0.0`:

```text
Breaking Changes

↓

Major versions only.
```

All breaking changes must be documented appropriately.

---

## Security Contributions

If your contribution addresses a security issue, please review:

```text
SECURITY.md
```

Please avoid publicly disclosing security vulnerabilities before they have been reviewed.

---

## Release Requirements

Code intended for release should:

* Pass all CI checks.
* Pass all tests.
* Pass formatting checks.
* Pass Clippy checks.
* Pass security checks.
* Include documentation updates when necessary.

Minimum requirements include:

```text
cargo nextest run

cargo fmt --check

cargo clippy --all-features

cargo audit

cargo deny check
```

---

## Architectural Decisions

Before introducing:

* New public APIs
* New crates
* Breaking changes
* Large refactors
* New feature flags

Please consider whether the proposed changes align with:

* [PROJECT_STATE.md](PROJECT_STATE.md)
* [ARCHITECTURE.md](ARCHITECTURE.md)
* [ROADMAP.md](ROADMAP.md)
* [VERSIONING.md](VERSIONING.md)

The project intentionally favors simplicity over unnecessary abstractions. New crates should only be introduced when they provide clear architectural benefits.

---

## Thank You

Every contribution—whether it is code, documentation, tests, examples or discussions—helps improve Jules-SDK.

Thank you for helping make Jules-SDK a production-ready and idiomatic Rust SDK for the community.

