🏃‍♂️ Pacer: PH4-04.2 Run load/soak testing on client

💡 What: Implemented a load/soak test for the `jules-api` client. It runs 10,000 concurrent requests across 100 tasks to verify the HTTP transport layer's concurrency behavior and resilience.

🎯 Why: To perform stability validations in Phase 4 and guarantee that the transport layer handles high concurrency accurately and doesn't crash or drop requests when operating under stress.

📊 Impact: Satisfies Order 13 task PH4-04.2 (Run load/soak testing on client), reducing the remaining pending subtasks by 1 and pushing Phase 4 closer to completion.

🔬 Measurement: Run `cargo test --test load_test` to verify load testing passes.
