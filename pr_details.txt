Title: 🏃‍♂️ Pacer: PH4-04.1: Run fuzz testing on core parsing paths

💡 What:
- Added `proptest` dev-dependency to both `jules-api` and `jules-core`.
- Implemented fuzz tests using `proptest!` macro in `crates/jules-api/src/streaming/sse.rs` to validate arbitrary data against the SSE parser.
- Added `crates/jules-core/tests/fuzz_test.rs` to run arbitrary string input against serialization of `Message`, `ClientResponse`, `StreamEvent`, and `ToolCall` JSON models.
- Updated `PROJECT_STATE.md` to reflect the completion of the `PH4-04.1` task and updated completion counts.

🎯 Why:
- Fuzz testing parsing paths guards against panic cases due to invalid characters and memory exhaustion on arbitrarily large strings from untrusted endpoints. It provides structural integrity confidence beyond the defined spec.

📊 Impact:
- Subtask PH4-04.1 from Phase 4 is now marked ✅. Total pending subtasks decremented and `Completed Tasks Log` updated.

🔬 Measurement:
Run `cargo test --workspace` to execute all tests including the new `proptest` suites and run `cargo clippy --workspace --all-features -- -D warnings` and `cargo fmt --all --check` to verify code quality.
