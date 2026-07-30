# Release Policy

> Jules-SDK follows a quality-first release process. Every release must satisfy the project's requirements for correctness, testing, documentation and maintainability before being published.

---

## Release Philosophy

Jules-SDK adopts the Rust ecosystem philosophy of:

* Release early
* Iterate responsibly
* Maintain production-grade quality standards
* Prioritize long-term maintainability
* Preserve API stability after `v1.0.0`

No release should be published unless it satisfies the minimum quality requirements defined in this document.

---

## Release Lifecycle

```text
Planning
    ↓
Implementation
    ↓
Testing
    ↓
Code Review
    ↓
Documentation Review
    ↓
CI Validation
    ↓
Release Candidate
    ↓
Version Tagging
    ↓
crates.io Publication
    ↓
GitHub Release
    ↓
Release Notes
```

---

## Release Types

### Initial Release

```text
v0.1.0
```

Requirements:

* Workspace is functional.
* Core APIs are implemented.
* Documentation is available.
* CI pipelines are operational.
* Test suites are passing.

---

### Feature Releases

```text
0.x.0

↓

1.1.0

↓

1.2.0
```

Requirements:

* New functionality is implemented.
* Documentation is updated.
* Existing tests continue to pass.
* Public APIs are reviewed.

---

### Patch Releases

```text
1.0.1

↓

1.1.1

↓

1.2.1
```

Requirements:

* Bug fixes
* Security fixes
* Documentation corrections
* Performance improvements

Patch releases MUST NOT introduce breaking changes.

---

### Major Releases

```text
2.0.0

↓

3.0.0
```

Requirements:

* Breaking changes are documented.
* Migration documentation is provided.
* Public APIs are reviewed.
* Upgrade paths are explained.

---

## Release Requirements

Every release MUST satisfy the following requirements.

### Formatting

```bash
cargo fmt --all --check
```

Must pass successfully.

---

### Clippy

```bash
cargo clippy --workspace --all-features -- -D warnings
```

Requirements:

* No warnings.
* No denied lints.
* No unnecessary allocations when avoidable.

---

### Testing

```bash
cargo nextest run
```

Requirements:

* All tests must pass.
* No ignored failures.
* Integration tests must pass successfully.

---

### Security Checks

```bash
cargo audit
```

```bash
cargo deny check
```

Requirements:

* No known critical vulnerabilities.
* Dependency licenses must comply with the project's licensing policy.

---

### Workspace Validation

```bash
cargo check --workspace --all-features
```

```bash
cargo build --workspace --all-features
```

Requirements:

* Successful compilation.
* No workspace failures.

---

### Documentation Validation

The following documentation MUST be reviewed when appropriate:

```text
README.md

ROADMAP.md

ARCHITECTURE.md

CHANGELOG.md

VERSIONING.md

CONTRIBUTING.md

TESTING.md

FEATURES.md
```

Documentation changes are required whenever:

* Public APIs change.
* New features are added.
* Breaking changes are introduced.
* Examples become outdated.

---

## Release Checklist

### Code Quality

* [ ] Formatting checks passed.
* [ ] Clippy checks passed.
* [ ] Workspace builds successfully.
* [ ] CI pipelines passed.
* [ ] Tests passed successfully.

### Documentation

* [ ] README updated.
* [ ] CHANGELOG updated.
* [ ] Documentation reviewed.
* [ ] Examples verified.

### Security

* [ ] cargo-audit passed.
* [ ] cargo-deny passed.
* [ ] Dependencies reviewed.

### API Review

* [ ] Public APIs reviewed.
* [ ] Feature flags reviewed.
* [ ] Breaking changes documented.
* [ ] Deprecations documented.

### Publication

* [ ] Version updated.
* [ ] Git tag prepared.
* [ ] crates.io publication completed.
* [ ] GitHub release created.

---

## CI Requirements

All release candidates MUST successfully complete:

