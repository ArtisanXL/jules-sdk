use jules_api::http::{HttpRequest, HttpResponse, Method, Transport};
use jules_core::errors::SDKError;
use std::future::Future;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

struct MockTransport {
    counter: Arc<AtomicUsize>,
}

impl Transport for MockTransport {
    fn send(
        &self,
        _request: HttpRequest,
    ) -> impl Future<Output = Result<HttpResponse, SDKError>> + Send {
        self.counter.fetch_add(1, Ordering::SeqCst);
        async move {
            Ok(HttpResponse::new(
                200,
                vec![("Content-Type".into(), "application/json".into())],
                b"{}".to_vec(),
            ))
        }
    }
}

#[tokio::test]
async fn test_client_load() {
    let counter = Arc::new(AtomicUsize::new(0));
    let transport = Arc::new(MockTransport {
        counter: counter.clone(),
    });

    let mut handles = Vec::new();

    // 100 concurrent tasks, each making 100 requests (10,000 total)
    for _ in 0..100 {
        let t = transport.clone();
        handles.push(tokio::spawn(async move {
            for _ in 0..100 {
                let req = HttpRequest::new(Method::Get, "https://api.example.com");
                let _ = t.send(req).await;
            }
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    assert_eq!(counter.load(Ordering::SeqCst), 10_000);
}
