//! Middleware abstractions and pipeline for Jules-SDK.
//!
//! Middlewares allow you to intercept, observe, or modify outgoing requests and incoming responses.

use crate::client::ClientRequest;
use crate::errors::SDKError;
use crate::response::ClientResponse;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// A pinned, boxed future that implements `Send`.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// A function type that represents the execution of the next middleware in the pipeline.
///
/// This is `Fn` (not `FnOnce`) so that middlewares such as `RetryMiddleware` can invoke
/// `next` more than once to retry a request.
pub type NextFn<'a> = Box<
    dyn Fn(ClientRequest) -> BoxFuture<'a, Result<ClientResponse, SDKError>> + Send + Sync + 'a,
>;

/// A trait for intercepting and modifying requests and responses.
///
/// Implementations of this trait can be added to a `MiddlewarePipeline`.
pub trait Middleware: Send + Sync {
    /// Executes the middleware.
    ///
    /// The middleware is responsible for calling the `next` function to proceed down the pipeline,
    /// passing the potentially modified `request`. It receives a `Future` of the `ClientResponse`,
    /// which it can await and then potentially modify before returning.
    fn execute<'a>(
        &'a self,
        request: ClientRequest,
        next: NextFn<'a>,
    ) -> BoxFuture<'a, Result<ClientResponse, SDKError>>;
}

/// A pipeline of middlewares executed in order.
pub struct MiddlewarePipeline {
    middlewares: Vec<Arc<dyn Middleware>>,
}

impl Default for MiddlewarePipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl MiddlewarePipeline {
    /// Creates a new, empty middleware pipeline.
    #[must_use]
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
        }
    }

    /// Adds a middleware to the end of the pipeline.
    pub fn add<M: Middleware + 'static>(&mut self, middleware: M) {
        self.middlewares.push(Arc::new(middleware));
    }

    /// Returns the number of middlewares in the pipeline.
    #[must_use]
    pub fn len(&self) -> usize {
        self.middlewares.len()
    }

    /// Returns `true` if the pipeline contains no middlewares.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.middlewares.is_empty()
    }

    /// Executes the pipeline with the given request and a final handler.
    ///
    /// The final handler is typically the actual network call.
    #[allow(clippy::missing_errors_doc)]
    pub async fn execute<F, Fut>(
        &self,
        request: ClientRequest,
        final_handler: F,
    ) -> Result<ClientResponse, SDKError>
    where
        F: Fn(ClientRequest) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<ClientResponse, SDKError>> + Send + 'static,
    {
        let mut next: NextFn<'_> = Box::new(move |req| Box::pin(final_handler(req)));
        for middleware in self.middlewares.iter().rev() {
            // `next` must remain callable more than once (it is now `Fn`, not `FnOnce`) so that
            // a middleware further up the chain (e.g. `RetryMiddleware`) can invoke it multiple
            // times. Since `NextFn` is passed by value into `Middleware::execute`, we share the
            // previous `next` behind an `Arc` and hand each invocation a fresh `NextFn` wrapper
            // that delegates through it.
            let next_shared: Arc<NextFn<'_>> = Arc::new(next);
            // Capture a reference to the middleware instead of cloning the Arc.
            // This is safe because `self` outlives the `NextFn` closures and `execute` call.
            let m: &dyn Middleware = &**middleware;
            next = Box::new(move |req| {
                let next_shared = Arc::clone(&next_shared);
                m.execute(req, Box::new(move |req2| next_shared(req2)))
            });
        }
        next(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversation::Conversation;
    use crate::message::{Message, Role};
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TestMiddleware {
        calls: Arc<AtomicUsize>,
    }

    impl Middleware for TestMiddleware {
        fn execute<'a>(
            &'a self,
            request: ClientRequest,
            next: NextFn<'a>,
        ) -> BoxFuture<'a, Result<ClientResponse, SDKError>> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Box::pin(async move {
                let mut res = next(request).await?;
                // Modify the response string to prove we intercepted it
                let mut new_msg = res.message.clone();
                let mut content = new_msg.content().to_string();
                content.push_str(" (modified)");
                new_msg = crate::message::Message::new(new_msg.role().clone(), content);
                res.message = new_msg;
                Ok(res)
            })
        }
    }

    #[tokio::test]
    async fn test_middleware_pipeline() {
        let mut pipeline = MiddlewarePipeline::new();
        let calls = Arc::new(AtomicUsize::new(0));

        pipeline.add(TestMiddleware {
            calls: Arc::clone(&calls),
        });

        let request = ClientRequest::new(Conversation::new());

        let handler = |_req: ClientRequest| async move {
            Ok(ClientResponse::new(Message::new(
                Role::Assistant,
                "Original",
            )))
        };

        let res = pipeline.execute(request, handler).await.unwrap();

        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(res.message.content(), "Original (modified)");
    }
}
pub mod logging;
pub mod retry;
