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
}
