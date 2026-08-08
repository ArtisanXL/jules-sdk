//! `chat` subcommand: build a local conversation from a single message.
//!
//! This command operates entirely locally/offline: it builds a
//! [`Conversation`] and appends the given text as a [`Role::User`]
//! [`Message`], then renders the resulting conversation state. Live network
//! execution against the Jules API is not yet wired up here — that is a
//! natural follow-up once the v1alpha REST client lands in `jules-api`.

use clap::Args;
use jules_sdk::jules_core::conversation::Conversation;
use jules_sdk::jules_core::message::{Message, Role};
use serde::Serialize;

use crate::utils::{OutputFormat, Render};

/// Arguments for the `chat` subcommand.
#[derive(Debug, Args)]
pub struct ChatArgs {
    /// The message to append to the conversation, as the user.
    pub message: String,
}

/// A single rendered message within a [`ChatResult`].
#[derive(Debug, Serialize)]
pub struct ChatMessageView {
    /// The role of the message sender (e.g. `"user"`).
    pub role: String,
    /// The message content.
    pub content: String,
}

/// The rendered result of a `chat` subcommand invocation.
#[derive(Debug, Serialize)]
pub struct ChatResult {
    /// The messages currently in the conversation.
    pub messages: Vec<ChatMessageView>,
}

impl Render for ChatResult {
    fn render_plain(&self) -> String {
        self.messages
            .iter()
            .map(|m| format!("{}: {}", m.role, m.content))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Handles the `chat` subcommand.
///
/// Builds a [`Conversation`] locally and appends `args.message` as a
/// [`Role::User`] message. This performs no network I/O: live execution
/// against the Jules API is not yet wired up (see the module documentation).
///
/// # Errors
///
/// Returns an error if rendering the result as JSON fails.
pub fn handle(args: &ChatArgs, format: OutputFormat) -> Result<String, serde_json::Error> {
    let mut conversation = Conversation::new();
    conversation.add_message(Message::new(Role::User, args.message.clone()));

    let messages = conversation
        .messages()
        .iter()
        .map(|message| ChatMessageView {
            // `Role`'s `Debug` output (`User`, `System`, ...) lowercased
            // matches its `serde(rename_all = "lowercase")` wire format.
            role: format!("{:?}", message.role()).to_lowercase(),
            content: message.content().to_string(),
        })
        .collect();

    ChatResult { messages }.render(format)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_single_user_message() {
        let args = ChatArgs {
            message: "hello".to_string(),
        };
        let output = handle(&args, OutputFormat::Plain).unwrap();
        assert_eq!(output, "user: hello");
    }

    #[test]
    fn renders_json_with_role_and_content() {
        let args = ChatArgs {
            message: "hi there".to_string(),
        };
        let output = handle(&args, OutputFormat::Json).unwrap();
        assert!(output.contains("\"role\": \"user\""));
        assert!(output.contains("\"content\": \"hi there\""));
    }
}
