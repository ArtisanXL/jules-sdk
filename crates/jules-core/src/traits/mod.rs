//! Traits module for Jules-SDK core abstractions.

use crate::errors::SDKError;
use crate::message::Message;
use std::future::Future;

/// The core Client trait defining standard interactions with the API.
pub trait Client {
    /// The response type returned by the client.
    type Response;

    /// Sends a message and receives a response asynchronously.
    fn send_message(
        &self,
        message: Message,
    ) -> impl Future<Output = Result<Self::Response, SDKError>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::Role;

    struct MockClient;

    impl Client for MockClient {
        type Response = String;

        fn send_message(
            &self,
            _message: Message,
        ) -> impl Future<Output = Result<Self::Response, SDKError>> + Send {
            async { Ok("Mock response".to_string()) }
        }
    }

    #[tokio::test]
    async fn test_mock_client_send_message() {
        let client = MockClient;
        let message = Message::new(Role::User, "Hello");
        let response = client.send_message(message).await.unwrap();
        assert_eq!(response, "Mock response");
    }
}
