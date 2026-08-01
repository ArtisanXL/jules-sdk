//! Logging middleware for Jules-SDK.
//!
//! Intercepts requests and responses to log details using the `tracing` crate.

use super::{BoxFuture, Middleware, NextFn};
use crate::client::ClientRequest;
use crate::errors::SDKError;
use crate::response::ClientResponse;
use tracing::{error, info, trace};

/// A middleware that logs outgoing requests and incoming responses.
#[derive(Debug, Clone, Default)]
pub struct LoggingMiddleware;

impl LoggingMiddleware {
    /// Creates a new `LoggingMiddleware`.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Middleware for LoggingMiddleware {
    fn execute<'a>(
        &'a self,
        request: ClientRequest,
        next: NextFn<'a>,
    ) -> BoxFuture<'a, Result<ClientResponse, SDKError>> {
        Box::pin(async move {
            let model = "default".to_string();
            let message_count = request.conversation.messages().len();

            info!(
                model = %model,
                messages = message_count,
                "Sending request"
            );

            trace!(?request, "Full request details");

            let start = std::time::Instant::now();
            let result = next(request).await;
            let duration = start.elapsed();

            match &result {
                Ok(response) => {
                    info!(
                        duration_ms = duration.as_millis(),
                        role = ?response.message.role(),
                        "Received response"
                    );
                    trace!(?response, "Full response details");
                }
                Err(e) => {
                    error!(
                        duration_ms = duration.as_millis(),
                        error = %e,
                        "Request failed"
                    );
                }
            }

            result
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversation::Conversation;
    use crate::message::{Message, Role};
    use crate::middleware::MiddlewarePipeline;

    #[tokio::test]
    async fn test_logging_middleware() {
        let mut pipeline = MiddlewarePipeline::new();
        pipeline.add(LoggingMiddleware::new());

        let request = ClientRequest::new(Conversation::new());

        let handler = |_req: ClientRequest| async move {
            Ok(ClientResponse::new(Message::new(
                Role::Assistant,
                "Success",
            )))
        };

        // This test mostly verifies it compiles and runs without panicking.
        // Capturing tracing output in tests requires tracing-subscriber setup,
        // which we'll skip for a simple unit test.
        let res = pipeline.execute(request, handler).await.unwrap();
        assert_eq!(res.message.content(), "Success");
    }
}
