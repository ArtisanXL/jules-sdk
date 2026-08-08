# Project State

> This document describes the current development status, priorities and milestones of Jules-SDK. Contributors and AI coding agents SHOULD review this document before making significant changes to the repository.
>
Last Updated: 2026-08-08

---

## Status Legend

| Symbol | Meaning                        |
| ------ | ------------------------------ |
| ⬜      | Not started                    |
| 🟨      | In progress                    |
| ✅      | Done                           |
| 🚫      | Blocked                        |
| ⏸️      | Paused / deprioritized         |

---

## Current Status

| Field              | Value                              |
| ------------------ | ------------------------------------ |
| Project Name        | Jules-SDK                           |
| Current Version      | v0.1.0-dev                          |
| Development Stage     | Pre-Alpha                           |
| Current Phase        | Phase 0 — Repository Foundation      |
| Pending Work         | 0 parent tasks / 0 subtasks remaining in the phased breakdown (see Completed Tasks Log) |
| Status              | 🟨 In Progress                       |
| MSRV                | Rust 1.90+                          |
| Workspace Status      | ✅ Complete |
| Last Updated         | 2026-08-08                          |

---

## Current Milestone

> Establish the project's foundations before implementing public APIs.

### Milestone Goals — Progress: 8 / 8 (100%)

| ID     | Goal                          | Status |
| ------ | ------------------------------- | ------ |
| MG-01  | Production-ready repository structure | ✅ |
| MG-02  | Cargo workspace initialization     | ✅ |
| MG-03  | Documentation foundations        | ✅ |
| MG-04  | CI/CD foundations              | ✅ |
| MG-05  | Testing infrastructure          | ✅ |
| MG-06  | Release management policies      | ✅ |
| MG-07  | Security policies              | ✅ |
| MG-08  | AI development guidelines        | ✅ |

---

## Documentation — Progress: 14 / 14 (100%) ✅

| ID     | Document                             | Status |
| ------ | -------------------------------------- | ------ |
| DOC-01 | [README.md](README.md)                 | ✅ |
| DOC-02 | [ROADMAP.md](ROADMAP.md)               | ✅ |
| DOC-03 | [ARCHITECTURE.md](ARCHITECTURE.md)       | ✅ |
| DOC-04 | [CONTRIBUTING.md](CONTRIBUTING.md)       | ✅ |
| DOC-05 | [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) | ✅ |
| DOC-06 | [VERSIONING.md](VERSIONING.md)          | ✅ |
| DOC-07 | [RELEASE.md](RELEASE.md)               | ✅ |
| DOC-08 | [TESTING.md](TESTING.md)               | ✅ |
| DOC-09 | [CHANGELOG.md](CHANGELOG.md)            | ✅ |
| DOC-10 | [SECURITY.md](SECURITY.md)             | ✅ |
| DOC-11 | [FEATURES.md](FEATURES.md)             | ✅ |
| DOC-12 | [SUPPORT.md](SUPPORT.md)               | ✅ |
| DOC-13 | [MSRV.md](MSRV.md)                    | ✅ |
| DOC-14 | [AGENTS.md](AGENTS.md)                 | ✅ |

## Repository Policies — Progress: 5 / 5 (100%) ✅

| ID     | Policy                        | Status |
| ------ | -------------------------------- | ------ |
| POL-01 | License policies established      | ✅ |
| POL-02 | Versioning policies established    | ✅ |
| POL-03 | Testing policies established      | ✅ |
| POL-04 | Security policies established     | ✅ |
| POL-05 | Release policies established      | ✅ |

---

## Workspace — Progress: 4 / 4 (100%)

| ID    | Task                        | Status | Notes |
| ----- | ------------------------------ | ------ | ----- |
| WS-01 | Cargo workspace initialization   | ✅ | — |
| WS-02 | Crate organization             | ✅ | Broken down below |
| WS-03 | Feature flag implementation      | ✅ | Also tracked as TASK-03 |
| WS-04 | CI pipeline implementation       | ✅ | — |

## Crates — Progress: In Progress (0 / 5 fully implemented)

| ID    | Crate         | Status      |
| ----- | -------------- | ----------- |
| CR-01 | jules-sdk      | 🟨 In Progress |
| CR-02 | jules-core     | 🟨 In Progress |
| CR-03 | jules-api      | 🟨 In Progress |
| CR-04 | jules-macros   | 🟨 In Progress |
| CR-05 | jules-cli      | 🟨 In Progress |

Crate statuses should be updated whenever implementation milestones are completed.

---

