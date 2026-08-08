# Changelog

All notable changes to Jules-SDK will be documented in this file.

The format is based on Keep a Changelog and this project adheres to Semantic Versioning (SemVer).

## Versioning

* MAJOR versions introduce breaking changes.
* MINOR versions introduce backwards-compatible features.
* PATCH versions introduce backwards-compatible bug fixes.

For more information, please refer to:

* VERSIONING.md
* RELEASE.md

---

## [Unreleased]

### Added

* Nothing yet.

### Changed

* Nothing yet.

### Deprecated

* Nothing yet.

### Removed

* Nothing yet.

### Fixed

* Nothing yet.

### Security

* Nothing yet.

---

## [0.1.0-rc.1] - 2026-08-02

> Pre-alpha scaffolding for Jules-SDK. Not a functional release — nothing in this
> workspace can make a real request to the Jules API yet. See
> [PROJECT_STATE.md](PROJECT_STATE.md) for accurate, per-crate implementation status.

### Added

#### Workspace

* Cargo workspace architecture.
* Five-crate project structure.
* Repository layout and policy documentation.

#### Core Functionality

* Public `jules-sdk` facade crate (re-exports only).
* Core abstractions (types, builders, trait signatures — no working transport).
* Builder pattern foundations.
* Error handling infrastructure.
* Configuration management.

#### API Support

* Type-safe request and response data models (not wired to any real HTTP transport).
* Async-first API design.

#### Integrations & Features
* Tool registry scaffolding (not wired to any model/API round-trip).
* Middleware pipeline and logging middleware (retry middleware does not yet retry — see PROJECT_STATE.md).
* CLI crate skeleton (no argument parsing or subcommands implemented yet).
* Streaming/SSE parsing utilities (not connected to any real endpoint).
* Session and Conversation builders (data-only; no CRUD or network calls).

#### Documentation

* README.md
* PROJECT_STATE.md
* ROADMAP.md
* ARCHITECTURE.md
* CONTRIBUTING.md
* CODE_OF_CONDUCT.md
* VERSIONING.md
* RELEASE.md
* TESTING.md
* CHANGELOG.md
* SECURITY.md
* FEATURES.md
* SUPPORT.md
* MSRV.md
* AGENTS.md

#### Development Tooling

* Cargo workspace support.
* Clippy configuration.
* Formatting validation.
* Workspace-wide testing support.
* CI/CD foundations.

### Changed

* Initial implementation.

### Deprecated

* None.

### Removed

* None.

### Fixed

* None.

### Security

* Initial security policies and dependency validation support.

---

## [0.2.0] - Planned

### Added

* Session management.
* Conversation management.
* Configuration improvements.
* Additional integration tests.

### Changed

* Builder APIs.
* Public ergonomics improvements.

### Deprecated

* None.

### Removed

* None.

### Fixed

* Bug fixes and documentation improvements.

### Security

* Dependency updates.

---

## [0.3.0] - Planned

### Added

* Streaming APIs.
* Streaming events.
* Async stream handling.
* Additional examples.

### Changed

* Performance improvements.
* Internal API optimizations.

### Fixed

* Streaming related issues.

---

## [0.4.0] - Planned

### Added

* Tool calling support.
* Tool abstractions.
* Tool execution APIs.

### Changed

* Session integrations.
* Streaming integrations.

### Fixed

* API handling improvements.

---

## [0.5.0] - Planned

### Added

* Official CLI implementation.
* Configuration support.
* Diagnostics and debugging tools.

### Changed

* Developer experience improvements.

### Fixed

* CLI related issues.

---

## [0.6.0] - Planned

### Added

* Proc macros support.
* Additional derive macros.
* Compile-time validations.

### Changed

* Macro ergonomics improvements.

---

## [0.7.0] - Planned

### Added

* Middleware system.
* Middleware abstractions.
* Request and response interception support.

### Changed

* Internal architecture improvements.

---

## [0.8.0] - Planned

### Added

* WASM support.
* Cloudflare Workers compatibility improvements.
* Feature flag enhancements.

### Changed

* Platform compatibility improvements.

---

## [0.9.0] - Planned

> API Freeze Candidate

### Added

* Release candidate improvements.
* Stability validations.
* Additional compatibility tests.

### Changed

* Public API refinements.
* Documentation improvements.

### Fixed

* Final pre-1.0 bug fixes.

### Security

* Dependency and security reviews.

---

## [1.0.0] - Planned

> Stable Release

### Added

* Stable public APIs.
* Production-ready documentation.
* Long-term API stability guarantees.
* Comprehensive test coverage.
* Stable feature flags.

### Changed

* Public APIs finalized.
* Documentation finalized.

### Fixed

* Final stability improvements.

### Security

* Production release security review completed.

---

## Changelog Categories

Every release SHOULD document changes using the following categories:

| Category   | Purpose                           |
| ---------- | --------------------------------- |
| Added      | New functionality                 |
| Changed    | Changes to existing functionality |
| Deprecated | Features scheduled for removal    |
| Removed    | Removed functionality             |
| Fixed      | Bug fixes                         |
| Security   | Security-related improvements     |

---

## Breaking Changes

Breaking changes MUST:

* Be clearly documented.
* Include migration guidance when appropriate.
* Follow the project's versioning policy.
* Be highlighted in release notes.

Examples include:

* Public API changes.
* Feature flag changes.
* MSRV changes.
* Module reorganizations.
* Builder API changes.

For more information, see `VERSIONING.md`.

---

## Release Notes Policy

Every published release SHOULD include:

* New features.
* API changes.
* Bug fixes.
* Security updates.
* Documentation updates.
* Performance improvements.
* Breaking changes, when applicable.

Release notes should remain:

* Concise
* Accurate
* Actionable
* User-focused

---

## Migration Policy

Whenever practical, migration guidance SHOULD be provided for:

* Breaking changes.
* Deprecated APIs.
* Major feature redesigns.
* Public API replacements.

Migration documentation may be provided through:

* CHANGELOG.md
* README.md
* Migration guides
* Release notes

---

## Final Notes

The Changelog serves as the authoritative record of notable changes made to Jules-SDK throughout its lifecycle.

Contributors are encouraged to keep entries:

* Accurate
* Concise
* Well organized
* User focused

Every release should clearly communicate what changed, why it changed and how users may be affected.

> Good release notes are part of the developer experience.
