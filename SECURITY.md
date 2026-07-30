# Security Policy

> Security is a fundamental requirement of Jules-SDK. We are committed to providing secure, reliable and maintainable software for the Rust ecosystem.

This document describes how to report security vulnerabilities, our supported versions policy and the security standards expected throughout the project's lifecycle.

---

## Supported Versions

The following table describes which versions currently receive security updates.

| Version              | Supported |
| -------------------- | --------- |
| 1.x                  | Yes       |
| 0.9.x                | Yes       |
| 0.8.x                | Yes       |
| Older 0.x Releases   | No        |
| Unsupported Releases | No        |

> **Note:** Prior to `v1.0.0`, only the latest actively maintained `0.x` release series will receive security updates.

---

## Reporting a Vulnerability

Please DO NOT report security vulnerabilities through:

* Public GitHub Issues
* Pull Requests
* Discussions
* Social media platforms

Instead, please report vulnerabilities privately.

### Information to Include

Security reports SHOULD include:

* Jules-SDK version
* Rust version
* Operating system
* Description of the vulnerability
* Reproduction steps
* Proof-of-concept code, when applicable
* Potential impact assessment
* Suggested mitigations, if available

Providing a minimal reproducible example is highly encouraged.

---

## Responsible Disclosure

We kindly request that security researchers and contributors follow responsible disclosure practices.

The preferred process is:

```text
Discover Vulnerability
          ↓
     Private Report
          ↓
Initial Investigation
          ↓
Impact Assessment
          ↓
      Security Fix
          ↓
      New Release
          ↓
Public Disclosure
```

Please allow maintainers reasonable time to investigate and remediate reported issues before public disclosure.

---

## Security Response Process

Security reports will generally follow the process below:

```text
Security Report Received
            ↓
      Initial Review
            ↓
      Severity Assessment
            ↓
     Reproducibility Check
            ↓
       Security Fix
            ↓
       Release Process
            ↓
      Security Advisory
            ↓
       Public Disclosure
```

Depending on severity, security releases may be published independently from feature releases.

---

## Severity Classification

Security vulnerabilities are generally categorized as:

### Critical

Examples include:

* Remote code execution
* Authentication bypass
* Severe dependency vulnerabilities
* Sensitive data exposure

### High

Examples include:

* Privilege escalation
* Serious API vulnerabilities
* Severe denial-of-service vectors

### Medium

Examples include:

* Information disclosure issues
* Configuration vulnerabilities
* Resource exhaustion vulnerabilities

### Low

Examples include:

* Minor information leaks
* Limited denial-of-service scenarios
* Documentation-related security concerns

Severity classifications are intended as general guidance and may vary based on context.

---

## Security Requirements

Security is considered part of the project's definition of done.

All contributions SHOULD prioritize:

* Correctness
* Memory safety
* Type safety
* Principle of least privilege
* Defensive programming
* Secure defaults

Public APIs should favor secure and explicit behavior whenever practical.

---

## Dependency Security Policy

Dependencies should satisfy the following requirements whenever possible:

* Actively maintained
* Widely adopted
* Security reviewed
* Compatible licensing
* Production-ready

New dependencies SHOULD be evaluated for:

* Maintenance status
* Security history
* Ecosystem adoption
* Documentation quality
* Long-term sustainability

---

## Dependency Validation

Preferred tooling includes:

```bash
cargo audit
```

```bash
cargo deny check
```

Security validation is expected before every release.

Requirements include:

* No known critical vulnerabilities.
* Approved dependency licenses.
* Valid dependency graphs.
* No abandoned dependencies when avoidable.

---

## Unsafe Code Policy

Jules-SDK prioritizes Rust's safety guarantees.

The project follows the principle of:

```text
Safe Rust First
        ↓
Avoid Unsafe Whenever Possible
        ↓
Require Justification When Necessary
        ↓
Review Unsafe Implementations Carefully
```

### Unsafe Code Guidelines

Unsafe code:

* MUST be justified.
* SHOULD remain minimal.
* SHOULD include appropriate documentation.
* SHOULD include comprehensive tests.

Unsafe implementations SHOULD explain:

* Why unsafe code is necessary.
* What guarantees are being upheld.
* What invariants must be preserved.

Safe abstractions should always be preferred whenever practical.

---

## API Security

Public APIs SHOULD prioritize:

* Explicit behavior
* Type safety
* Secure defaults
* Comprehensive error handling
* Defensive validation

Examples include:

```text
Input Validation
        ↓
Configuration Validation
        ↓
Request Validation
        ↓
Error Handling
        ↓
Resource Management
```

Security considerations should be incorporated during API design rather than introduced later in development.

---

## Feature Flag Security

Feature flags are considered part of the public API surface.

Security-sensitive functionality SHOULD:

* Be explicitly enabled when appropriate.
* Remain well documented.
* Include appropriate test coverage.

Experimental functionality SHOULD NOT be assumed to provide the same stability guarantees as stable functionality.

---

## Supply Chain Security

Supply chain security is an important consideration for all releases.

Recommended validation includes:

```text
Dependency Reviews
        ↓
License Validation
        ↓
Security Auditing
        ↓
Workspace Validation
        ↓
Release Validation
```

The project may periodically update dependencies to address:

* Security vulnerabilities
* Maintenance concerns
* Ecosystem compatibility
* Performance improvements

---

## Continuous Integration Requirements

Security validation SHOULD include:

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
cargo audit
        ↓
cargo deny
        ↓
Release Validation
```

Security checks are considered mandatory release requirements.

---

## Authentication and Secrets

Contributors SHOULD NOT:

* Commit API keys.
* Commit access tokens.
* Commit credentials.
* Commit private certificates.
* Commit sensitive configuration values.

Sensitive information MUST NOT be committed to the repository.

Examples include:

```text
.env Files

API Keys

Access Tokens

Secrets

Private Certificates

Production Credentials
```

Environment variables should be preferred for sensitive configuration whenever possible.

---

## Security Releases

Security releases may be published independently of feature releases when necessary.

Examples include:

```text
Critical Vulnerability
        ↓
Immediate Patch Release


---------------------


Dependency Vulnerability
        ↓
Security Release


---------------------


High Severity Issue
        ↓
Expedited Release
```

Security fixes are prioritized appropriately based on severity and impact.

---

## Supported Tooling

Recommended security tooling includes:

```text
cargo-audit

cargo-deny

cargo-nextest

cargo-clippy

cargo-fmt
```

Additional tooling may be introduced as the project evolves.

---

## Contributor Responsibilities

Contributors are encouraged to:

* Review security implications of their changes.
* Include appropriate tests.
* Follow secure coding practices.
* Validate dependency additions carefully.
* Document security-sensitive behavior when necessary.

Security is a shared responsibility across the entire development process.

---

## Security Policy After v1.0.0

Beginning with `v1.0.0`:

* Security fixes will be prioritized.
* Public APIs will maintain stability guarantees.
* Critical vulnerabilities may warrant expedited patch releases.
* Security advisories may accompany relevant releases.

Supported stable releases will receive security updates in accordance with the project's maintenance policy.

---

## Final Notes

Security is not a single feature—it is an ongoing commitment throughout the design, implementation, testing and release processes of Jules-SDK.

Contributors are encouraged to prioritize:

* Memory safety
* Correctness
* Secure defaults
* Defensive programming
* Long-term maintainability
* Responsible disclosure practices

> Secure software begins with secure engineering practices.