```text
Formatting Checks

↓

Unit Tests

↓

Integration Tests

↓

Security Checks

↓

Workspace Builds

↓

Documentation Checks

↓

Release Validation
```

No release may proceed if any required CI job fails.

---

## Release Candidate Policy

Large releases SHOULD be treated as release candidates before publication.

Examples:

```text
v0.9.0

↓

Release Candidate


---------------------


v1.0.0

↓

Release Candidate


---------------------


v2.0.0

↓

Release Candidate
```

Release candidates should focus on:

* API validation
* Performance validation
* Documentation review
* Community feedback

---

## Version Updates

Before publication:

```text
Cargo.toml

↓

Version Updated


---------------------


CHANGELOG.md

↓

Updated


---------------------


Documentation

↓

Updated
```

All published versions MUST be reflected consistently throughout the repository.

---

## CHANGELOG Policy

Every release MUST include release notes.

### Example

```text
v0.4.0

Added
------
- Tool calling support

Changed
--------
- Streaming APIs

Deprecated
-----------
- Legacy builders

Fixed
------
- Session handling bugs

Security
---------
- Dependency updates
```

All release notes should remain concise, accurate and actionable.

---

## crates.io Publication

Before publishing:

```bash
cargo package
```

```bash
cargo publish --dry-run
```

Requirements:

* Successful package validation.
* Successful dry-run publication.
* No missing files.
* No packaging warnings requiring action.

After validation:

```bash
cargo publish
```

For workspace releases:

```bash
cargo publish -p jules-core

cargo publish -p jules-api

cargo publish -p jules-sdk
```

Publication order should respect crate dependencies.

---

## Git Tagging

Preferred format:

```text
v0.1.0

v0.2.0

v1.0.0

v1.1.0
```

Examples:

```bash
git tag v0.1.0

git push origin v0.1.0
```

Git tags should always correspond to published releases.

---

## Performance Requirements

Performance-sensitive changes SHOULD include:

* Benchmarks
* Allocation reviews
* Streaming performance validation
* Async performance validation

Suggested tooling includes:

```text
criterion

cargo-bloat

cargo-flamegraph

cargo-nextest
```

Performance regressions should be investigated before publication.

---

## Security Releases

Security releases may be published independently of feature releases.

Examples include:

```text
Critical Vulnerabilities

↓

Immediate Release


---------------------


Dependency Vulnerabilities

↓

Patch Release


---------------------


Security Improvements

↓

Minor Release
```

Security-related releases should be prioritized appropriately.

---

## Breaking Changes

Before introducing breaking changes:

* Review VERSIONING.md.
* Update CHANGELOG.md.
* Update migration documentation.
* Review public APIs.

Breaking changes introduced after `v1.0.0` MUST only occur through major releases.

Examples:

```text
Allowed

↓

v2.0.0


---------------------


NOT ALLOWED

↓

v1.2.1
```

---

## Release Schedule

Jules-SDK does not follow a fixed release schedule.

Releases are made when:

* Features are complete.
* Quality requirements are satisfied.
* Documentation is complete.
* CI requirements are fulfilled.

The project intentionally favors quality over release frequency.

---

## Publication Requirements Summary

A release is eligible for publication only if:

```text
Formatting Passed

        ↓

Clippy Passed

        ↓

Tests Passed

        ↓

Security Checks Passed

        ↓

Documentation Complete

        ↓

API Review Complete

        ↓

CI Successful

        ↓

cargo publish --dry-run

        ↓

Git Tag Created

        ↓

crates.io Publication

        ↓

GitHub Release Published
```

---

## Final Notes

Every published version of Jules-SDK represents a commitment to quality, correctness and maintainability.

Contributors are encouraged to prioritize:

* Stability
* Documentation
* Testing
* Performance
* Security
* Developer experience

No feature is considered complete until it is properly tested, documented and reviewed.

> If it isn't tested, documented and reviewed, it isn't ready for release.
