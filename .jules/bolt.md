
## 2024-08-11 - Prevent capacity reset during String reassignment in buffer drain
**Learning:** Assigning a substring back to a `String` variable (`buffer = buffer[idx..].to_string()`) creates a new String, completely dropping the carefully pre-allocated capacity. In hot paths like a streaming chunk buffer, this leads to continuous reallocation on every single read.
**Action:** Use `String::drain(..idx)` instead of reassignment to remove a prefix while preserving the buffer's existing capacity, significantly reducing allocations in hot loops.
## 2024-06-25 - Optimize SSE Parsing String Allocations
**Learning:** Found O(N^2) memory reallocation in streaming parser. `self.buffer = self.buffer[pos+2..].to_string()` in a loop causes quadratic behavior when receiving large batches of SSE events.
**Action:** Use `.find()` with string slices `&self.buffer[last_pos..abs_pos]` and single `.drain(..last_pos)` at the end to turn O(N^2) into O(N).
## 2024-08-07 - Optimize Middleware Pipeline Iteration
**Learning:** Building middleware execution chains recursively wraps the final handler in an `Arc` and incurs multiple heap allocation penalties due to recursive `Fn` closures.
**Action:** Build the execution chain iteratively in reverse (`iter().rev()`), which allows using `FnOnce` bounds, avoids wrapping the final handler in an `Arc`, and eliminates recursive closure heap allocations.
## 2024-08-11 - Pre-allocate string capacities in streaming hot paths
**Learning:** In hot loops like Server-Sent Events (SSE) parsing, `String::new()` followed by repeated `push_str()` (e.g., when accumulating multiple `data:` lines) causes unnecessary O(log N) string capacity reallocations per event. Similarly, initializing a chunk buffer parser with `Default::default()` creates a zero-capacity string, leading to immediate reallocations on the first few chunks.
**Action:** Always pre-allocate strings in hot paths. For stream parsers, manually implement `Default` to initialize buffers with sensible capacities (e.g., `String::with_capacity(8192)`). For parsed events, use the input slice length (`block.len()`) as a safe upper bound to initialize the string (`String::with_capacity(block.len())`), ensuring O(1) allocation per event.

## 2024-08-10 - Retaining String Capacity in Hot Loops
**Learning:** In Rust, assigning a newly created `String` to an existing `String` variable (e.g., `self.buffer = new_string;`) drops the original string's capacity. When dealing with pre-allocated buffers in hot streaming loops, this forces continuous memory reallocation.
**Action:** Use `String::clear()` and `String::push_str()` instead of assignment to overwrite a buffer while preserving its allocated capacity.
## 2024-08-11 - Optimize ASCII string replacement in hot loops
**Learning:** Using `.replace()` on a string creates a completely new string allocation. Chaining `.replace().replace()` creates multiple allocations. In hot loops like stream parsing (e.g. CRLF normalization), this causes immense allocator thrashing.
**Action:** When replacing or removing ASCII characters (like `\r` and `\n`) in strings where you can guarantee the replacement is the same size or smaller, use `unsafe { string.as_mut_vec() }` to iterate through and modify the bytes in-place. Truncate the resulting length. This preserves the original string capacity and eliminates intermediate string allocations, providing dramatic performance improvements.
## 2024-08-14 - Optimize HTTP Header Sanitization Allocations
**Learning:** Using `.replace()` on a Rust `String` creates a completely new string allocation. In hot paths like HTTP request header construction, this causes unnecessary heap allocations for every single header key and value. When an owned `String` is already available, this is highly inefficient.
**Action:** When filtering out specific characters (like `\r` and `\n` for header injection prevention) from an owned `String`, use the safe `String::retain` method instead of `.replace()`. This modifies the string in-place, preserving the existing capacity and eliminating redundant allocations, while remaining completely safe.
## 2026-08-08 - Use String::retain for safe in-place character removal
**Learning:** Using `unsafe { string.as_mut_vec() }` to manually replace bytes in a hot path is an anti-pattern when dropping characters (like `\r`) achieves the same goal.
**Action:** Use `String::retain(|c| c != '\r')` to safely remove characters in O(N) time with zero allocations, avoiding `unsafe` blocks.
## 2026-08-08 - String capacity estimation for percent-encoded URLs
**Learning:** In URL construction loops (`Endpoint::build_url`), estimating string capacity strictly by `key.len() + value.len()` ignores that percent encoding (`%XX`) expands spaces and special characters. This under-allocation causes `String::with_capacity` to silently fail at its purpose, leading to multiple hidden allocations during the `.extend()` operations in a hot path.
**Action:** When pre-allocating capacity for percent-encoded data, conservatively multiply the unencoded lengths by 3 (the maximum possible expansion factor) to ensure O(1) string capacity behavior.
## 2026-08-26 - Optimize string draining in streaming buffer
**Learning:** Using `.drain(..).collect()` on a `String` is ~3x slower in Rust compared to slicing and cloning (`string[..idx].to_owned()`) followed by `.drain(..idx)`, because `collect()` iterates over `char`s individually instead of doing a fast memory copy.
**Action:** Avoid `.drain(..).collect()` for large or frequently accessed `String` buffers in hot loops; use `to_owned()` and then `.drain()` instead to optimize memcpy.
## 2024-09-03 - Eliminate unnecessary `to_string()` allocations
**Learning:** Calling `to_string()` on `&str` references before passing them into string-formatting macros (`format!`) or builder functions that already accept generic `impl Into<String>` forces redundant heap allocations for intermediate strings.
**Action:** When working with macros like `format!` or functions that accept `impl Into<String>`, use the `&str` reference directly to avoid the intermediate allocation.
