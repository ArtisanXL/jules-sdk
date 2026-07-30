//! Procedural macros for Jules-SDK.

#![deny(missing_docs)]

use proc_macro::TokenStream;

/// A placeholder derive macro.
#[proc_macro_derive(Placeholder)]
pub fn placeholder_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
