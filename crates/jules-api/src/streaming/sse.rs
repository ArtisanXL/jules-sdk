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
#[derive(Default)]
pub struct SseParser {
    buffer: String,
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
        let mut events = Vec::new();

        while let Some(pos) = self.buffer.find("\n\n") {
            let block = self.buffer[..pos].to_string();
            self.buffer = self.buffer[pos + 2..].to_string();

            if let Some(event) = Self::parse_block(&block) {
                events.push(event);
            }
        }

        events
    }

    fn parse_block(block: &str) -> Option<SseEvent> {
        if block.is_empty() {
            return None;
        }

        let mut event = SseEvent::default();
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
}

#[cfg(test)]
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
