//! Core traits and abstractions for Jules-SDK.

#![deny(missing_docs)]

pub mod builder;
pub mod client;
pub mod config;
pub mod conversation;
pub mod errors;
pub mod message;
pub mod pagination;
pub mod response;
pub mod session;
/// Streaming APIs, stream event handling, async stream abstractions, incremental responses.
#[cfg(feature = "streaming")]
pub mod streaming;
/// Tool calling abstractions and APIs.
/// This module is only available when the `tools` feature is enabled.
#[cfg(feature = "tools")]
pub mod tool;
pub mod traits;

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[cfg(feature = "tools")]
    fn test_tools_feature_compiles() {
        // Use a construct that proves `tool` module is loaded when `tools` feature is enabled.
        #[allow(unused_imports)]
        use crate::tool;
    }
}
