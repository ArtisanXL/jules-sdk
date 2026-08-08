
## $(date +%Y-%m-%d) - Prevent capacity reset during String reassignment in buffer drain
**Learning:** Assigning a substring back to a `String` variable (`buffer = buffer[idx..].to_string()`) creates a new String, completely dropping the carefully pre-allocated capacity. In hot paths like a streaming chunk buffer, this leads to continuous reallocation on every single read.
**Action:** Use `String::drain(..idx)` instead of reassignment to remove a prefix while preserving the buffer's existing capacity, significantly reducing allocations in hot loops.
## 2024-06-25 - Optimize SSE Parsing String Allocations
**Learning:** Found O(N^2) memory reallocation in streaming parser. `self.buffer = self.buffer[pos+2..].to_string()` in a loop causes quadratic behavior when receiving large batches of SSE events.
**Action:** Use `.find()` with string slices `&self.buffer[last_pos..abs_pos]` and single `.drain(..last_pos)` at the end to turn O(N^2) into O(N).
## 2024-08-07 - Optimize Middleware Pipeline Iteration
**Learning:** Building middleware execution chains recursively wraps the final handler in an `Arc` and incurs multiple heap allocation penalties due to recursive `Fn` closures.
**Action:** Build the execution chain iteratively in reverse (`iter().rev()`), which allows using `FnOnce` bounds, avoids wrapping the final handler in an `Arc`, and eliminates recursive closure heap allocations.
## $(date +%Y-%m-%d) - Pre-allocate string capacities in streaming hot paths
**Learning:** In hot loops like Server-Sent Events (SSE) parsing, `String::new()` followed by repeated `push_str()` (e.g., when accumulating multiple `data:` lines) causes unnecessary O(log N) string capacity reallocations per event. Similarly, initializing a chunk buffer parser with `Default::default()` creates a zero-capacity string, leading to immediate reallocations on the first few chunks.
**Action:** Always pre-allocate strings in hot paths. For stream parsers, manually implement `Default` to initialize buffers with sensible capacities (e.g., `String::with_capacity(8192)`). For parsed events, use the input slice length (`block.len()`) as a safe upper bound to initialize the string (`String::with_capacity(block.len())`), ensuring O(1) allocation per event.
