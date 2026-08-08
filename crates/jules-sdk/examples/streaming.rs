use jules_sdk::jules_core::streaming::{Stream, StreamEvent};
use std::future::Future;

struct DummyStream {
    count: u32,
}

impl Stream for DummyStream {
    type Item = StreamEvent;

    fn next(&mut self) -> impl Future<Output = Option<Self::Item>> + Send {
        let res = match self.count {
            0..=2 => {
                self.count += 1;
                Some(StreamEvent::TextChunk(format!("Chunk {}\n", self.count)))
            }
            3 => {
                self.count += 1;
                Some(StreamEvent::Done)
            }
            _ => None,
        };
        async move { res }
    }
}

fn main() {
    println!("Starting stream...");
    let mut stream = DummyStream { count: 0 };

    let f = async {
        while let Some(event) = stream.next().await {
            match event {
                StreamEvent::TextChunk(text) => {
                    print!("{text}");
                }
                StreamEvent::Done => {
                    println!("Stream finished.");
                    break;
                }
            }
        }
    };

    // Poor man's block_on
    let waker = std::task::Waker::noop();
    let mut cx = std::task::Context::from_waker(waker);
    let mut future = std::boxed::Box::pin(f);
    let mut iters = 0;
    while std::future::Future::poll(future.as_mut(), &mut cx).is_pending() {
        iters += 1;
        assert!(iters <= 100, "future didn't resolve");
    }
}
