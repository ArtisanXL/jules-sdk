use crate::client::ClientRequest;
use crate::conversation::Conversation;
use crate::message::Message;

/// A builder for creating a `ClientRequest`.
#[derive(Debug, Clone, Default)]
pub struct RequestBuilder {
    conversation: Conversation,
}

impl RequestBuilder {
    /// Creates a new `RequestBuilder`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the entire conversation for the request.
    #[must_use]
    pub fn conversation(mut self, conversation: Conversation) -> Self {
        self.conversation = conversation;
        self
    }

    /// Adds a single message to the conversation.
    #[must_use]
    pub fn add_message(mut self, message: Message) -> Self {
        self.conversation.add_message(message);
        self
    }

    /// Builds the `ClientRequest`.
    #[must_use]
    pub fn build(self) -> ClientRequest {
        ClientRequest::new(self.conversation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::Role;

    #[test]
    fn test_request_builder_default() {
        let builder = RequestBuilder::new();
        let req = builder.build();
        assert!(req.conversation.messages().is_empty());
    }

    #[test]
    fn test_request_builder_with_conversation() {
        let mut conv = Conversation::new();
        conv.add_message(Message::new(Role::User, "Hello"));

        let builder = RequestBuilder::new().conversation(conv.clone());
        let req = builder.build();

        assert_eq!(req.conversation, conv);
    }

    #[test]
    fn test_request_builder_add_message() {
        let builder = RequestBuilder::new().add_message(Message::new(Role::User, "Hello"));
        let req = builder.build();

        assert_eq!(req.conversation.messages().len(), 1);
        assert_eq!(req.conversation.messages()[0].content(), "Hello");
    }
}
