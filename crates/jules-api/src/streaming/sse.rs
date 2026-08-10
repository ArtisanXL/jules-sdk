//! Server-Sent Events (SSE) parsing.
use serde::{Deserialize, Serialize};

/// Represents a single Server-Sent Event (SSE).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SseEvent {
    /// The event type (e.g., "message").
    pub event: Option<String>,
    /// The event payload.
    pub data: String,
    /// An optional event ID.
    pub id: Option<String>,
    /// The retry time in milliseconds.
    pub retry: Option<u64>,
}

/// A parser for buffering and yielding `SseEvent`s from a text stream.
pub struct SseParser {
    buffer: String,
}

impl Default for SseParser {
    fn default() -> Self {
        Self {
            // Bolt optimization: Pre-allocate 8KB to avoid reallocations on initial streaming chunks.
            buffer: String::with_capacity(8192),
        }
    }
}

impl SseParser {
    /// Creates a new `SseParser`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes a chunk of text into the parser, returning any complete `SseEvent`s parsed.
    pub fn push(&mut self, chunk: &str) -> Vec<SseEvent> {
        self.buffer.push_str(chunk);
        // Normalize CRLF/CR line endings to LF so block-boundary detection (`\n\n`) also
        // matches CRLF-terminated events (`\r\n\r\n`), as permitted by the SSE spec. Done over
        // the whole buffer (not just the new chunk) so a CRLF split across two `push` calls is
        // still normalized correctly once both halves have arrived. A trailing lone `\r` is
        // held back from normalization in case the matching `\n` arrives in the next chunk.
        if self.buffer.contains('\r') {
            let holdback = self.buffer.ends_with('\r');
            let split_at = self.buffer.len() - usize::from(holdback);
            let normalized = self.buffer[..split_at]
                .replace("\r\n", "\n")
                .replace('\r', "\n");

            // Bolt optimization: Retain buffer capacity instead of reassigning
            // which drops the 8KB pre-allocation and forces continuous memory reallocation.
            self.buffer.clear();
            self.buffer.push_str(&normalized);
            if holdback {
                self.buffer.push('\r');
            }
        }
        let mut events = Vec::new();

        let mut last_pos = 0;
        while let Some(pos) = self.buffer[last_pos..].find("\n\n") {
            let abs_pos = last_pos + pos;
            let block = &self.buffer[last_pos..abs_pos];

            if let Some(event) = Self::parse_block(block) {
                events.push(event);
            }

            last_pos = abs_pos + 2;
        }

        if last_pos > 0 {
            self.buffer.drain(..last_pos);
        }

        events
    }

    fn parse_block(block: &str) -> Option<SseEvent> {
        if block.is_empty() {
            return None;
        }

        let mut event = SseEvent {
            // Bolt optimization: Pre-allocate `data` capacity to the block length (the absolute maximum possible size).
            // This completely eliminates string reallocations when appending multiple `data:` lines in a hot streaming loop.
            data: String::with_capacity(block.len()),
            ..Default::default()
        };
        let mut has_data = false;

        for line in block.lines() {
            if line.starts_with(':') {
                continue; // Comment
            }

            if let Some(colon_pos) = line.find(':') {
                let field = &line[..colon_pos];
                let mut value = &line[colon_pos + 1..];

                if value.starts_with(' ') {
                    value = &value[1..];
                }

                match field {
                    "event" => event.event = Some(value.to_string()),
                    "data" => {
                        if has_data {
                            event.data.push('\n');
                        }
                        event.data.push_str(value);
                        has_data = true;
                    }
                    "id" => event.id = Some(value.to_string()),
                    "retry" => {
                        if let Ok(retry) = value.parse() {
                            event.retry = Some(retry);
                        }
                    }
                    _ => {} // Ignore unknown fields
                }
            }
        }

        if has_data || event.event.is_some() || event.id.is_some() || event.retry.is_some() {
            Some(event)
        } else {
            None
        }
    }
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use super::*;

    #[test]
    fn test_sse_parser_basic() {
        let mut parser = SseParser::new();
        let events = parser.push("data: hello\n\n");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].data, "hello");
    }

    #[test]
    fn test_sse_parser_fragmented() {
        let mut parser = SseParser::new();
        let mut events = parser.push("data: he");
        assert!(events.is_empty());
        events = parser.push("llo\n\n");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].data, "hello");
    }

    #[test]
    fn test_sse_parser_multiline() {
        let mut parser = SseParser::new();
        let events = parser.push("data: line1\ndata: line2\n\n");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].data, "line1\nline2");
    }

    #[test]
    fn test_sse_parser_all_fields() {
        let mut parser = SseParser::new();
        let events = parser.push("id: 123\nevent: message\ndata: payload\nretry: 5000\n\n");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].id, Some("123".to_string()));
        assert_eq!(events[0].event, Some("message".to_string()));
        assert_eq!(events[0].data, "payload");
        assert_eq!(events[0].retry, Some(5000));
    }

    /// Real servers/proxies often emit CRLF-terminated SSE events. Block boundaries must be
    /// detected on `\r\n\r\n`, not just a literal `\n\n`.
    #[test]
    fn test_sse_parser_crlf_line_endings() {
        let mut parser = SseParser::new();
        let events = parser.push("id: 1\r\ndata: hello\r\n\r\n");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].id, Some("1".to_string()));
        assert_eq!(events[0].data, "hello");
    }

    /// A CRLF split across two `push` calls (the `\r` in one chunk, `\n` in the next) must
    /// still be normalized and parsed correctly.
    #[test]
    fn test_sse_parser_crlf_split_across_pushes() {
        let mut parser = SseParser::new();
        let events = parser.push("data: hello\r");
        assert!(events.is_empty());
        let events = parser.push("\ndata: world\r\n\r\n");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].data, "hello\nworld");
    }
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod fuzz {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_sse_parser_doesnt_crash(s in ".*") {
            let mut parser = SseParser::new();
            let _ = parser.push(&s);
        }

        #[test]
        fn test_sse_parser_multiple_pushes(chunks in proptest::collection::vec(".*", 1..10)) {
            let mut parser = SseParser::new();
            for chunk in chunks {
                let _ = parser.push(&chunk);
            }
        }
    }
}