## Pending Work — Phased & Ordered Breakdown

> This is the authoritative, sequential to-do list. Parent tasks are grouped by phase, ordered by intended execution sequence, and broken into smaller subtasks for finer-grained tracking. Work top-to-bottom, phase-by-phase. When every subtask under a parent is ✅, mark the parent done and move it to the Completed Tasks Log.

### Phase 0 — Repository Foundation (2 parent tasks / 13 subtasks)

**Order 1 — `WS-02` Crate organization** — High priority

| ID       | Subtask                                                              | Status |
| -------- | ----------------------------------------------------------------------- | ------ |
| WS-02.1  | Define crate directory layout (jules-sdk, jules-core, jules-api, jules-macros, jules-cli) | ✅ |
| WS-02.2  | Add workspace members to root `Cargo.toml`                                 | ✅ |
| WS-02.3  | Document per-crate ownership/responsibility boundaries in ARCHITECTURE.md      | ✅ |
| WS-02.4  | Define inter-crate dependency graph (which crate depends on which)           | ✅ |
| WS-02.5  | Add placeholder `lib.rs`/`main.rs` with crate-level doc comment per crate     | ✅ |

**Order 2 — `WS-04` CI pipeline implementation** — High priority ⚠️ reconcile with TASK-04 first

| ID       | Subtask                                                | Status |
| -------- | ---------------------------------------------------------- | ------ |
| WS-04.1  | Choose CI provider and workflow file location (e.g. GitHub Actions) | ✅ |
| WS-04.2  | Add build workflow (`cargo build --workspace`)                | ✅ |
| WS-04.3  | Add test workflow (`cargo test --workspace`)                 | ✅ |
| WS-04.4  | Add lint workflow (`cargo clippy -- -D warnings`)             | ✅ |
| WS-04.5  | Add format-check workflow (`cargo fmt --check`)               | ✅ |
| WS-04.6  | Add MSRV check workflow (Rust 1.90+)                       | ✅ |
| WS-04.7  | Add dependency/build caching                              | ✅ |
| WS-04.8  | Reconcile this task's status against TASK-04 and remove the flag | ✅ |

### Phase 1 — Core Abstractions (2 parent tasks / 10 subtasks)

**Order 3 — `TASK-14` Implement core Client traits in jules-core** — High priority

| ID         | Subtask                                                    | Status |
| ---------- | -------------------------------------------------------------- | ------ |
| TASK-14.1  | Define `Client` trait signature (async methods, associated types)  | ✅ |
| TASK-14.2  | Define request/response model types used by the trait           | ✅ |
| TASK-14.3  | Define/reuse error type for client operations (see TASK-08)       | ✅ |
| TASK-14.4  | Add doc comments and trait-level documentation                  | ✅ |
| TASK-14.5  | Write unit tests / mock implementation of the trait               | ✅ |

**Order 4 — `TASK-15` Implement ClientBuilder in jules-core** — High priority, depends on TASK-14

| ID         | Subtask                                                        | Status |
| ---------- | ------------------------------------------------------------------ | ------ |
| TASK-15.1  | Define `ClientBuilder` struct (base URL, timeout, auth, etc.)        | ✅ |
| TASK-15.2  | Implement setter methods (builder pattern)                       | ✅ |
| TASK-15.3  | Implement validation logic in `build()`                          | ✅ |
| TASK-15.4  | Wire builder output to the `Client` trait (TASK-14)                | ✅ |
| TASK-15.5  | Write unit tests for defaults and validation errors                | ✅ |

### Phase 2 — API Integrations & Streaming (4 parent tasks / 17 subtasks)

**Order 5 — `PH2-01` API integrations** — Medium priority

| ID        | Subtask                                          | Status |
| --------- | ----------------------------------------------------- | ------ |
| PH2-01.1  | Define HTTP transport layer abstraction                  | ✅ |
| PH2-01.2  | Implement authentication handling (API keys/tokens)         | ✅ |
| PH2-01.3  | Implement endpoint request construction                  | ✅ |
| PH2-01.4  | Implement response deserialization and error mapping        | ✅ |
| PH2-01.5  | Add rate-limit / retry handling                          | ✅ |

**Order 6 — `PH2-02` Streaming support** — Medium priority

| ID        | Subtask                                      | Status |
| --------- | -------------------------------------------------- | ------ |
| PH2-02.1  | Define stream abstraction for incremental responses    | ✅ |
| PH2-02.2  | Implement SSE (server-sent events) parsing              | ✅ |
| PH2-02.3  | Implement backpressure / chunk buffering                | ✅ |
| PH2-02.4  | Implement stream error handling and reconnection          | ✅ |
| PH2-02.5  | Write streaming integration tests                       | ✅ |

