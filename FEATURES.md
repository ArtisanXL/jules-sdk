# Feature Flags

> Jules-SDK adopts a feature flag driven architecture to provide a flexible, modular and production-ready developer experience.

Feature flags allow users to include only the functionality required by their applications while minimizing compile times, binary sizes and unnecessary dependencies.

---

## Design Philosophy

Feature flags are designed around the following principles:

* Minimal default dependencies
* Explicit opt-in functionality
* Production-ready defaults
* Platform compatibility
* Long-term maintainability
* Backwards compatibility after `v1.0.0`

The project intentionally favors modularity over monolithic functionality.

---

## Default Features

The default feature set provides the recommended experience for most users.

```toml
[features]

default = [
    "streaming",
    "tools",
]
```

### Included Functionality

| Feature   | Description           |
| --------- | --------------------- |
| streaming | Streaming API support |
| tools     | Tool calling support  |

Default features are expected to remain production-ready and well supported.

---

## Available Feature Flags

### streaming

```toml
streaming = []
```

Provides:

* Streaming APIs
* Stream event handling
* Async stream abstractions
* Incremental responses

Recommended for:

* Interactive applications
* Real-time integrations
* Long-running operations

---

### tools

```toml
tools = []
```

Provides:

* Tool calling support
* Tool abstractions
* Tool execution APIs
* Tool response handling

Recommended for:

* AI applications
* Automation workflows
* External integrations

---

### middleware

```toml
middleware = []
```

Provides:

* Request middleware
* Response middleware
* Middleware composition
* Request interception

Recommended for:

* Logging
* Monitoring
* Authentication
* Custom request handling

---

### telemetry

```toml
telemetry = []
```

Provides:

* Metrics support
* Tracing integrations
* Diagnostics capabilities
* Performance instrumentation

Recommended for:

* Production environments
* Monitoring systems
* Performance analysis

Telemetry functionality SHOULD avoid collecting sensitive information by default.

---

### cli

```toml
cli = []
```

Provides:

* Official CLI support
* Interactive commands
* Configuration management
* Diagnostic tooling

Recommended for:

* Development workflows
* Local experimentation
* Automation scripts

---

### wasm

```toml
wasm = []
```

Provides:

* WebAssembly support
* Browser compatibility
* WASM-specific abstractions

Recommended for:

* Browser environments
* Edge computing
* WASM deployments

Platform-specific limitations may apply.

---

### experimental

```toml
experimental = []
```

Provides:

* Experimental functionality
* Preview APIs
* Prototype implementations

> **Warning:** Experimental features are NOT considered stable and MAY change without notice.

Experimental functionality:

* Is not covered by stability guarantees.
* May be redesigned or removed.
* Should not be relied upon in production environments unless explicitly documented otherwise.

---

## Future Feature Flags

The following feature flags are planned but not guaranteed.

### Cloudflare Workers

```toml
cloudflare-workers = []
```

Potential functionality includes:

* Workers compatibility improvements
* Platform integrations
* Environment-specific abstractions

---

### GitHub Tools

```toml
github-tools = []
```

Potential functionality includes:

* GitHub integrations
* Repository tooling
* Workflow automation

---

### Filesystem Tools

```toml
filesystem-tools = []
```

Potential functionality includes:

* Filesystem abstractions
* File manipulation tools
* Local development integrations

---

### Browser Tools

```toml
browser-tools = []
```

Potential functionality includes:

* Browser-specific tooling
* Web integrations
* Frontend compatibility improvements

Future feature flags remain subject to architectural and maintenance considerations.

---

## Recommended Configurations

### Minimal Configuration

```toml
[dependencies]
jules-sdk = { version = "0.1", default-features = false }
```

Suitable for:

* Minimal environments
* Custom integrations
* Advanced use cases

---

### Default Configuration

```toml
[dependencies]
jules-sdk = "0.1"
```

Suitable for:

* Most applications
* Production environments
* General development

---

### Streaming Only

```toml
[dependencies]
jules-sdk = { version = "0.1", default-features = false, features = [
    "streaming",
] }
```

---

### Tools Only

```toml
[dependencies]
jules-sdk = { version = "0.1", default-features = false, features = [
    "tools",
] }
```

---

### Full Configuration

```toml
[dependencies]
jules-sdk = { version = "0.1", features = [
    "middleware",
    "telemetry",
    "cli",
    "wasm",
] }
```

---

## Feature Flag Compatibility

| Feature      | Production Ready | Stable After v1.0 | Optional |
| ------------ | ---------------- | ----------------- | -------- |
| streaming    | Yes              | Yes               | Yes      |
| tools        | Yes              | Yes               | Yes      |
| middleware   | Yes              | Yes               | Yes      |
| telemetry    | Yes              | Yes               | Yes      |
| cli          | Yes              | Yes               | Yes      |
| wasm         | Yes              | Yes               | Yes      |
| experimental | No               | No                | Yes      |

Experimental functionality is intentionally excluded from stability guarantees.

---

## Testing Policy

The following configurations SHOULD be validated continuously:

```bash
cargo check --no-default-features
```

```bash
cargo check --all-features
```

```bash
cargo test --all-features
```

```bash
cargo nextest run
```

Feature flag combinations SHOULD be tested whenever practical.

---

## Public API Considerations

Beginning with `v1.0.0`:

* Feature flags are considered part of the public API.
* Removing stable feature flags is considered a breaking change.
* Changes to existing behavior MUST follow Semantic Versioning.

Examples include:

```text
Allowed

↓

1.1.0

↓

Adding New Features


---------------------


NOT ALLOWED

↓

1.1.0

↓

Removing Existing Features


---------------------


Allowed

↓

2.0.0

↓

Breaking Changes
```

For additional details, please refer to:

* VERSIONING.md
* RELEASE.md

---

## Platform Support

Some feature flags may be platform dependent.

Examples include:

| Feature    | Linux | Windows | macOS | WASM    |
| ---------- | ----- | ------- | ----- | ------- |
| streaming  | Yes   | Yes     | Yes   | Planned |
| tools      | Yes   | Yes     | Yes   | Planned |
| middleware | Yes   | Yes     | Yes   | Planned |
| telemetry  | Yes   | Yes     | Yes   | Planned |
| cli        | Yes   | Yes     | Yes   | No      |
| wasm       | No    | No      | No    | Yes     |

Platform support may evolve as the project matures.

---

## Contributor Guidelines

Before introducing a new feature flag, contributors SHOULD consider:

* Is a new feature flag necessary?
* Can existing functionality be extended?
* Does it increase maintenance costs?
* Does it introduce additional dependencies?
* Is the functionality sufficiently modular?

Feature flags SHOULD remain:

* Well documented
* Well tested
* Clearly scoped
* Easy to understand

Large and unrelated functionality SHOULD NOT be grouped under a single feature flag.

---

## Stability Guarantees

### Stable Features

Stable features provide:

* Semantic Versioning guarantees
* Documentation support
* Long-term maintenance

### Experimental Features

Experimental features:

* May change without notice.
* May be removed without deprecation.
* Are not considered production-ready.
* Are exempt from stability guarantees.

Users should opt into experimental functionality consciously.

---

## Final Notes

Feature flags are intended to provide a flexible and modular development experience while preserving the project's commitment to stability, performance and maintainability.

Contributors are encouraged to prioritize:

* Simplicity
* Explicitness
* Stability
* Documentation
* Maintainability
* Platform compatibility

> Small, composable and well-documented features scale better than monolithic designs.
