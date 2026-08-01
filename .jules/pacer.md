## 2026-08-01 - Overall Progress Counters
**Learning:** `PROJECT_STATE.md` does not have "Overall Progress" or "Milestones" tables as implicitly expected by instructions in previous tasks. They might have been removed in a previous phase. The instructions mentioned "update the area's row in `## Overall Progress` ... the `Completed: N / Remaining: N` total ... and the relevant milestone row in `## Milestones`". However, those sections do not exist in the exact format, but there is "Pending Work" count and specific documentation/workspace counts.
**Action:** Before writing regexes to update non-existent counters based on prompt instructions, I will always explicitly read the whole file to confirm what exact sections and strings exist, obeying the Groundedness Rule.
## 2026-08-01 - [Retry implementation with backoff]
**Learning:** When implementing retry mechanisms like exponential backoff, it is crucial to handle state safely, especially without relying on async state within the logic itself when simpler functional returns (like `Option<u64>` for delay) decouple the state from the executor.
**Action:** Extract retry logic into standalone, testable units (like the `should_retry` trait method) rather than coupling it directly into the HTTP transport implementation. This keeps the transport clean and makes the policy entirely unit-testable.
## 2025-01-20 - Streaming reconnection trait boundaries
**Learning:** When dealing with asynchronous reconnect logic that returns `impl Future<Output = Option<Self::Item>> + Send`, the underlying items (e.g., `Result<T, E>`) and the generic variables (`T`, `E`) must strictly implement `Send`. The compiler explicitly complained that `E` (error type) is used across an await point.
**Action:** Always constrain generic parameters `T: Send` and `E: Send` when implementing streams that execute asynchronous `match` block branches where internal parameters may outlive the `await`.

## 2026-08-01 - Mocking Async Streams for Reconnect Testing
**Learning:** When writing integration tests for async streams that require `Stream` traits (like `ReconnectableStream`), using a poor man's `block_on` (polling the future manually with a noop waker) is a viable way to test async stream logic locally without requiring the `tokio` runtime to be added as a dev-dependency if it causes unresolved module issues.
**Action:** Use manual future polling loops (with a bounded iteration count to prevent infinite loops) to test simple async stream interactions when avoiding heavy runtime dependencies in tests.
## 2026-08-01 - [Retry implementation with backoff]
**Learning:** When implementing retry mechanisms like exponential backoff, it is crucial to handle state safely, especially without relying on async state within the logic itself when simpler functional returns (like `Option<u64>` for delay) decouple the state from the executor.
**Action:** Extract retry logic into standalone, testable units (like the `should_retry` trait method) rather than coupling it directly into the HTTP transport implementation. This keeps the transport clean and makes the policy entirely unit-testable.
