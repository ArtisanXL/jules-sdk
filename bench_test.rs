use std::time::Instant;

fn main() {
    let start = Instant::now();
    for _ in 0..10_000_000 {
        let msg = String::from("Hello");
        let _ = msg.clone();
    }
    println!("With clone: {:?}", start.elapsed());

    let start = Instant::now();
    for _ in 0..10_000_000 {
        let msg = String::from("Hello");
        let _ = msg;
    }
    println!("Without clone: {:?}", start.elapsed());
}
