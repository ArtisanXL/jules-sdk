🏃‍♂️ Pacer: Tool calling support (PH3-01)

💡 What: Implemented tool calling support including the `Tool` and `DynTool` traits, a `ToolRegistry` for managing tool instances, and updated the `Message` model to support tool calls and tool call IDs when the `tools` feature is enabled. Wrote full integration tests to verify the tool execution pipeline.
🎯 Why: This enables LLMs interacting with Jules-SDK to request execution of specific functions or external services, expanding the capabilities of the SDK.
📊 Impact: Completes Phase 3 subtasks (PH3-01.1 to PH3-01.5) and fully satisfies the `PH3-01` parent task constraint.
🔬 Measurement: Verify changes with `cargo fmt --all --check`, `cargo clippy --workspace --all-features -- -D warnings`, and `cargo test --workspace --all-features`.
