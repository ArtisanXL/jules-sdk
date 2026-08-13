
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
