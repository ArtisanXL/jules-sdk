🏃‍♂️ Pacer: Order 11 - PH3-04 Additional Examples

💡 What:
Implemented remaining examples for the Jules-SDK to cover additional functionalities:
1. `streaming`: Demonstrates the `Stream` and `StreamEvent` API.
2. `tools`: Demonstrates `Tool` implementation and the `ToolRegistry` API.
3. `middleware`: Showcases custom `Middleware` and pipeline execution.
4. `cli_usage`: Simulates initial configuration usage when run as a CLI tool.

🎯 Why:
This was pending in the backlog to complete phase 3 (`PH3-04`), helping developers understand how to extend and use these advanced components effectively.

📊 Impact:
Provides concrete code implementations for advanced usages of Jules-SDK, increasing usability and decreasing onboarding time for developers extending the system. Completed parent task `PH3-04` by finishing its 4 subtasks.

🔬 Measurement:
`cargo fmt --all --check`
`cargo clippy --workspace --all-features -- -D warnings`
`cargo test`
`cargo check --examples`
