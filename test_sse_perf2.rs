#[derive(Debug)]
struct SseEvent {
    data: String,
}

fn parse_block(block: &str) -> Option<SseEvent> {
    if block.is_empty() { return None; }
    Some(SseEvent { data: block.to_string() })
}

fn parse(buffer: &mut String) -> Vec<SseEvent> {
    let mut events = Vec::new();
    while let Some(pos) = buffer.find("\n\n") {
        let block = buffer[..pos].to_string();
        *buffer = buffer[pos + 2..].to_string();
        if let Some(event) = parse_block(&block) {
            events.push(event);
        }
    }
    events
}

fn parse_optimized(buffer: &mut String) -> Vec<SseEvent> {
    let mut events = Vec::new();
    let mut last_pos = 0;
    while let Some(pos) = buffer[last_pos..].find("\n\n") {
        let abs_pos = last_pos + pos;
        let block = &buffer[last_pos..abs_pos];
        if let Some(event) = parse_block(block) {
            events.push(event);
        }
        last_pos = abs_pos + 2;
    }
    if last_pos > 0 {
        buffer.drain(..last_pos);
    }
    events
}

fn main() {
    let mut buf = String::new();
    for i in 0..50000 {
        buf.push_str(&format!("data: {}\n\n", i));
    }
    let mut buf_opt = buf.clone();

    let t0 = std::time::Instant::now();
    let events1 = parse(&mut buf);
    println!("Unoptimized: {:?}", t0.elapsed());

    let t1 = std::time::Instant::now();
    let events2 = parse_optimized(&mut buf_opt);
    println!("Optimized: {:?}", t1.elapsed());

    assert_eq!(events1.len(), events2.len());
    assert_eq!(buf, buf_opt);
}
