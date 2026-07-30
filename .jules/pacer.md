## 2026-07-30 - [Crate Structure Implementation]
**Learning:** Re-exporting overlapping internal modules into a facade crate without renaming them can cause ambiguous glob re-exports. To avoid this, it's safer to re-export the entire crate rather than the internal modules. Also, procedular macro crates cannot export public items besides macros.
**Action:** When working with facade crates, prefer explicit crate exports instead of relying on wildcard re-exports of modules. When creating a procedural macro crate, make sure all modules except macros are private.
