# Versioning Policy

> Jules-SDK follows Semantic Versioning (SemVer) with an iterative `0.x` development lifecycle and API stability guarantees beginning with `v1.0.0`.

---

## Overview

Jules-SDK adopts the Rust ecosystem philosophy of releasing early, iterating during development and committing to long-term API stability only when the project reaches `v1.0.0`.

The project's versioning strategy is designed to balance:

* Rapid iteration
* API ergonomics
* Long-term maintainability
* Production readiness
* Backwards compatibility

---

## Semantic Versioning

Jules-SDK follows the Semantic Versioning specification:

```text
MAJOR.MINOR.PATCH

1.2.3

│ │ └──── Bug fixes
│ └────── New features
└──────── Breaking changes
```

### Examples

```text
1.0.0

↓

Initial Stable Release


---------------------


1.1.0

↓

New Features


---------------------


1.1.1

↓

Bug Fixes


---------------------


2.0.0

↓

Breaking Changes
```

---

## Development Lifecycle

```text
v0.1.0
↓

Initial Release


↓

v0.2.0

↓

Sessions & Conversations


↓

v0.3.0

↓

Streaming APIs


↓

v0.4.0

↓

Tool Calling


↓

v0.5.0

↓

Official CLI


↓

v0.6.0

↓

Proc Macros


↓

v0.7.0

↓

Middleware System


↓

v0.8.0

↓

WASM Support


↓

v0.9.0

↓

API Freeze Candidate


↓

v1.0.0

↓

Stable Release
```

---

## Versioning Policy Before v1.0.0

During the `0.x` lifecycle:

* Breaking changes are permitted.
* Public APIs may evolve significantly.
* Internal architecture may change when justified.
* Feature flags may be added, removed or redesigned.

### Allowed Changes

```text
Public API Changes

↓

Allowed


------------------


Builder Improvements

↓

Allowed


------------------


Module Refactoring

↓

Allowed


------------------


Internal Changes

↓

Allowed
```

### Example

```text
v0.2.0

Client::create()


↓

v0.3.0

Client::builder()


↓

Breaking Change

↓

Allowed
```

Contributors should expect APIs to evolve during the `0.x` development cycle.

---

## Versioning Policy After v1.0.0

Beginning with `v1.0.0`, Jules-SDK guarantees public API stability in accordance with Semantic Versioning.

### Allowed Changes

```text
1.1.0

↓

New Features


------------------


1.1.1

↓

Bug Fixes


------------------


1.2.0

↓

Additional APIs


------------------


2.0.0

↓

Breaking Changes
```

Breaking changes MUST NOT be introduced in:

```text
1.0.x

1.1.x

1.2.x
```

Breaking changes MUST ONLY be introduced in:

```text
2.0.0

3.0.0

4.0.0
```

---

## Public API Stability

The following APIs are considered public:

* Public structs
* Public enums
* Public traits
* Public functions
* Public macros
* Public feature flags
* Public modules
* Public builders

Examples include:

```rust
use jules_sdk::Client;
use jules_sdk::Session;
use jules_sdk::Conversation;
use jules_sdk::Tool;
```

Changes affecting public APIs are subject to this versioning policy.

---

## Internal APIs

The following components are considered internal implementation details:

```text
jules-core

jules-api

jules-macros

Internal Modules

Private Traits

Private Utilities
```

Internal implementations may evolve independently provided that:

* Public APIs remain compatible.
* Documentation remains accurate.
* Existing guarantees are preserved.

---

## Feature Flags

### Before v1.0.0

Feature flags may evolve freely.

```text
streaming

↓

Allowed Changes


------------------


tools

↓

Allowed Changes


------------------


experimental

↓

Allowed Changes
```

### After v1.0.0

Feature flag changes are considered public API changes.

#### Allowed

```text
Adding New Features

↓

Minor Releases


------------------


Adding New Feature Flags

↓

Minor Releases
```

#### Not Allowed

```text
Removing Existing Features

↓

Major Releases Only


------------------


Changing Existing Behavior

↓

Major Releases Only
```

---

## Deprecation Policy

Whenever possible, public APIs should be deprecated before removal.

### Preferred Approach

```text
v1.1.0

↓

Deprecation Warning


↓

v1.2.0

↓

Migration Guide


↓

v2.0.0

↓

Removal
```

This approach allows users sufficient time to migrate their code.

---

## MSRV Policy

Jules-SDK currently supports:

```text
Rust 1.90+
```

### Before v1.0.0

MSRV changes are permitted when justified.

### After v1.0.0

Changes to the Minimum Supported Rust Version are considered breaking changes and SHOULD be introduced only through major releases whenever possible.

Examples:

```text
1.0.0

↓

Rust 1.90


↓

2.0.0

↓

Rust 1.94


↓

Allowed
```

---

## Documentation Requirements

The following changes MUST include documentation updates:

* Breaking changes
* Public API changes
* Feature additions
* Deprecations
* Migration requirements

Documentation updates may include:

```text
CHANGELOG.md

README.md

ARCHITECTURE.md

Migration Guides

Examples

Rustdoc
```

---

## Changelog Policy

All releases MUST be documented in:

```text
CHANGELOG.md
```

Release notes should include:

* Added
* Changed
* Deprecated
* Removed
* Fixed
* Security

### Example

```text
v1.1.0

Added
------
- Streaming improvements

Changed
--------
- Session builders

Deprecated
-----------
- Legacy APIs

Fixed
------
- Authentication bugs

Security
---------
- Dependency updates
```

---

## Experimental Features

Experimental APIs are exempt from stability guarantees.

Examples include:

```text
experimental

nightly

unstable
```

Experimental functionality:

* May change without notice.
* May be removed without deprecation.
* Is not considered production-ready.

Experimental features should always be clearly documented.

---

## Release Categories

| Version       | Purpose                 |
| ------------- | ----------------------- |
| 0.1.0         | Initial Release         |
| 0.2.0 - 0.8.0 | Active Development      |
| 0.9.0         | API Freeze Candidate    |
| 1.0.0         | Stable Release          |
| 1.x.y         | Stable Feature Releases |
| 2.0.0         | Breaking Changes        |

---

## Contributor Expectations

Before introducing:

* New public APIs
* Breaking changes
* New feature flags
* Major refactors
* MSRV changes

Contributors should review:

```text
ROADMAP.md

ARCHITECTURE.md

CONTRIBUTING.md

CHANGELOG.md
```

All changes affecting the public API should be carefully evaluated for backwards compatibility and long-term maintainability.

---

## Stability Guarantees

### Before v1.0.0

```text
API Stability

↓

NOT GUARANTEED
```

### After v1.0.0

```text
API Stability

↓

GUARANTEED

↓

Unless a new major version is released.
```

---

## Final Notes

Jules-SDK intentionally embraces rapid iteration during the `0.x` lifecycle while maintaining a strong commitment to API stability after `v1.0.0`.

Breaking changes are acceptable during early development when they improve the project's design, ergonomics or maintainability. Once the project reaches `v1.0.0`, public APIs will evolve conservatively and in accordance with Semantic Versioning principles.

> Release early. Iterate responsibly. Stabilize deliberately.
