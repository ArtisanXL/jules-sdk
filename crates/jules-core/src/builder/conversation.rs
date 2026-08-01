use crate::conversation::Conversation;
use crate::message::Message;

/// A builder for creating a `Conversation`.
#[derive(Debug, Clone, Default)]
pub struct ConversationBuilder {
    conversation: Conversation,
}

impl ConversationBuilder {
    /// Creates a new `ConversationBuilder`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a single message to the conversation.
    #[must_use]
    pub fn add_message(mut self, message: Message) -> Self {
        self.conversation.add_message(message);
        self
    }

    /// Adds multiple messages to the conversation.
    #[must_use]
    pub fn add_messages(mut self, messages: impl IntoIterator<Item = Message>) -> Self {
        for message in messages {
            self.conversation.add_message(message);
        }
        self
    }

    /// Builds the `Conversation`.
    #[must_use]
    pub fn build(self) -> Conversation {
        self.conversation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::Role;

    #[test]
    fn test_conversation_builder_default() {
        let builder = ConversationBuilder::new();
        let conv = builder.build();
        assert!(conv.messages().is_empty());
    }

    #[test]
    fn test_conversation_builder_add_message() {
        let builder = ConversationBuilder::new().add_message(Message::new(Role::User, "Hello"));
        let conv = builder.build();

        assert_eq!(conv.messages().len(), 1);
        assert_eq!(conv.messages()[0].content(), "Hello");
    }

    #[test]
    fn test_conversation_builder_add_messages() {
        let msgs = vec![
            Message::new(Role::User, "Hello"),
            Message::new(Role::Assistant, "Hi there!"),
        ];
        let builder = ConversationBuilder::new().add_messages(msgs);
        let conv = builder.build();

        assert_eq!(conv.messages().len(), 2);
        assert_eq!(conv.messages()[0].content(), "Hello");
        assert_eq!(conv.messages()[1].content(), "Hi there!");
    }
}
