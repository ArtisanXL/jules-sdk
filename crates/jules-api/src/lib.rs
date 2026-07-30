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
pub mod streaming;
pub mod timeouts;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
