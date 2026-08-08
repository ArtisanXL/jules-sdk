//! Proves the facade crate's top-level re-exports actually resolve through `jules_sdk::`, not
//! just through the underlying `jules_core`/`jules_api` crates directly. These symbols are
//! otherwise only referenced from `examples/`, which CI does not compile as part of
//! `cargo build`/`cargo check`/`cargo nextest run --workspace`, so a broken `cfg` gate or a
//! typo'd re-export path would previously go undetected.
#![cfg(all(feature = "middleware", not(target_arch = "wasm32")))]

use jules_sdk::{JulesClient, JulesClientBuilder};

#[test]
fn test_jules_client_reachable_via_facade() {
    let client: Result<JulesClient, _> = JulesClientBuilder::new()
        .base_url("https://api.example.com")
        .build();

    assert!(client.is_ok());
}
