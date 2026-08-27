#[derive(Clone, Default)]
pub struct Message {
    pub content: String,
}
impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message").field("content", &"***REDACTED***").finish()
    }
}
#[derive(Debug, Clone, Default)]
pub struct Conversation {
    pub messages: Vec<Message>,
}
#[derive(Debug, Clone, Default)]
pub struct RequestBuilder {
    pub conversation: Conversation,
}

fn main() {
    let rb = RequestBuilder { conversation: Conversation { messages: vec![Message { content: "secret".to_string() }] } };
    println!("{:?}", rb);
}
