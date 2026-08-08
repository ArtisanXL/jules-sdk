//! Proves the facade crate's top-level re-exports actually resolve through `jules_sdk::`, not
//! just through the underlying `jules_core`/`jules_api` crates directly. These symbols are
//! otherwise only referenced from `examples/`, which CI does not compile as part of
//! `cargo build`/`cargo check`/`cargo nextest run --workspace`, so a broken `cfg` gate or a
//! typo'd re-export path would previously go undetected.

#[cfg(all(feature = "middleware", not(target_arch = "wasm32")))]
#[test]
fn test_jules_client_reachable_via_facade() {
    use jules_sdk::{JulesClient, JulesClientBuilder};

    let client: Result<JulesClient, _> = JulesClientBuilder::new()
        .base_url("https://api.example.com")
        .build();

    assert!(client.is_ok());
}

#[cfg(feature = "tools")]
struct EchoTool;

#[cfg(feature = "tools")]
impl jules_sdk::Tool for EchoTool {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn description(&self) -> &'static str {
        "Echoes the input."
    }

    fn parameters(&self) -> jules_sdk::jules_core::tool::ToolParameters {
        jules_sdk::jules_core::tool::ToolParameters {
            properties: std::collections::HashMap::new(),
            required: Vec::new(),
        }
    }

    fn call(
        &self,
        args: &str,
    ) -> impl std::future::Future<Output = Result<String, jules_sdk::jules_core::errors::ToolError>> + Send
    {
        let args = args.to_string();
        async move { Ok(args) }
    }
}

#[cfg(feature = "tools")]
#[test]
fn test_tool_reachable_via_facade() {
    use jules_sdk::Tool;
    assert_eq!(EchoTool.name(), "echo");
}

struct DummyClient;

impl jules_sdk::Client for DummyClient {
    async fn send_request(
        &self,
        _request: jules_sdk::jules_core::client::ClientRequest,
    ) -> Result<
        jules_sdk::jules_core::response::ClientResponse,
        jules_sdk::jules_core::errors::SDKError,
    > {
        unimplemented!("only used to prove the Client trait resolves via the facade")
    }
}

fn assert_client_trait<T: jules_sdk::Client>() {}

/// Locks in that `jules_sdk::experimental` resolves under the `experimental` feature flag, so a
/// broken `cfg` gate is caught even while the module itself is still empty. The import is the
/// assertion: this test's only job is to fail to compile if the path stops resolving.
#[cfg(feature = "experimental")]
#[allow(unused_imports)]
use jules_sdk::experimental as _;

#[cfg(feature = "experimental")]
#[test]
fn test_experimental_module_reachable_via_facade() {}

#[test]
fn test_core_types_reachable_via_facade() {
    let config = jules_sdk::Config::builder()
        .api_key("test-key")
        .build()
        .unwrap();
    assert_eq!(config.api_key(), "test-key");

    let session = jules_sdk::Session::builder()
        .name("sessions/1")
        .build()
        .unwrap();
    assert_eq!(session.name(), Some("sessions/1"));

    assert_client_trait::<DummyClient>();
}
