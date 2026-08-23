use crate::message::{Message, Role};

/// A builder for creating a `Message`.
#[derive(Clone)]
pub struct MessageBuilder {
    role: Role,
    content: String,
}

impl std::fmt::Debug for MessageBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageBuilder")
            .field("role", &self.role)
            .field("content", &"***REDACTED***")
            .finish()
    }
}

impl Default for MessageBuilder {
    fn default() -> Self {
        Self {
            role: Role::User,
            content: String::new(),
        }
    }
}

impl MessageBuilder {
    /// Creates a new `MessageBuilder`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the role of the message.
    #[must_use]
    pub fn role(mut self, role: Role) -> Self {
        self.role = role;
        self
    }

    /// Sets the content of the message.
    #[must_use]
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    /// Builds the `Message`.
    #[must_use]
    pub fn build(self) -> Message {
        Message::new(self.role, self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_builder_default() {
        let builder = MessageBuilder::new();
        let msg = builder.build();
        assert_eq!(*msg.role(), Role::User);
        assert!(msg.content().is_empty());
    }

    #[test]
    fn test_message_builder_custom() {
        let builder = MessageBuilder::new()
            .role(Role::Assistant)
            .content("Hello from assistant");
        let msg = builder.build();

        assert_eq!(*msg.role(), Role::Assistant);
        assert_eq!(msg.content(), "Hello from assistant");
    }

    #[test]
    fn test_message_builder_debug_redacts_content() {
        let builder = MessageBuilder::new()
            .role(Role::User)
            .content("secret builder prompt");
        let debug_str = format!("{:?}", builder);
        assert!(!debug_str.contains("secret builder prompt"));
        assert!(debug_str.contains("***REDACTED***"));
    }
}
