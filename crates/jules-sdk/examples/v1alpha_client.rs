//! Demonstrates the real, network-capable `v1alpha` session/source client.
//!
//! Unlike this SDK's other examples, this one talks to the live Jules API
//! (`https://jules.googleapis.com`) — `JulesClient` wraps a real `reqwest`-backed transport
//! rather than a pluggable mock, so there's no fully-offline way to exercise it. Set a real
//! `JULES_API_KEY` environment variable to actually run it; otherwise it prints instructions
//! and exits. Only read-only calls are made (`list_sessions`, `list_sources`), so it's safe to
//! run against a real account.

#[cfg(all(feature = "middleware", not(target_arch = "wasm32")))]
#[tokio::main]
async fn main() {
    use jules_sdk::jules_api::auth::AuthType;
    use jules_sdk::JulesClientBuilder;

    let Ok(api_key) = std::env::var("JULES_API_KEY") else {
        println!("Set the JULES_API_KEY environment variable to run this example against the");
        println!("real Jules API. It only makes read-only calls (list_sessions, list_sources).");
        return;
    };

    let client = JulesClientBuilder::new()
        .auth(AuthType::google_api_key(api_key))
        .build()
        .expect("failed to build JulesClient");

    println!("Listing up to 5 sessions...");
    match client.list_sessions(Some(5), None).await {
        Ok(page) => {
            for session in page.items() {
                println!(
                    "  {}  {}  {}",
                    session.id().unwrap_or("-"),
                    session.state().unwrap_or("-"),
                    session.title().unwrap_or("-"),
                );
            }
            if page.items().is_empty() {
                println!("  (no sessions found)");
            }
        }
        Err(err) => println!("  failed to list sessions: {err}"),
    }

    println!("Listing up to 5 sources...");
    match client.list_sources(Some(5), None).await {
        Ok(page) => {
            for source in page.items() {
                println!("  {}", source.name().unwrap_or("-"));
            }
            if page.items().is_empty() {
                println!("  (no sources found)");
            }
        }
        Err(err) => println!("  failed to list sources: {err}"),
    }
}

#[cfg(not(all(feature = "middleware", not(target_arch = "wasm32"))))]
fn main() {
    println!("Please enable the `middleware` feature (and build for a non-wasm32 target) to run this example.");
}
