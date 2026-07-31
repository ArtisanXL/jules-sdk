# Project State

> This document describes the current development status, priorities and milestones of Jules-SDK. Contributors and AI coding agents SHOULD review this document before making significant changes to the repository.

---

## Current Status

| Field             | Value                           |
| ----------------- | ------------------------------- |
| Project Name      | Jules-SDK                       |
| Current Version   | v0.1.0-dev                      |
| Development Stage | Pre-Alpha                       |
| Current Phase     | Phase 0 - Repository Foundation |
| Status            | In Progress                     |
| MSRV              | Rust 1.90+                      |
| Workspace Status  | In Progress                     |

---

## Current Milestone

> Establish the project's foundations before implementing public APIs.

### Milestone Goals

* Production-ready repository structure
* Cargo workspace initialization
* Documentation foundations
* CI/CD foundations
* Testing infrastructure
* Release management policies
* Security policies
* AI development guidelines

---

## Completed

### Documentation

* [x] [README.md](README.md)
* [x] [ROADMAP.md](ROADMAP.md)
* [x] [ARCHITECTURE.md](ARCHITECTURE.md)
* [x] [CONTRIBUTING.md](CONTRIBUTING.md)
* [x] [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
* [x] [VERSIONING.md](VERSIONING.md)
* [x] [RELEASE.md](RELEASE.md)
* [x] [TESTING.md](TESTING.md)
* [x] [CHANGELOG.md](CHANGELOG.md)
* [x] [SECURITY.md](SECURITY.md)
* [x] [FEATURES.md](FEATURES.md)
* [x] [SUPPORT.md](SUPPORT.md)
* [x] [MSRV.md](MSRV.md)
* [x] [AGENTS.md](AGENTS.md)

### Repository Policies

* [x] License policies established.
* [x] Versioning policies established.
* [x] Testing policies established.
* [x] Security policies established.
* [x] Release policies established.

---

## In Progress

### Workspace

* [x] Cargo workspace initialization.
* [ ] Crate organization.
* [ ] Feature flag implementation.
* [ ] CI pipeline implementation.

### Crates

* [ ] jules-sdk
* [ ] jules-core
* [ ] jules-api
* [ ] jules-macros
* [ ] jules-cli

---

## Planned

### Phase 1

* Core abstractions
* Session management
* Configuration management
* Error handling

### Phase 2

* API integrations
* Streaming support
* Conversations
* Builders

### Phase 3

* Tool calling support
* Middleware support
* CLI support
* Additional examples

### Phase 4

* WASM support
* Platform integrations
* Performance improvements
* Stability validations

### Phase 5

* API freeze preparations
* Compatibility validations
* Documentation finalization
* Release candidate preparations

---

## Current Priorities

The following items SHOULD receive the highest development priority.

1. Cargo workspace implementation.
2. Core crate architecture.
3. Testing infrastructure.
4. CI/CD pipelines.
5. Public API foundations.
6. Session abstractions.
7. Streaming abstractions.
8. Documentation improvements.

Contributors SHOULD prioritize existing milestones before introducing new functionality.

---

## Current Tasks

### High Priority

* [x] Initialize the Cargo workspace.
* [x] Implement crate structure.
* [x] Configure feature flags.
* [x] Configure CI workflows.
* [x] Establish testing pipelines.

### Medium Priority

* [x] Implement SessionBuilder.
* [x] Implement configuration management.
* [x] Implement core error types.
* [x] Add initial examples.

### Low Priority

* [ ] WASM integrations.
* [ ] Additional tooling support.
* [ ] Performance optimizations.

Task priorities MAY change throughout development.

---

## Current Crate Status

| Crate        | Status  |
| ------------ | ------- |
| jules-sdk    | In Progress |
| jules-core   | In Progress |
| jules-api    | In Progress |
| jules-macros | In Progress |
| jules-cli    | In Progress |

Crate statuses should be updated whenever implementation milestones are completed.

---

## Supported Features

### Stable

```text
None
```

### In Development

```text
streaming

tools

middleware

telemetry

cli

wasm
```

### Experimental

```text
experimental
```

No feature should be considered production-ready until explicitly documented otherwise.

---

## Known Blockers

```text
None
```

When blockers exist, contributors SHOULD document:

* The affected functionality.
* The reason for the blocker.
* Potential mitigation strategies.
* Relevant dependencies.

Examples include:

* Dependency limitations.
* Platform compatibility issues.
* Architectural redesign requirements.
* API changes.

---

## Breaking Changes

```text
None
```

Breaking changes SHOULD include:

* Affected versions.
* Migration considerations.
* Related documentation updates.
* Relevant release notes.

For additional information, please refer to:

* [VERSIONING.md](VERSIONING.md)
* [CHANGELOG.md](CHANGELOG.md)

---

## Repository Context Priority

Before implementing significant changes, contributors and AI coding agents SHOULD review the following documents in order:

1. [PROJECT_STATE.md](PROJECT_STATE.md)
2. [AGENTS.md](AGENTS.md)
3. [ROADMAP.md](ROADMAP.md)
4. [ARCHITECTURE.md](ARCHITECTURE.md)
5. [FEATURES.md](FEATURES.md)
6. [TESTING.md](TESTING.md)
7. [RELEASE.md](RELEASE.md)
8. [VERSIONING.md](VERSIONING.md)
9. Relevant Crate Documentation
10. Implementation

Repository-specific policies ALWAYS take precedence over external development skills and general coding practices.

---

## AI Development Guidelines

AI coding agents SHOULD:

* Review the current project state before implementation.
* Respect milestone priorities.
* Preserve architectural consistency.
* Avoid introducing undocumented functionality.
* Prefer incremental and well-tested changes.

AI coding agents MUST NOT:

* Introduce breaking changes silently.
* Ignore repository policies.
* Modify unrelated functionality unnecessarily.
* Claim successful validation without verification.

When uncertainty exists, contributors SHOULD prioritize clarification over architectural assumptions.

---

## Release Targets

| Version | Status      |
| ------- | ----------- |
| v0.1.0  | In Progress |
| v0.2.0  | Planned     |
| v0.5.0  | Planned     |
| v0.9.0  | Planned     |
| v1.0.0  | Planned     |

Release targets MAY evolve as development progresses.

---

## Last Updated

```text
2026-07-31
```

This document SHOULD be updated whenever:

* Milestones are completed.
* Priorities change.
* New blockers are discovered.
* Releases are published.
* Architectural changes occur.

---

## Final Notes

`PROJECT_STATE.md` exists to provide a concise and authoritative overview of the repository's current state.

Contributors are encouraged to prioritize:

* Stability
* Documentation
* Testing
* Maintainability
* Incremental development
* Clear communication

> Before writing code, understand where the project is today and where it is going tomorrow.
