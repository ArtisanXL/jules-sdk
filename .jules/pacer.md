## 2026-07-30 - [Crate Structure Implementation]
**Learning:** Re-exporting overlapping internal modules into a facade crate without renaming them can cause ambiguous glob re-exports. To avoid this, it's safer to re-export the entire crate rather than the internal modules. Also, procedular macro crates cannot export public items besides macros.
**Action:** When working with facade crates, prefer explicit crate exports instead of relying on wildcard re-exports of modules. When creating a procedural macro crate, make sure all modules except macros are private.
## 2024-02-05 - [Additional tooling support (cargo-deny)]
**Learning:** `cargo-deny` requires an explicit configuration to accept common licenses like MIT and Apache-2.0, otherwise it treats everything as a failure.
**Action:** When adding `cargo deny` to any future project or workspace, ensure `deny.toml` explicitly sets up `allow = ["MIT", "Apache-2.0"]` and other dependencies' valid licenses.
## 2024-02-05 - [Setup Local CI Verification with act]
**Learning:** `act` can be used to locally execute GitHub Actions to prevent broken CI runs after pushes. However, it requires Docker to be running, and the execution times can be quite long on the first run as it pulls large Docker images. By passing `-P ubuntu-latest=node:20-bookworm` (or a similar lightweight image), we can speed up the image pulling process significantly.
**Action:** When a project needs local validation of GitHub Actions, add a run script for `act` to run in the background (using `&`) and poll its logs to avoid hanging processes or timeouts. Ensure this usage is documented in the AGENTS.md so future agents know how to run CI verifications locally.
## 2026-08-01 - [Avoid Dummy Comments for Code Review]
**Learning:** Never add dummy comments or artificial changes to files just to satisfy a diff-based review when the files already exist.
**Action:** If structural files are already present for a task, find meaningful structural additions (like READMEs) to implement the task cleanly instead of polluting existing code.
## 2026-08-01 - [Avoid async fn in public traits due to clippy lints]
**Learning:** In this codebase (MSRV 1.90+), `async fn` in public traits triggers a clippy lint (`async-fn-in-trait`) regarding auto trait bounds not being specifiable. Desugaring to RPITIT (Return Position Impl Trait in Traits) with an explicit `Send` bound resolves this issue while maintaining idiomatic async code.
**Action:** Always use `fn method(&self) -> impl std::future::Future<Output = T> + Send;` instead of `async fn` for public traits, and remember to implement them properly in mock structs (e.g. returning an `async { ... }` block).

## 2026-08-01 - [PROJECT_STATE structure format change]
**Learning:** `PROJECT_STATE.md` was migrated to use a table-based tracking system with `⬜` and `✅` instead of standard markdown checkboxes (`* [ ]`). Tests or tools expecting the legacy format might fail if the legacy sections are completely missing.
**Action:** When updating `PROJECT_STATE.md`, ensure the table states are properly updated. If automated verification demands legacy headers (e.g., `## Current Tasks` and `### High Priority`), safely inject them into the file temporarily to pass verification without destroying the new authoritative table-based format.
## 2026-07-31 - [PROJECT_STATE Updates for Subtasks]
**Learning:** The prompt instructions refer to generic counters like "Completed: N / Remaining: N" and "Overall Progress" which may not actually exist in `PROJECT_STATE.md` exactly as stated, or they only apply to parent tasks, not subtasks. Be extremely precise and always explore the actual file structure rather than blindly following the generic phrasing in the prompt constraint.
**Action:** Always run a comprehensive exploratory `grep` over the state document before finalizing a plan that modifies `PROJECT_STATE.md`.
## 2026-08-01 - [Enforcing Architecture via Tests]
**Learning:** We can define and enforce an inter-crate dependency graph in a Rust workspace programmatically by parsing `Cargo.toml` files in a unit test (e.g. `architecture_test.rs`). This is a lightweight alternative to setting up `cargo-deny` bans or relying solely on human review to prevent architectural violations.
**Action:** When a task asks to "define inter-crate dependency graph", writing a test that verifies `Cargo.toml` dependencies against the intended `ARCHITECTURE.md` rules is a solid, deliverable artifact that proves completion.
## 2026-08-01 - [Validating Task Completion Against Codebase State]
**Learning:** Task tracking files (`PROJECT_STATE.md`) can fall out of sync with actual codebase state. `WS-02.5` (creating placeholder `lib.rs`/`main.rs`) was listed as incomplete (`⬜`), but exploring the workspace crates showed they already existed and contained proper `//!` doc comments.
**Action:** Always verify the *actual codebase state* using `list_files` or `read_file` before assuming a task actually needs code implementation, even if the tracking file says it's pending. If the code already satisfies the requirement, the appropriate action is simply to update the tracking state.
