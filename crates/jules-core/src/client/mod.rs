//! Client module.

use crate::conversation::Conversation;

/// A request to be sent by a client.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ClientRequest {
    /// The conversation containing messages to send.
    pub conversation: Conversation,
}

impl ClientRequest {
    /// Creates a new `ClientRequest` with the given conversation.
    #[must_use]
    pub fn new(conversation: Conversation) -> Self {
        Self { conversation }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_request_new() {
        let conv = Conversation::new();
        let req = ClientRequest::new(conv.clone());
        assert_eq!(req.conversation, conv);
    }
}
