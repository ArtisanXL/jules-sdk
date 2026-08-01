🏃‍♂️ Pacer: Middleware support (PH3-02)

💡 What: Implemented Middleware trait, pipeline, built-in logging and retry middlewares.
🎯 Why: To allow intercepting and modifying requests/responses seamlessly, fulfilling Order 9.
📊 Impact: Enables cross-cutting concerns (logging, retries) and extends the core architecture, marking PH3-02 as complete.
🔬 Measurement: Verify changes with `cargo fmt --all --check`, `cargo clippy --workspace --all-features -- -D warnings`, and `cargo test --workspace --all-features`.
