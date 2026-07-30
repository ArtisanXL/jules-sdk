# Support

> Thank you for using Jules-SDK. This document describes how to get help, report problems and contribute to the project's continued improvement.

Before requesting support, please consider checking:

* [README.md](README.md)
* [PROJECT_STATE.md](PROJECT_STATE.md)
* [CHANGELOG.md](CHANGELOG.md)
* [FEATURES.md](FEATURES.md)
* [TESTING.md](TESTING.md)
* [SECURITY.md](SECURITY.md)
* Existing GitHub Issues and Discussions

Many common questions are already addressed through the project's documentation.

---

## Getting Help

The preferred support channels are:

| Topic                      | Recommended Channel            |
| -------------------------- | ------------------------------ |
| Usage Questions            | GitHub Discussions             |
| Bug Reports                | GitHub Issues                  |
| Feature Requests           | GitHub Issues                  |
| Documentation Improvements | GitHub Issues                  |
| Security Vulnerabilities   | Private Security Report        |
| Contribution Questions     | GitHub Discussions             |
| Breaking Changes           | CHANGELOG.md and Release Notes |

Please use the most appropriate channel whenever possible.

---

## Before Opening an Issue

Before submitting an issue, please verify that:

* You are using a supported version.
* The issue has not already been reported.
* The documentation has been reviewed.
* The problem can be reproduced consistently.
* Relevant logs or error messages are available.

Providing a minimal reproducible example is highly encouraged.

---

## Bug Reports

Bug reports SHOULD include:

* Jules-SDK version
* Rust version
* Operating system
* Enabled feature flags
* Relevant error messages
* Reproduction steps
* Expected behavior
* Actual behavior

Examples of useful information include:

```text
Jules-SDK Version:
0.1.0

Rust Version:
1.90.0

Operating System:
Linux

Enabled Features:
streaming
tools

Steps to Reproduce:
1.
2.
3.

Expected Behavior:
...

Actual Behavior:
...
```

Well-structured bug reports significantly improve investigation and resolution times.

---

## Feature Requests

Feature requests SHOULD include:

* The problem being solved.
* The proposed functionality.
* Potential use cases.
* Alternative approaches considered.
* Relevant examples when appropriate.

Helpful questions include:

* Why is this feature needed?
* Does it affect public APIs?
* Should it be feature gated?
* Is it platform specific?
* Does it introduce additional dependencies?

---

## Documentation Improvements

Documentation contributions are always welcome.

Examples include:

* Typographical corrections
* Improved explanations
* Additional examples
* Missing documentation
* API clarification
* Migration guidance

Good documentation is considered a first-class contribution to the project.

---

## Security Issues

> **Please do NOT disclose security vulnerabilities publicly.**

Security vulnerabilities SHOULD NOT be reported through:

* Public GitHub Issues
* Pull Requests
* Discussions
* Social media platforms

Please follow the reporting guidelines described in:

```text
SECURITY.md
```

Responsible disclosure practices are greatly appreciated.

---

## Contribution Questions

For contribution-related questions, contributors are encouraged to review:

* [CONTRIBUTING.md](CONTRIBUTING.md)
* [AGENTS.md](AGENTS.md)
* [PROJECT_STATE.md](PROJECT_STATE.md)
* [TESTING.md](TESTING.md)
* [VERSIONING.md](VERSIONING.md)
* [RELEASE.md](RELEASE.md)
* [MSRV.md](MSRV.md)

These documents describe the project's expectations regarding:

* Coding standards
* Testing requirements
* Release policies
* Versioning policies
* MSRV requirements

---

## Supported Versions

Only actively maintained releases are supported.

| Version            | Status      |
| ------------------ | ----------- |
| 1.x                | Supported   |
| Latest 0.x Release | Supported   |
| Older 0.x Releases | Unsupported |
| Archived Releases  | Unsupported |

Please consider upgrading before requesting support for unsupported versions.

---

## What is Supported?

Examples of supported topics include:

* Installation issues
* API usage questions
* Bug reports
* Documentation issues
* Feature requests
* Compilation failures
* Feature flag questions
* Platform compatibility issues

Maintainers will make reasonable efforts to assist supported requests.

---

## What is NOT Supported?

Examples include:

* Unsupported releases
* Modified third-party forks
* Proprietary integrations that cannot be reproduced
* Requests unrelated to Jules-SDK
* Requests lacking sufficient information for investigation

The maintainers may request additional information when necessary.

---

## Experimental Features

Experimental functionality:

```text
experimental

↓

Preview APIs

↓

Prototype Implementations

↓

Unstable Behavior
```

Experimental features:

* Are not considered production-ready.
* May change without notice.
* May receive limited support.
* Are exempt from stability guarantees.

Please include the `experimental` feature flag when reporting issues related to experimental functionality.

---

## Platform Support

Support is provided on a best-effort basis for officially supported platforms.

Examples include:

* Linux
* Windows
* macOS
* WASM
* Cloudflare Workers (when officially supported)

Platform-specific limitations may apply depending on enabled feature flags.

---

## Response Expectations

Support requests are reviewed on a best-effort basis.

Response times may vary depending on:

* Severity
* Complexity
* Maintainer availability
* Reproducibility
* Project priorities

Providing clear and complete information greatly improves the likelihood of timely assistance.

---

## Release Support Policy

Beginning with `v1.0.0`:

* Stable releases will receive prioritized support.
* Security issues will be prioritized appropriately.
* Critical regressions may warrant expedited patch releases.
* Breaking changes will follow the project's versioning policy.

For additional information, please refer to:

* VERSIONING.md
* RELEASE.md
* CHANGELOG.md

---

## Contributor Responsibilities

Contributors are encouraged to:

* Write clear issue reports.
* Include reproducible examples.
* Follow contribution guidelines.
* Review relevant documentation.
* Provide constructive feedback.

Maintaining a healthy and welcoming community is a shared responsibility.

---

## Final Notes

Jules-SDK is an open source project built around correctness, maintainability and developer experience. Community feedback, bug reports and contributions are highly valued and help improve the project for everyone.

Thank you for taking the time to:

* Report bugs
* Improve documentation
* Suggest new ideas
* Review contributions
* Support the project's development

> Clear communication and well-documented issues help build better software.
