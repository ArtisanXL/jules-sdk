//! Facade crate for Google's Jules SDK.
//!
//! This crate re-exports the public API from `jules-core` and `jules-api`,
//! as well as procedural macros from `jules-macros`.

#![deny(missing_docs)]

pub use jules_api;
pub use jules_core;

pub use jules_core::session::{Session, SessionBuilder};

pub use jules_macros::Placeholder;
