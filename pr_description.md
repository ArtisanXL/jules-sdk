🏃‍♂️ Pacer: Implement general Builders (RequestBuilder, ConversationBuilder, MessageBuilder)

💡 What:
Implemented `RequestBuilder`, `ConversationBuilder`, and `MessageBuilder` in `jules-core`. Added integration tests to ensure that these builders interact correctly with each other and with the existing `ClientBuilder`. Also marked the overarching PH2-03 parent task and all of its subtasks as complete in `PROJECT_STATE.md`.

🎯 Why:
To provide a convenient, type-safe, and ergonomic way for users of the SDK to construct requests, conversations, and individual messages, fulfilling the requirements for Phase 2 Builders task.

📊 Impact:
Completes the entire Phase 2 Builders parent task (`PH2-03`) and subtasks (`PH2-03.1`, `PH2-03.2`, `PH2-03.3`, `PH2-03.4`). Advances the project towards a more comprehensive and developer-friendly public API.

🔬 Measurement:
Run the tests manually to verify the changes:
```bash
cargo fmt --all --check
cargo clippy --workspace --all-features -- -D warnings
cargo test --workspace --all-features
```
