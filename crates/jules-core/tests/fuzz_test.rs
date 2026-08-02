use jules_core::message::Message;
use jules_core::response::ClientResponse;
use jules_core::streaming::StreamEvent;
use jules_core::tool::ToolCall;
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_message_deserialization_fuzz(s in ".*") {
        let _ = serde_json::from_str::<Message>(&s);
    }

    #[test]
    fn test_client_response_deserialization_fuzz(s in ".*") {
        let _ = serde_json::from_str::<ClientResponse>(&s);
    }

    #[test]
    fn test_stream_event_deserialization_fuzz(s in ".*") {
        let _ = serde_json::from_str::<StreamEvent>(&s);
    }

    #[test]
    fn test_tool_call_deserialization_fuzz(s in ".*") {
        let _ = serde_json::from_str::<ToolCall>(&s);
    }
}