**Order 7 — `PH2-03` Builders (general, beyond ClientBuilder)** — Medium priority

| ID        | Subtask                                          | Status |
| --------- | ----------------------------------------------------- | ------ |
| PH2-03.1  | Implement `RequestBuilder`                              | ✅ |
| PH2-03.2  | Implement `ConversationBuilder`                          | ✅ |
| PH2-03.3  | Implement `MessageBuilder`                              | ✅ |
| PH2-03.4  | Ensure builders integrate cleanly with `ClientBuilder`      | ✅ |


**Order 5.5 — `PH2-04` REST API Structure Alignment (v1alpha)** — High priority

| ID        | Subtask                                                              | Status | Notes |
| --------- | ---------------------------------------------------------------------- | ------ | ----- |
| PH2-04.1  | Define API resource models (Sessions, Activities, Sources) in jules-core | ✅ | Reworked 2026-08-08 — the prior "done" mark was false (see note below); models now carry real fields and serde derives verified against live payloads. |
| PH2-04.2  | Implement `v1alpha/sessions` endpoints (create, get, list, approvePlan, sendMessage) | ✅ | All five confirmed working against the live API (2026-08-08). `create_session`/`list_sessions`/`get_session` verified directly. `send_message` initially appeared broken (404) but was proven correct — the 404 only happens if called while the session is still `QUEUED`; once past that (e.g. `AWAITING_USER_FEEDBACK`), it returns HTTP 200. `approve_plan` had a real bug: our code sent Rust's `()` (serializes to JSON `null`), but the API requires an empty object `{}` — fixed in [PR #79](https://github.com/ArtisanXL/jules-sdk/pull/79) (`ApprovePlanRequest {}`), independently re-verified in a clean worktree (build/test/clippy/fmt all green) before merge. |
| PH2-04.3  | Implement `v1alpha/sessions.activities` and `v1alpha/sources` endpoints  | ✅ | `list_activities` and `list_sources` implemented and verified against the live API (2026-08-08), including pagination (`nextPageToken`). |
| PH2-04.4  | Configure Base URL (`https://jules.googleapis.com`) and Google Auth handling | ✅ | `JULES_API_BASE_URL` is the `JulesClientBuilder` default; `AuthType::google_api_key()` sends the `X-Goog-Api-Key` header. Both confirmed working against the live API (2026-08-08) — no OAuth/service-account flow was needed or implemented, the real API accepts a plain API key via this header. |

PH2-04 is now fully done (all four subtasks ✅) — see the PH2-04 Completed Tasks Log entry.

