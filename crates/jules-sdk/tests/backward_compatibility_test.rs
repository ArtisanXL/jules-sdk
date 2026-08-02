use jules_sdk::jules_core::builder::ClientBuilder;

#[test]
fn test_v0_1_0_api_compatibility() {
    let builder = ClientBuilder::new()
        .base_url("https://api.example.com")
        .auth_token("test-token");

    // Test that the builder can still be built without issues, preserving backward compatibility of the core feature.
    assert!(builder.build().is_ok());
}
