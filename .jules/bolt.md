
## $(date +%Y-%m-%d) - Prevent capacity reset during String reassignment in buffer drain
**Learning:** Assigning a substring back to a `String` variable (`buffer = buffer[idx..].to_string()`) creates a new String, completely dropping the carefully pre-allocated capacity. In hot paths like a streaming chunk buffer, this leads to continuous reallocation on every single read.
**Action:** Use `String::drain(..idx)` instead of reassignment to remove a prefix while preserving the buffer's existing capacity, significantly reducing allocations in hot loops.
