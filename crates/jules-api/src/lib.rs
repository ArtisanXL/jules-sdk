//! API integrations and client implementation for Jules-SDK.

#![deny(missing_docs)]

pub mod auth;
pub mod client;

/// Re-exports of [`client::JulesClient`] and [`client::JulesClientBuilder`], the real,
/// network-capable client. Only available when the `middleware` feature is enabled (the
/// client relies on `jules_core`'s middleware pipeline) and on non-wasm targets (it relies on
/// [`http::reqwest_transport::ReqwestTransport`]).
#[cfg(all(feature = "middleware", not(target_arch = "wasm32")))]
pub use client::{JulesClient, JulesClientBuilder};

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

/// Experimental features and APIs.
///
/// Items in this module are NOT considered stable and MAY change without notice.
/// This module is only available when the `experimental` feature is enabled.
#[cfg(feature = "experimental")]
#[cfg_attr(docsrs, doc(cfg(feature = "experimental")))]
pub mod experimental {
    // Experimental items will be added here
}
