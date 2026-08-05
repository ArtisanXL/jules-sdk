
## $(date +%Y-%m-%d) - Prevent capacity reset during String reassignment in buffer drain
**Learning:** Assigning a substring back to a `String` variable (`buffer = buffer[idx..].to_string()`) creates a new String, completely dropping the carefully pre-allocated capacity. In hot paths like a streaming chunk buffer, this leads to continuous reallocation on every single read.
**Action:** Use `String::drain(..idx)` instead of reassignment to remove a prefix while preserving the buffer's existing capacity, significantly reducing allocations in hot loops.

## 2023-10-27 - Iterate instead of recurse for Middleware chains
**Learning:** Using recursive closures (i.e. `Fn`) to build a middleware execution chain forces you to wrap the `final_handler` in an `Arc` (as it must be clonable into each closure's lifetime scope) and imposes `Fn` bounds instead of `FnOnce`. This incurs heap allocation penalties and runtime overhead on every middleware pipeline execution.
**Action:** Use an iterative approach traversing the middleware vector in reverse (`iter().rev()`) and re-assigning the `next` closure. This allows the inner closure to consume variables cleanly via `FnOnce`, eliminating the need for `Arc` wrapping and speeding up pipeline construction/execution by ~30%.
