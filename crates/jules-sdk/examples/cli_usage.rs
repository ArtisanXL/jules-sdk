#[cfg(feature = "cli")]
fn main() {
    println!("Simulating CLI initialization with Jules SDK.");

    // As jules-cli is a separate crate and binary logic,
    // we just demonstrate a conceptual configuration load here as part of an SDK usage example,
    // reflecting how a CLI tool might consume the SDK configuration.

    // In a real usage scenario, the CLI parses args and builds the SDK Config.
    let api_key_from_env =
        std::env::var("JULES_API_KEY").unwrap_or_else(|_| "mock_key_from_cli".to_string());

    let config = jules_sdk::Config::builder()
        .api_key(&api_key_from_env)
        .timeout(30)
        .build()
        .expect("Failed to build config");

    println!("CLI initialized with API Key: {}", config.api_key());
    println!("CLI initialized with Timeout: {:?}", config.timeout());

    let _session = jules_sdk::Session::builder().build();

    println!("Session ready to be used by CLI commands.");
}

#[cfg(not(feature = "cli"))]
fn main() {
    println!("Please enable the `cli` feature to run this example.");
}
