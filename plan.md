1. **Remove `#[derive(Debug)]` from `StreamEvent`**
   - File: `crates/jules-core/src/streaming/mod.rs`
   - Modify `StreamEvent` to no longer derive `Debug`.

2. **Manually implement `std::fmt::Debug` for `StreamEvent`**
   - File: `crates/jules-core/src/streaming/mod.rs`
   - In the manual implementation, explicitly redact the text within `TextChunk` variants by replacing the string with `"***REDACTED***"`.
   - The `Done` variant can be logged as-is.

3. **Append to `.jules/sentinel.md`**
   - Following the Sentinel guidelines, append a journal entry for this finding to `.jules/sentinel.md`.

4. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**
   - Run tests, format, and lint to verify the fixes work properly and adhere to guidelines.

5. **Submit a Pull Request**
   - Use `submit` to create the PR, with title `🛡️ Sentinel: [HIGH] Redact PII in StreamEvent Debug`.
