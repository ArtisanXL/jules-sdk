## 2026-07-30 - [Crate Structure Implementation]
**Learning:** Re-exporting overlapping internal modules into a facade crate without renaming them can cause ambiguous glob re-exports. To avoid this, it's safer to re-export the entire crate rather than the internal modules. Also, procedular macro crates cannot export public items besides macros.
**Action:** When working with facade crates, prefer explicit crate exports instead of relying on wildcard re-exports of modules. When creating a procedural macro crate, make sure all modules except macros are private.
## 2024-02-05 - [Additional tooling support (cargo-deny)]
**Learning:** `cargo-deny` requires an explicit configuration to accept common licenses like MIT and Apache-2.0, otherwise it treats everything as a failure.
**Action:** When adding `cargo deny` to any future project or workspace, ensure `deny.toml` explicitly sets up `allow = ["MIT", "Apache-2.0"]` and other dependencies' valid licenses.