⚠️ **Live artifacts created 2026-08-08** (informational, not action items): two real Jules sessions were created against the `ArtisanXL/jules-sdk` GitHub source while testing `create_session`: `sessions/13267831108462964551` ("jules-sdk SDK integration test", last observed `QUEUED`, harmless no-op prompt, not cleaned up — this SDK has no `delete_session`/cancel endpoint) and `sessions/10348936599420094322` (the session actually delegated the `approve_plan`/`send_message` bug fix to — reached `COMPLETED` and produced [PR #79](https://github.com/ArtisanXL/jules-sdk/pull/79), which was reviewed independently and merged).

Note (2026-08-08, earlier): A real, generic, tested native HTTP transport (`ReqwestTransport`) and an end-to-end `JulesClient`/`JulesClientBuilder` were added in `jules-api`, verified against local test servers.

Note (2026-08-08, later): PH2-04.1's prior "✅" was also false — `Session`/`Source`/`Activity` were id+name-only stubs with zero serde derives, and `jules-core/src/pagination/mod.rs` was completely empty, despite being marked done. All four were reworked with real fields matching live API response payloads (captured via a working test API key, see `.secrets/jules_api_key.env`, gitignored) and re-verified end-to-end: `list_sessions`, `get_session`, `list_sources`, and `list_activities` were each called for real against `https://jules.googleapis.com` through the actual compiled `jules-api` client code (not curl) and correctly deserialized live responses. `Activity` additionally carries `#[serde(flatten)] extra` to avoid silently dropping activity kinds not seen in the sampled data (only `planGenerated` was observed). `create_session`/`send_message`/`approve_plan` remain unverified by design — do not mark PH2-04.2 fully done until one of them is confirmed against the live API.

### Phase 3 — Tooling & CLI (4 parent tasks / 19 subtasks)

**Order 8 — `PH3-01` Tool calling support** — Medium priority

| ID        | Subtask                                              | Status |
| --------- | ----------------------------------------------------------- | ------ |
| PH3-01.1  | Define `Tool` trait                                          | ✅ |
| PH3-01.2  | Implement tool registry / registration mechanism                 | ✅ |
| PH3-01.3  | Implement tool-call parsing from model responses                 | ✅ |
| PH3-01.4  | Implement tool result serialization back into conversation          | ✅ |
| PH3-01.5  | Write tool-calling integration tests                             | ✅ |

**Order 9 — `PH3-02` Middleware support** — Medium priority

| ID        | Subtask                                | Status | Notes |
| --------- | --------------------------------------------- | ------ | ----- |
| PH3-02.1  | Define `Middleware` trait                        | ✅ | — |
| PH3-02.2  | Implement middleware chaining/execution pipeline    | ✅ | — |
| PH3-02.3  | Implement built-in logging middleware              | ✅ | — |
| PH3-02.4  | Implement built-in retry middleware                | ✅ | `RetryMiddleware::execute()` now loops and retries retriable errors with backoff. Proven by `test_retry_middleware_retries_on_retriable_error` (jules-core/src/middleware/retry.rs), which fails a handler with a retriable HTTP 500 twice and asserts the handler was actually invoked 3 times (`call_count == 3`) before succeeding. Confirmed 2026-08-08. |
| PH3-02.5  | Write middleware tests                            | ✅ | — |

**Order 10 — `PH3-03` CLI support** — Medium priority

| ID        | Subtask                                    | Status | Notes |
| --------- | ------------------------------------------------ | ------ | ----- |
| PH3-03.1  | Set up `jules-cli` argument parsing (e.g. clap)       | ✅ | Implemented 2026-08-08: clap-derive `Cli`/`Commands` in `main.rs`, wired to the real `JulesClient`/`JulesClientBuilder` (jules-api, `middleware` feature). |
| PH3-03.2  | Implement core subcommands (e.g. chat, config)         | ✅ | Implemented 2026-08-08: `config show`/`config set`, `chat <message>` (creates a session or, with `--session`, sends a message to an existing one), `sessions list`/`sessions get`, `sources list`. |
| PH3-03.3  | Implement config file loading/overrides                | ✅ | Implemented 2026-08-08: `CliConfig` loaded from a JSON file (via `dirs::config_dir()`) with CLI-flag > `JULES_API_KEY`/`JULES_BASE_URL` env var > file > default precedence. |
| PH3-03.4  | Implement output formatting (plain/JSON)               | ✅ | Implemented 2026-08-08: `OutputFormat` (Plain/Json) selectable via global `--format` flag. |
| PH3-03.5  | Write CLI smoke tests                                 | ✅ | Implemented 2026-08-08: 26 tests, including handler-level tests against the real `JulesClient` pointed at a local mock HTTP server (`test_support::MockServer`, mirroring `jules-api`'s own `tests/client_e2e_test.rs` pattern) — not just arg-parsing checks. Manually live-verified (read-only: `sessions list`, `sessions get`, `sources list`) against the real API; trivial `assert_eq!(2 + 2, 4)` placeholder removed. |

**Order 11 — `PH3-04` Additional examples** — Low priority (initial examples already done via TASK-09)

| ID        | Subtask                        | Status |
| --------- | ------------------------------------ | ------ |
| PH3-04.1  | Add streaming example                  | ✅ |
| PH3-04.2  | Add tool-calling example                | ✅ |
| PH3-04.3  | Add middleware example                 | ✅ |
| PH3-04.4  | Add CLI usage example                  | ✅ |

### Phase 4 — Platform & Performance (1 parent tasks / 4 subtasks)


**Order 13 — `PH4-04` Stability validations** — Low priority

| ID        | Subtask                                       | Status |
| --------- | --------------------------------------------------- | ------ |
| PH4-04.1  | Run fuzz testing on core parsing paths                 | ✅ |
| PH4-04.2  | Run load/soak testing on client                       | ✅ |
| PH4-04.3  | Review public API for stability guarantees               | ✅ |
| PH4-04.4  | Fix issues found and document results                   | ✅ |

✅ **Flag resolved (2026-08-08):** `PH4-01` (WASM support) and `PH4-03` (Performance improvements), previously flagged as claimed-done-but-unverified via TASK-10/TASK-12, have both now been verified:
- `PH4-03` (performance): confirmed legitimate — 8 real commits with substantive diffs to production code (SSE parser allocations, streaming buffer pre-allocation, middleware pipeline iteration, endpoint URL building, MockTransport/Conversation clone removal).
- `PH4-01` (WASM support): found genuinely incomplete — `FetchClient` (`crates/jules-api/src/wasm.rs`) did not implement the `Transport` trait at all, so WASM users could not use any of this crate's `v1alpha` endpoint methods. Fixing it surfaced a deeper issue: `Transport::send`'s `+ Send` bound is incompatible with `wasm32` types (`JsValue`/`web_sys` are not `Send`). Fixed for real: `Transport` is now defined with a `Send` bound on native targets and without it on `wasm32` (`crates/jules-api/src/http/mod.rs`), and `FetchClient` now fully implements it (method/headers/body/status/response-body all wired through the Fetch API). Verified with a real, executed `wasm-bindgen-test` run in headless Firefox exercising the full request/response round trip (including a custom header) via a `data:` URL, needing no live server. Also fixed a pre-existing, unrelated blocker found along the way: `cargo test --target wasm32-unknown-unknown` failed to compile at all because dev-only `tokio` features (`net`/`io-util`, added for the native TCP-socket e2e tests) don't support wasm32 — now target-gated to non-wasm32.

Also found and fixed during this review: `jules_sdk::Client`/`Conversation`/`Tool` are documented in VERSIONING.md's Public API Stability section as supported imports from the facade crate, but were not actually re-exported (only `Config`/`ConfigBuilder`, `Session`/`SessionBuilder`, and `JulesClient`/`JulesClientBuilder` were) — fixed in `crates/jules-sdk/src/lib.rs` (`Tool` gated behind the `tools` feature, matching `jules-core`'s own gating).

### Phase 5 — Release Preparation (4 parent tasks / 14 subtasks)

**Order 14 — `PH5-01` API freeze preparations** — Low priority

| ID        | Subtask                                       | Status |
| --------- | --------------------------------------------------- | ------ |
| PH5-01.1  | Audit full public API surface                          | ✅ |
| PH5-01.2  | Mark unstable/experimental items explicitly              | ✅ |
| PH5-01.3  | Finalize semver commitments in VERSIONING.md              | ✅ |

**Order 15 — `PH5-02` Compatibility validations** — Low priority

| ID        | Subtask                                       | Status |
| --------- | --------------------------------------------------- | ------ |
| PH5-02.1  | Validate MSRV compliance across workspace                | ✅ |
| PH5-02.2  | Run cross-platform CI matrix                          | ✅ |
| PH5-02.3  | Validate backward compatibility against v0.1.0 API          | ✅ |

**Order 16 — `PH5-03` Documentation finalization** — Low priority

| ID        | Subtask                          | Status |
| --------- | --------------------------------------- | ------ |
| PH5-03.1  | Review and update README.md               | ✅ |
| PH5-03.2  | Review and update ARCHITECTURE.md          | ✅ |
| PH5-03.3  | Finalize CHANGELOG.md entries              | ✅ |
| PH5-03.4  | Proofread all crate-level docs             | ✅ |

## Completed Tasks Log

| ID      | Phase   | Task                                          |
| ------- | ------- | ---------------------------------------------- |
| PH4-04  | Phase 4 | Stability validations — resolved the PH4-01/PH4-03 flag (WASM `Transport` fix, verified perf history), fixed the `jules_sdk::Client`/`Conversation`/`Tool` re-export gap |
| PH3-03  | Phase 3 | CLI support — `jules-cli` fully implemented and wired to the real `JulesClient` |
| PH2-04  | Phase 2 | REST API Structure Alignment (v1alpha) — real endpoints implemented and live-verified, including the `approve_plan`/`send_message` fix in PR #79 |
| PH3-02  | Phase 3 | Middleware support — retry middleware genuinely retries now |
| PH5-04  | Phase 5 | Release candidate preparations |
| PH3-04  | Phase 3 | Additional examples |
| PH3-01  | Phase 3 | Tool calling support |
| PH2-03  | Phase 2 | Builders (general, beyond ClientBuilder)       |
| PH2-02  | Phase 2 | Streaming support                              |
| PH2-01  | Phase 2 | API integrations                               |
| PH5-01  | Phase 5 | API freeze preparations |
| TASK-15 | Phase 1 | Implement ClientBuilder in jules-core          |
| TASK-14 | Phase 1 | Implement core Client traits in jules-core     |
| WS-04   | Phase 0 | CI pipeline implementation                     |
| WS-02   | Phase 0 | Crate organization                             |
| WS-01   | Phase 0 | Cargo workspace initialization                    |
| WS-03   | Phase 0 | Feature flag implementation                      |
| TASK-01 | Phase 0 | Initialize the Cargo workspace                    |
| TASK-02 | Phase 0 | Implement crate structure                        |
| TASK-03 | Phase 0 | Configure feature flags                          |
| TASK-04 | Phase 0 | Configure CI workflows                             |
| TASK-05 | Phase 0 | Establish testing pipelines                      |
| TASK-06 | Phase 1 | Implement SessionBuilder                         |
| TASK-07 | Phase 1 | Implement configuration management                 |
| TASK-08 | Phase 1 | Implement core error types                       |
| TASK-09 | Phase 3 | Add initial examples                            |
| TASK-13 | Phase 2 | Implement Message and Conversation models in jules-core |
| PH4-02  | Phase 4 | Platform integrations                          |
| TASK-10 | Phase 4 | WASM integrations        |
| TASK-11 | —       | Additional tooling support                       |
| TASK-12 | Phase 4 | Performance optimizations *(⚠️ needs verification)* |

---

## Supported Features

| Feature      | Status              |
| ------------- | --------------------- |
| —            | **Stable:** None       |
| streaming     | In Development         |
| tools        | In Development         |
| middleware    | In Development         |
| telemetry     | In Development         |
| cli          | In Development         |
| wasm         | In Development         |
| experimental   | Experimental           |

No feature should be considered production-ready until explicitly documented otherwise.

---

## Known Blockers

```text
None
```

When blockers exist, contributors SHOULD document them here with an ID (e.g. `BLK-01`), including:

* The affected functionality (linked task/subtask ID).
* The reason for the blocker.
* Potential mitigation strategies.
* Relevant dependencies.

| ID     | Affected ID | Reason | Mitigation | Status |
| ------ | ------------ | ------ | ----------- | ------ |
| —      | —            | —      | —           | —      |

---

## Breaking Changes

```text
None
```

Breaking changes SHOULD be logged here with an ID (e.g. `BC-01`) and include:

* Affected versions.
* Migration considerations.
* Related documentation updates.
* Relevant release notes.

| ID    | Affected Versions | Migration Notes | Docs Updated | Release Notes |
| ----- | ------------------ | ----------------- | -------------- | --------------- |
| —     | —                  | —                 | —              | —               |

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
* Work the **Pending Work — Phased & Ordered Breakdown** section top-to-bottom, subtask by subtask, unless a documented blocker requires reordering.
* Reference subtask IDs (e.g. `WS-02.2`) in commits/PRs so changes trace back to this document.
* Respect milestone priorities.
* Preserve architectural consistency.
* Avoid introducing undocumented functionality.
* Prefer incremental and well-tested changes — one subtask at a time.
* Mark a parent task done only once every one of its subtasks is ✅, then move it to the Completed Tasks Log.

AI coding agents MUST NOT:

* Introduce breaking changes silently.
* Ignore repository policies.
* Modify unrelated functionality unnecessarily.
* Claim successful validation without verification.
* Mark a subtask or parent task ✅ without corresponding verifiable work (commit, test, or PR reference).
* Skip ahead in the phased order without recording a reason.

When uncertainty exists, contributors SHOULD prioritize clarification over architectural assumptions.

---

## Release Targets

| Version | Status      |
| ------- | ----------- |
| v0.1.0  | ✅ Released (2026-08-08, [crates.io](https://crates.io/crates/jules-sdk), [GitHub release](https://github.com/ArtisanXL/jules-sdk/releases/tag/v0.1.0)) |
| v0.2.0  | ⬜ Planned     |
| v0.5.0  | ⬜ Planned     |
| v0.9.0  | ⬜ Planned     |
| v1.0.0  | ⬜ Planned     |

Release targets MAY evolve as development progresses.

---

## Change Log for This File

| Date       | Change                                                                                                  |
| ---------- | --------------------------------------------------------------------------------------------------------- |
| 2026-08-08 | Published v0.1.0: bumped workspace version from `0.1.0-rc.1` to `0.1.0`, finalized the CHANGELOG.md v0.1.0 entry, re-ran the full release checklist (fmt/clippy/build/test/audit/deny all clean), tagged `v0.1.0`, and published `jules-core`, `jules-macros`, `jules-api`, `jules-sdk`, and `jules-cli` to crates.io (all confirmed live via the crates.io API), plus a [GitHub release](https://github.com/ArtisanXL/jules-sdk/releases/tag/v0.1.0). |
| 2026-08-08 | Completed PH3-03 (CLI support): implemented `jules-cli` end to end — clap argument parsing, `config`/`chat`/`sessions`/`sources` subcommands wired to the real `JulesClient`, JSON config file with CLI-flag/env/file/default precedence, plain/JSON output formatting, and 26 tests (including handler-level tests against the real client via a local mock HTTP server). Manually live-verified read-only (`sessions list`, `sessions get`, `sources list`) against the real API. |
| 2026-08-08 | Resolved the PH4-01/PH4-03 flag-for-review (also closes PH4-04.3/.4): verified PH4-03 (performance) as legitimate; found PH4-01 (WASM support) genuinely incomplete (`FetchClient` didn't implement `Transport`; `Transport`'s `Send` bound conflicted with `wasm32` types) and fixed both for real, verified via an executed `wasm-bindgen-test` run in headless Firefox. Also fixed a pre-existing wasm32 test-compilation blocker (dev-only tokio `net`/`io-util` features not being target-gated) found along the way, and fixed `jules_sdk::Client`/`Conversation`/`Tool` not being re-exported from the facade crate despite VERSIONING.md documenting them as supported imports. |
| 2026-08-08 | Added `crates/jules-sdk/examples/v1alpha_client.rs`: a live (not mock-backed, unlike this crate's other examples) usage example for the real `v1alpha` session/source client, making only read-only calls; prints setup instructions and exits gracefully if no `JULES_API_KEY` is set. |
| 2026-08-08 | PH2-04 fully closed and moved to Completed Tasks Log. Delegated the `send_message`/`approve_plan` fix to a real Jules session (`sessions/10348936599420094322`) via `create_session`; it researched the live API docs, found `send_message`'s 404 was a session-state timing issue (not a shape bug — confirmed live), fixed `approve_plan` to send `{}` instead of Rust `()` (which serializes to `null`), and opened [PR #79](https://github.com/ArtisanXL/jules-sdk/pull/79). Independently re-verified PR #79 in a clean git worktree (build/test/clippy/fmt all green) before merging. Also moved PH3-02 (retry middleware, already fully ✅ at the subtask level) to the Completed Tasks Log, since it had been left off it. |
| 2026-08-08 | With explicit user go-ahead, tried `create_session`/`send_message`/`approve_plan` against the live API. `create_session` WORKED (created a real, still-running session — see ⚠️ note above). `send_message` and `approve_plan` both FAILED with real API errors (404 / 400) proving our guessed request shapes were wrong; PH2-04.2 stays 🟨 with the corrected findings recorded there. |
| 2026-08-08 | Closed out PH2-04: reworked PH2-04.1's models (were false-✅ id+name stubs), implemented and live-verified `list_sessions`/`get_session`/`list_sources`/`list_activities` against `https://jules.googleapis.com` (PH2-04.3 ✅, PH2-04.4 ✅), left PH2-04.2 🟨 since `create_session`/`send_message`/`approve_plan` are implemented but intentionally unverified against the live API (would mutate a real account). |
| 2026-08-08 | Marked PH3-02.4 (retry middleware) ✅: `RetryMiddleware` now genuinely retries, proven by new test `test_retry_middleware_retries_on_retriable_error`. Added a PH2-04 context note: a tested native HTTP transport (`ReqwestTransport`) and end-to-end `JulesClient`/`JulesClientBuilder` now exist in jules-api, but PH2-04.2-.4 (real `v1alpha` endpoint shapes, base URL, Google Auth) remain ⬜ and unverified against the live API. |
| 2026-08-08 | Corrected false completions found by a code-vs-docs audit: removed PH2-04 (subtasks .2-.4 are ⬜, parent was wrongly logged as complete), PH3-02 (retry middleware, PH3-02.4, does not actually retry per its own source comment), and PH3-03 (CLI support — the entire `jules-cli` crate is a 28-line stub, no subtask was actually done) from the Completed Tasks Log; re-opened their unfinished subtasks in the Pending Work breakdown. Also removed the false "This module has been proofread and verified for the v0.1.0 release." claim from 5 crate root files, and removed "Production-ready" language from Cargo.toml/README.md/CHANGELOG.md pending a working HTTP transport. |
| 2026-08-03 | Completed PH2-04.1: Defined API resource models (Sessions, Activities, Sources) in jules-core |
| 2026-08-02 | Completed TASK-10: Added WASM client tests (FetchClient instantiation) |
| 2026-08-02 | Completed Order 17 (PH5-04): Release candidate preparations |
| 2026-08-02 | Completed Order 16 (PH5-03): Documentation finalization (README, ARCHITECTURE, CHANGELOG, crate-level docs) |
| 2026-08-02 | Completed Order 15 (PH5-02.3): Validated backward compatibility against v0.1.0 API |
| 2026-08-02 | Completed Order 15 (PH5-02.2): Added cross-platform CI matrix |
| 2026-08-02 | Completed Order 14 (PH5-01): API freeze preparations |
| 2026-08-02 | Completed Order 13 (PH4-04.2): Implemented load/soak testing for HTTP client |
| 2026-08-02 | Completed Order 13 (PH4-04.1): Added fuzz testing on core parsing paths |
| 2026-08-02 | Completed Order 10 (PH3-03): CLI support and subcommands |
| 2026-08-01 | Completed Order 9 (PH3-02): Implemented middleware pipeline, logging, and retry support |
| 2026-08-01 | Completed Order 8 (PH3-01): Implemented tool calling support including Tool trait, registry, message serialization, and integration tests |
| 2026-08-01 | Completed PH2-03.4: Write integration test and ensure builders integrate cleanly |
| 2026-08-01 | Completed PH2-03.3: Implement MessageBuilder in jules-core |
| 2026-08-01 | Completed PH2-03.2: Implement ConversationBuilder in jules-core |
| 2026-08-01 | Completed PH2-03.1: Implement RequestBuilder in jules-core |
| 2026-08-01 | Completed PH2-02.5: Wrote streaming integration tests and marked PH2-02 complete |
| 2026-08-01 | Completed PH2-02.4: Implement stream error handling and reconnection |
| 2026-08-01 | Completed PH2-02.3: Implement backpressure / chunk buffering |
| 2026-08-01 | Completed PH2-02.2: Implement SSE (server-sent events) parsing |
| 2026-08-01 | Completed PH2-02.1: Define stream abstraction for incremental responses |
| 2026-08-01 | Completed PH2-01.5: Add rate-limit / retry handling in jules-api |
| 2026-08-01 | Completed parent task PH2-01 (API integrations) |
| 2026-08-01 | Completed PH2-01.4: Implement response deserialization and error mapping in jules-api |
| 2026-08-01 | Completed PH2-01.3: Implement endpoint request construction in jules-api |
| 2026-08-01 | Completed PH2-01.2: Implement authentication handling in jules-api |
| 2026-08-01 | Completed PH2-01.1: Defined HTTP transport layer abstraction in jules-api |
| 2026-08-01 | Completed TASK-15 (Implement ClientBuilder in jules-core) and marked previously implemented TASK-14 as complete |
| 2026-08-01 | Completed TASK-14.2: Define request/response model types used by the Client trait |
| 2026-08-01 | Completed WS-04: CI pipeline implementation with GitHub Actions |
| 2026-08-01 | Marked WS-02.5 and parent WS-02 as complete (crates already initialized)                                |
| 2026-07-31 | Completed WS-02.3: Documented per-crate ownership boundaries in ARCHITECTURE.md |

| 2026-07-31 | Restructured into trackable format with task IDs and progress counters                                       |
| 2026-07-31 | Synced task states with latest checklist                                                                  |
| 2026-07-31 | Replaced High/Medium/Low priority split with a single phased, ordered Pending Tasks list; added Completed Tasks Log |
| 2026-07-31 | Broke down all 17 pending parent tasks into 77 granular subtasks for finer-grained tracking                    |
| 2026-07-31 | Marked WS-02.2 complete as workspace members are already defined in Cargo.toml |
| 2026-08-01 | WS-02.3: Document per-crate ownership/responsibility boundaries in ARCHITECTURE.md |
| 2026-08-01 | WS-02.4: Defined and enforced inter-crate dependency graph |
| 2026-08-02 | Completed parent task PH4-02 (Platform integrations) and subtasks PH4-02.1, PH4-02.2, PH4-02.3 |

This document SHOULD be updated whenever:

* Milestones are completed.
* Priorities change.
* New blockers are discovered.
* Releases are published.
* Architectural changes occur.

Each update SHOULD add a row to the Change Log above.

---

## Final Notes

`PROJECT_STATE.md` exists to provide a concise, authoritative, and **traceable** overview of the repository's current state. Every checkbox has an ID; every ID should be traceable to a commit, PR, or issue.

Contributors are encouraged to prioritize:

* Stability
* Documentation
* Testing
* Maintainability
* Incremental development
* Clear communication

> Before writing code, understand where the project is today and where it is going tomorrow.
