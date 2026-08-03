use std::time::Instant;

#[derive(Clone, PartialEq, Eq)]
pub enum Role {
    User,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Message {
    role: Role,
    content: String,
}

impl Message {
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }
}

pub struct Conversation {
    messages: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Self {
        Self { messages: Vec::new() }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}

fn main() {
    let iters = 1_000_000;

    // Benchmark 1: With clone
    let start = Instant::now();
    let mut conv = Conversation::new();
    let msg = Message::new(Role::User, "Hello world this is a test string to be cloned");
    for _ in 0..iters {
        conv.add_message(msg.clone());
    }
    let duration_with_clone = start.elapsed();
    println!("With clone: {:?}", duration_with_clone);

    // Benchmark 2: Without clone (just moving, though in loop we need to recreate if we don't clone. Wait, actually we can just recreate)
    let start = Instant::now();
    let mut conv2 = Conversation::new();
    for _ in 0..iters {
        let msg = Message::new(Role::User, "Hello world this is a test string to be cloned");
        conv2.add_message(msg);
    }
    let duration_without_clone = start.elapsed();
    println!("Without clone (recreating instead): {:?}", duration_without_clone);
}
