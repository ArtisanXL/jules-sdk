//! Conversation module.

use crate::message::Message;

/// A conversation consisting of a sequence of messages.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Conversation {
    messages: Vec<Message>,
}

impl Conversation {
    /// Creates a new, empty [`Conversation`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a [`Message`] to the end of the conversation.
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    /// Returns a slice of the [`Message`]s in the conversation.
    #[must_use]
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::Role;

    #[test]
    fn test_conversation_add_message() {
        let mut conv = Conversation::new();
        assert!(conv.messages().is_empty());

        let msg = Message::new(Role::User, "Hello");
        conv.add_message(msg);

        assert_eq!(conv.messages().len(), 1);
        assert_eq!(conv.messages()[0], Message::new(Role::User, "Hello"));
    }
}
