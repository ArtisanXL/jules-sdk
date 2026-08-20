use std::fs;

fn main() {
    let mut contents = fs::read_to_string("crates/jules-api/src/streaming/sse.rs").unwrap();

    let search = r#"        if self.buffer.contains('\r') {
            // Bolt optimization: Perform CRLF/CR normalization in-place without allocating intermediate strings.
            // We only replace \r (ASCII) with \n (ASCII) or remove bytes, so valid UTF-8 remains valid UTF-8.
            let bytes = unsafe { self.buffer.as_mut_vec() };
            let holdback = bytes.ends_with(b"\r");
            let split_at = bytes.len() - usize::from(holdback);

            let mut read_idx = 0;
            let mut write_idx = 0;

            while read_idx < split_at {
                let b = bytes[read_idx];
                if b == b'\r' {
                    if read_idx + 1 < split_at && bytes[read_idx + 1] == b'\n' {
                        bytes[write_idx] = b'\n';
                        write_idx += 1;
                        read_idx += 2;
                        continue;
                    }
                    bytes[write_idx] = b'\n';
                } else {
                    bytes[write_idx] = b;
                }
                write_idx += 1;
                read_idx += 1;
            }

            if holdback {
                bytes[write_idx] = b'\r';
                write_idx += 1;
            }

            bytes.truncate(write_idx);
        }"#;

    let replace = r#"        if self.buffer.contains('\r') {
            let holdback = self.buffer.ends_with('\r');
            if holdback {
                self.buffer.pop();
            }
            self.buffer.retain(|c| c != '\r');
            if holdback {
                self.buffer.push('\r');
            }
        }"#;

    contents = contents.replace(search, replace);
    fs::write("crates/jules-api/src/streaming/sse.rs", contents).unwrap();
}
