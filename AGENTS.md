# AGENTS.md

> This document defines the expectations, responsibilities and development guidelines for both human contributors and AI coding agents working on Jules-SDK.

AI agents are expected to prioritize correctness, maintainability, documentation quality and long-term API stability over implementation speed.

---

## Core Principles

All contributions SHOULD prioritize:

* Correctness
* Maintainability
* Type safety
* Memory safety
* Developer experience
* Long-term API stability
* Production readiness
* Comprehensive testing
* Clear documentation

The preferred development philosophy is:

```text
Design First
      ↓
Write Tests
      ↓
Implement Features
      ↓
Review APIs
      ↓
Validate Documentation
      ↓
Run CI Checks
      ↓
Prepare Releases
```

No implementation should sacrifice maintainability for short-term convenience.

---

## Repository Context Priority

AI coding agents MUST review the following documents before making significant changes:

1. [PROJECT_STATE.md](PROJECT_STATE.md)
2. [AGENTS.md](AGENTS.md)
3. [ROADMAP.md](ROADMAP.md)
4. [ARCHITECTURE.md](ARCHITECTURE.md)
5. [FEATURES.md](FEATURES.md)
6. [TESTING.md](TESTING.md)
7. [RELEASE.md](RELEASE.md)
8. [VERSIONING.md](VERSIONING.md)
9. Relevant crate documentation.

PROJECT_STATE.md MUST be treated as the authoritative source for the repository's current development status and priorities.

---


## AI Development Skills

Additional repository-specific AI development skills may be provided under:

```text
.agents/skills/
```

AI coding agents SHOULD:

- Discover available skills automatically.
- Apply relevant skills when appropriate.
- Follow repository-specific development requirements.
- Preserve architectural consistency throughout the workspace.

The following priority order MUST be respected:

```text
Repository Requirements
        ↓
AGENTS.md
        ↓
Repository Documentation
        ↓
AI Development Skills
        ↓
General Language Best Practices
        ↓
General AI Coding Practices
```

If conflicts exist between an installed skill and repository policies, repository policies MUST take precedence.

---

## Repository Structure

```text
jules-sdk/

├── crates/
│
│   ├── jules-sdk/
│   ├── jules-core/
│   ├── jules-api/
│   ├── jules-macros/
│   └── jules-cli/
│
├── docs/
├── examples/
├── .github/
│
├── README.md
├── ROADMAP.md
├── ARCHITECTURE.md
├── CONTRIBUTING.md
├── VERSIONING.md
├── RELEASE.md
├── TESTING.md
├── FEATURES.md
├── SECURITY.md
├── SUPPORT.md
└── AGENTS.md
```

AI agents MUST preserve the existing repository structure unless architectural changes are explicitly required.

---

## Development Rules

AI agents SHOULD:

* Prefer small and focused changes.
* Preserve backwards compatibility whenever practical.
* Favor composition over unnecessary abstractions.
* Prefer explicit implementations over clever implementations.
* Minimize dependency additions.
* Avoid introducing unnecessary feature flags.

AI agents MUST NOT:

* Introduce undocumented public APIs.
* Remove existing functionality without justification.
* Introduce breaking changes silently.
* Ignore existing architectural decisions.
* Modify unrelated files unnecessarily.

---

## Public API Guidelines

Public APIs SHOULD be:

* Explicit
* Type-safe
* Well documented
* Easy to discover
* Backwards compatible after `v1.0.0`

AI agents SHOULD favor:

```text
Builders
    ↓
Traits
    ↓
Strong Types
    ↓
Explicit Errors
    ↓
Composable APIs
```

Avoid:

* Excessive generics
* Hidden side effects
* Unnecessary macros
* Implicit conversions

Developer experience is considered part of the public API.

---

## Dependency Policy

Before introducing a dependency, AI agents SHOULD consider:

* Is the dependency actively maintained?
* Is it widely adopted?
* Is it necessary?
* Does it increase the MSRV?
* Does it affect compile times significantly?
* Can existing dependencies provide similar functionality?

New dependencies SHOULD provide clear architectural or maintenance benefits.

AI agents SHOULD prefer:

* Tokio
* Serde
* Futures
* Reqwest
* Widely adopted Rust ecosystem crates

Avoid introducing niche or unmaintained dependencies whenever practical.

---

## Testing Requirements

Every implementation SHOULD include appropriate tests whenever practical.

Required validation includes:

```bash
cargo fmt --all --check
```

```bash
cargo clippy --workspace --all-features -- -D warnings
```

```bash
cargo check --workspace --all-features
```

```bash
cargo nextest run
```

AI agents MUST NOT claim that tests pass unless they have been executed successfully.

---

## Documentation Requirements

Documentation is considered part of every contribution.

The following files SHOULD be updated whenever appropriate:

* [README.md](README.md)
* [PROJECT_STATE.md](PROJECT_STATE.md)
* [CHANGELOG.md](CHANGELOG.md)
* [FEATURES.md](FEATURES.md)
* [TESTING.md](TESTING.md)
* [VERSIONING.md](VERSIONING.md)
* [RELEASE.md](RELEASE.md)
* [SECURITY.md](SECURITY.md)

