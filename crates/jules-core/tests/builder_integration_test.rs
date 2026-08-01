use jules_core::builder::{ClientBuilder, ConversationBuilder, MessageBuilder, RequestBuilder};
use jules_core::message::Role;
use std::time::Duration;

#[test]
fn test_builder_integration() {
    let _client = ClientBuilder::new()
        .base_url("https://api.example.com")
        .auth_token("secret")
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let message1 = MessageBuilder::new()
        .role(Role::System)
        .content("You are a helpful assistant.")
        .build();

    let message2 = MessageBuilder::new()
        .role(Role::User)
        .content("What is the capital of France?")
        .build();

    let conversation = ConversationBuilder::new()
        .add_messages(vec![message1, message2])
        .build();

    let request = RequestBuilder::new()
        .conversation(conversation)
        .add_message(
            MessageBuilder::new()
                .role(Role::User)
                .content("And Germany?")
                .build(),
        )
        .build();

    assert_eq!(request.conversation.messages().len(), 3);
    assert_eq!(
        request.conversation.messages()[0].content(),
        "You are a helpful assistant."
    );
    assert_eq!(
        request.conversation.messages()[1].content(),
        "What is the capital of France?"
    );
    assert_eq!(request.conversation.messages()[2].content(), "And Germany?");
}
