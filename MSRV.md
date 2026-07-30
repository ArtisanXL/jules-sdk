# Minimum Supported Rust Version (MSRV)

> Jules-SDK follows a conservative and stability-focused Minimum Supported Rust Version (MSRV) policy to provide a reliable developer experience across supported platforms.

The project's MSRV policy balances:

* Long-term maintainability
* Ecosystem compatibility
* Developer experience
* Stability
* Performance
* Access to modern Rust language features

---

## Current MSRV

```text
Rust 1.90+
```

All officially supported crates within the Jules-SDK workspace MUST compile successfully on the project's current MSRV.

This requirement applies to:

* jules-sdk
* jules-core
* jules-api
* jules-macros
* jules-cli

---

## Supported Platforms

The MSRV policy applies to all supported platforms, including:

* Linux
* Windows
* macOS
* WASM
* Cloudflare Workers
* WASI

Platform-specific limitations may apply depending on enabled feature flags.

---

## MSRV Philosophy

Jules-SDK follows the principle of:

```text
Prefer Stability
        ↓
Prefer Compatibility
        ↓
Upgrade Deliberately
        ↓
Avoid Frequent MSRV Changes
        ↓
Maintain Developer Experience
```

The project intentionally avoids unnecessarily frequent MSRV upgrades.

---

## Versioning Policy

### Before v1.0.0

During the `0.x` development lifecycle:

* MSRV changes are permitted when justified.
* Contributors SHOULD avoid introducing unnecessary MSRV increases.
* MSRV changes MUST be documented appropriately.

Examples include:

```text
v0.1.0

↓

Rust 1.90


------------------


v0.4.0

↓

Rust 1.92


------------------


Allowed When Justified
```

---

### After v1.0.0

Beginning with `v1.0.0`:

* MSRV changes are considered breaking changes whenever they affect users.
* MSRV upgrades SHOULD occur conservatively.
* Major releases are preferred when increasing the MSRV.

Examples include:

```text
v1.0.0

↓

Rust 1.90


------------------


v2.0.0

↓

Rust 1.94


------------------


Preferred Approach
```

For more information, please refer to:

* VERSIONING.md
* RELEASE.md

---

## Dependency Policy

All dependencies SHOULD:

* Support the project's MSRV.
* Be actively maintained.
* Avoid introducing unnecessary MSRV increases.
* Maintain ecosystem compatibility whenever practical.

New dependencies SHOULD be evaluated for:

* MSRV requirements
* Maintenance status
* Ecosystem adoption
* Long-term sustainability

Contributors SHOULD avoid introducing dependencies that significantly increase the project's MSRV without clear architectural or maintenance benefits.

---

## Contributor Requirements

Before introducing code that requires a newer Rust version, contributors SHOULD consider:

* Is the MSRV increase necessary?
* Is there a compatible alternative?
* Does the change improve maintainability?
* Does the change affect public APIs?
* Does the change justify the upgrade cost?

MSRV upgrades should be deliberate rather than incidental.

---

## CI Requirements

Continuous Integration SHOULD validate:

```text
Formatting Checks
        ↓
MSRV Compilation
        ↓
Workspace Compilation
        ↓
Unit Tests
        ↓
Integration Tests
        ↓
Security Validation
        ↓
Release Validation
```

MSRV validation is considered part of the project's release requirements.

---

## Workspace Validation

All workspace crates MUST successfully compile using the current MSRV.

Examples include:

```bash
cargo check --workspace
```

```bash
cargo build --workspace
```

```bash
cargo test --workspace
```

Feature flag combinations SHOULD also be validated whenever practical.

Examples:

```bash
cargo check --all-features

cargo check --no-default-features
```

---

## Feature Flags

Feature flags SHOULD NOT unnecessarily increase the project's MSRV.

Examples include:

```text
streaming

tools

middleware

telemetry

wasm

experimental
```

When a feature flag requires a newer Rust version, the requirement MUST be clearly documented.

---

## Documentation Requirements

Whenever the MSRV changes, the following documentation SHOULD be updated:

* [README.md](README.md)
* [PROJECT_STATE.md](PROJECT_STATE.md)
* [MSRV.md](MSRV.md)
* [CHANGELOG.md](CHANGELOG.md)
* [VERSIONING.md](VERSIONING.md)
* Release Notes

Documentation updates MUST clearly communicate:

* The previous MSRV.
* The new MSRV.
* The reason for the change.
* Any migration considerations.

---

## Release Requirements

Before publishing a release, contributors SHOULD ensure that:

```text
MSRV Validation Passed
            ↓
Workspace Compilation Passed
            ↓
Tests Passed
            ↓
Documentation Updated
            ↓
CI Validation Passed
            ↓
Release Approved
```

MSRV validation is considered a required component of release readiness.

---

## Public API Considerations

MSRV changes may affect:

* Public APIs
* Feature flags
* Platform support
* Dependency compatibility
* Build environments

When practical, backwards-compatible approaches SHOULD be preferred over increasing the project's MSRV requirements.

---

## Upgrade Policy

The project follows a conservative upgrade strategy.

Preferred approach:

```text
New Rust Release
        ↓
Ecosystem Adoption
        ↓
Dependency Compatibility Review
        ↓
MSRV Evaluation
        ↓
CI Validation
        ↓
Documentation Updates
        ↓
Future Release Planning
```

MSRV upgrades SHOULD NOT occur solely because a newer Rust version becomes available.

The project prioritizes ecosystem stability and developer experience over adopting the latest language features immediately.

---

## Exceptions

In limited circumstances, an MSRV increase may be justified due to:

* Security considerations
* Dependency requirements
* Significant performance improvements
* Long-term maintainability benefits
* Platform compatibility improvements

Such changes SHOULD be documented thoroughly and communicated clearly through release notes.

---

## Future Considerations

As Jules-SDK evolves, the MSRV policy may incorporate additional considerations regarding:

* WASM compatibility
* Cloudflare Workers support
* Async ecosystem changes
* Dependency maintenance policies
* Platform-specific requirements

Any changes to the MSRV policy will be documented appropriately.

---

## Final Notes

The Minimum Supported Rust Version policy exists to provide a predictable and stable experience for both contributors and users of Jules-SDK.

Contributors are encouraged to prioritize:

* Stability
* Compatibility
* Maintainability
* Conservative upgrades
* Clear documentation
* Long-term ecosystem support

> Upgrade deliberately. Maintain compatibility. Prioritize stability.
