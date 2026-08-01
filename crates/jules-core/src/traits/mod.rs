//! Traits module for Jules-SDK core abstractions.

use crate::client::ClientRequest;
use crate::errors::SDKError;
use crate::response::ClientResponse;
use std::future::Future;

/// The core Client trait defining standard interactions with the API.
///
/// It encapsulates sending a request and asynchronously waiting for a response.
pub trait Client {
    /// Sends a request and receives a response asynchronously.
    fn send_request(
        &self,
        request: ClientRequest,
    ) -> impl Future<Output = Result<ClientResponse, SDKError>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversation::Conversation;
    use crate::message::{Message, Role};

    struct MockClient;

    impl Client for MockClient {
        fn send_request(
            &self,
            _request: ClientRequest,
        ) -> impl Future<Output = Result<ClientResponse, SDKError>> + Send {
            async {
                Ok(ClientResponse::new(Message::new(
                    Role::Assistant,
                    "Mock response",
                )))
            }
        }
    }

    #[tokio::test]
    async fn test_mock_client_send_request() {
        let client = MockClient;
        let mut conv = Conversation::new();
        conv.add_message(Message::new(Role::User, "Hello"));
        let req = ClientRequest::new(conv);
        let response = client.send_request(req).await.unwrap();
        assert_eq!(
            response.message,
            Message::new(Role::Assistant, "Mock response")
        );
    }
}