Examples requiring documentation updates include:

* New public APIs
* Breaking changes
* New feature flags
* MSRV changes
* Platform support changes

Undocumented functionality is considered incomplete.

---

## Feature Flags

Feature flags SHOULD remain:

* Small
* Modular
* Explicit
* Well documented

Examples include:

```toml
streaming

tools

middleware

telemetry

cli

wasm

experimental
```

AI agents MUST NOT:

* Introduce hidden feature dependencies.
* Create monolithic feature flags.
* Modify stable feature behavior unexpectedly.

Feature flags are considered part of the public API beginning with `v1.0.0`.

---

## Security Guidelines

AI agents SHOULD prioritize:

* Safe Rust
* Defensive programming
* Explicit validation
* Secure defaults

Avoid introducing:

* Hardcoded credentials
* Sensitive information
* Unsafe code without justification
* Undocumented security-sensitive behavior

Whenever unsafe code is necessary, contributors SHOULD document:

* Why it is necessary.
* Which invariants are maintained.
* How correctness is validated.

---

## Workspace Responsibilities

### jules-sdk

Responsible for:

* Public APIs
* Re-exports
* Feature management

### jules-core

Responsible for:

* Traits
* Builders
* Core abstractions
* Error handling

### jules-api

Responsible for:

* API integrations
* Sessions
* Conversations
* Streaming functionality

### jules-macros

Responsible for:

* Proc macros
* Compile-time validations
* Code generation

### jules-cli

Responsible for:

* CLI functionality
* Diagnostics
* Configuration management

AI agents SHOULD preserve crate boundaries whenever practical.

---

## Pull Request Expectations

Contributions SHOULD remain:

* Small
* Focused
* Well documented
* Properly tested

Preferred workflow:

```text
Review Requirements
        ↓
Design Changes
        ↓
Implement Changes
        ↓
Write Tests
        ↓
Update Documentation
        ↓
Run Validation Checks
        ↓
Prepare Pull Request
```

Large architectural changes SHOULD be separated into smaller and reviewable contributions.

---

## Breaking Changes

AI agents MUST explicitly document:

* Breaking changes
* API redesigns
* Feature removals
* MSRV increases
* Migration considerations

Breaking changes after `v1.0.0` MUST follow Semantic Versioning requirements.

---

## Performance Guidelines

AI agents SHOULD consider:

* Compile times
* Memory allocations
* Async performance
* Binary size
* Dependency impact

Performance optimizations SHOULD NOT sacrifice:

* Readability
* Maintainability
* API clarity

Premature optimization should be avoided.

---

## Release Requirements

Before a release, contributors SHOULD validate:

```text
Formatting Checks
        ↓
Compilation Checks
        ↓
Tests Passed
        ↓
Security Validation
        ↓
Documentation Updated
        ↓
Version Updated
        ↓
Release Validation
```

Release policies are defined in:

* RELEASE.md
* VERSIONING.md
* TESTING.md

---

## AI Agent Expectations

AI coding agents SHOULD:

* Read existing documentation before implementing changes.
* Review the active development goals, priorities, and blockers in [PROJECT_STATE.md](PROJECT_STATE.md).
* Respect established architectural decisions.
* Preserve API consistency.
* Prefer incremental improvements.
* Provide well-reasoned implementations.

AI coding agents MUST NOT:

* Invent undocumented behavior.
* Introduce breaking changes silently.
* Ignore feature flag requirements.
* Modify unrelated functionality.
* Claim successful validation without performing it.

When uncertainty exists, agents SHOULD prefer asking for clarification rather than making architectural assumptions.

---

## Final Notes

Jules-SDK prioritizes correctness, maintainability and developer experience above implementation speed.

All contributors—human and AI alike—are expected to value:

* Stability
* Documentation
* Testing
* Security
* Maintainability
* Long-term ecosystem compatibility

> Well-designed software is implemented deliberately, documented clearly and maintained responsibly.

## Local CI Verification

Before pushing code or opening a PR, verify changes pass CI locally using `act`.

### Prerequisites
- Docker must be running
- If `act` is not installed, run: `bash scripts/act/install-act.sh`

### How to Verify

1. Read `.github/workflows/` to find the CI workflow and identify the job ID
2. Run the verification script:
   ```bash
   bash scripts/act/run-act.sh "push -j <JOB_ID>"
   ```
   With matrix: `bash scripts/act/run-act.sh "push -j <JOB_ID> --matrix <KEY>:<VALUE>"`
3. If the run fails, read the log output, fix the code, and re-run
4. After verification, clean up:
   ```bash
   rm -f act_output.log
   git checkout <any unintended file changes>
   ```

### Configuration
- Timeout: `ACT_TIMEOUT=900 bash scripts/act/run-act.sh "..."`  (default: 600s)
- Poll interval: `ACT_POLL=15 bash scripts/act/run-act.sh "..."`  (default: 10s)
- Custom image: pass `-P ubuntu-latest=node:20-bookworm` in the arguments for faster pulls
