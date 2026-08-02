#[cfg(feature = "middleware")]
use jules_sdk::jules_core::client::ClientRequest;
#[cfg(feature = "middleware")]
use jules_sdk::jules_core::conversation::Conversation;
#[cfg(feature = "middleware")]
use jules_sdk::jules_core::errors::SDKError;
#[cfg(feature = "middleware")]
use jules_sdk::jules_core::message::{Message, Role};
#[cfg(feature = "middleware")]
use jules_sdk::jules_core::middleware::{BoxFuture, Middleware, MiddlewarePipeline, NextFn};
#[cfg(feature = "middleware")]
use jules_sdk::jules_core::response::ClientResponse;
#[cfg(feature = "middleware")]
use std::sync::atomic::{AtomicUsize, Ordering};
#[cfg(feature = "middleware")]
use std::sync::Arc;

#[cfg(feature = "middleware")]
struct CounterMiddleware {
    count: Arc<AtomicUsize>,
}

#[cfg(feature = "middleware")]
impl Middleware for CounterMiddleware {
    fn execute<'a>(
        &'a self,
        request: ClientRequest,
        next: NextFn<'a>,
    ) -> BoxFuture<'a, Result<ClientResponse, SDKError>> {
        self.count.fetch_add(1, Ordering::SeqCst);
        println!("Request passing through middleware.");

        Box::pin(async move {
            let mut response = next(request).await?;
            let old_content = response.message.content().to_string();
            response.message = Message::new(
                response.message.role().clone(),
                format!("{} (intercepted)", old_content),
            );
            Ok(response)
        })
    }
}

#[cfg(feature = "middleware")]
fn main() {
    let mut pipeline = MiddlewarePipeline::new();
    let count = Arc::new(AtomicUsize::new(0));

    pipeline.add(CounterMiddleware {
        count: Arc::clone(&count),
    });

    let request = ClientRequest::new(Conversation::new());

    let final_handler = |_req: ClientRequest| async move {
        println!("Handling request in final handler.");
        Ok(ClientResponse::new(Message::new(Role::Assistant, "Hello")))
    };

    let f = async {
        let result = pipeline.execute(request, final_handler).await.unwrap();
        println!("Result: {}", result.message.content());
        println!(
            "Middleware execution count: {}",
            count.load(Ordering::SeqCst)
        );
    };

    // Poor man's block_on
    let waker = std::task::Waker::noop();
    let mut cx = std::task::Context::from_waker(&waker);
    let mut future = std::boxed::Box::pin(f);
    let mut iters = 0;
    while std::future::Future::poll(future.as_mut(), &mut cx).is_pending() {
        iters += 1;
        if iters > 100 {
            panic!("future didn't resolve");
        }
    }
}

#[cfg(not(feature = "middleware"))]
fn main() {
    println!("Please enable the `middleware` feature to run this example.");
}
