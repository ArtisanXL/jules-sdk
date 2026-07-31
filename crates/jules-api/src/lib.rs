//! API integrations and client implementation for Jules-SDK.

#![deny(missing_docs)]

pub mod auth;
pub mod client;
pub mod conversation;
pub mod errors;
pub mod http;
pub mod response;
pub mod retry;
pub mod session;
/// Streaming API abstractions and response handling.
/// This module is only available when the `streaming` feature is enabled.
#[cfg(feature = "streaming")]
pub mod streaming;
pub mod timeouts;

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[cfg(feature = "streaming")]
    fn test_streaming_feature_compiles() {
        // Ensure the streaming module is accessible when the feature is enabled.
        #[allow(unused_imports)]
        use crate::streaming;
    }
}
