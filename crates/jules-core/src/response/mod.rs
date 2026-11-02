//! Response module.

use crate::message::Message;
use serde::{Deserialize, Serialize};

/// A response received from a client.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientResponse {
    /// The generated message.
    pub message: Message,
}

impl std::fmt::Debug for ClientResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientResponse")
            .field("message", &self.message)
            .finish()
    }
}

impl ClientResponse {
    /// Creates a new `ClientResponse` with the given message.
    #[must_use]
    pub fn new(message: Message) -> Self {
        Self { message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::Role;

    #[test]
    fn test_client_response_new() {
        let msg = Message::new(Role::Assistant, "Response");
        let resp = ClientResponse::new(msg.clone());
        assert_eq!(resp.message, msg);
    }
}
