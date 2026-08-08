//! Facade crate for Google's Jules SDK.
//!
//! This crate re-exports the public API from `jules-core` and `jules-api`,
//! as well as procedural macros from `jules-macros`.

#![deny(missing_docs)]

pub use jules_api;
pub use jules_core;

pub use jules_core::config::{Config, ConfigBuilder};
pub use jules_core::conversation::Conversation;
pub use jules_core::session::{Session, SessionBuilder};
pub use jules_core::traits::Client;

pub use jules_macros::Placeholder;

/// Tool calling abstractions and APIs.
///
/// This is only available when the `tools` feature is enabled.
#[cfg(feature = "tools")]
#[cfg_attr(docsrs, doc(cfg(feature = "tools")))]
pub use jules_core::tool::Tool;

/// Experimental features and APIs.
///
/// Items in this module are NOT considered stable and MAY change without notice.
/// This module is only available when the `experimental` feature is enabled.
#[cfg(feature = "experimental")]
#[cfg_attr(docsrs, doc(cfg(feature = "experimental")))]
pub mod experimental {
    // Experimental items will be added here
}
