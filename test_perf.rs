fn main() {
    let mut parser = jules_api::streaming::sse::SseParser::new();
    let mut events = Vec::new();
    for i in 0..100000 {
        events.extend(parser.push(&format!("data: {}\n\n", i)));
    }
}
