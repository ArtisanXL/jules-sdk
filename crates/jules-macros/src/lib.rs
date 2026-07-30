//! Procedural macros for Jules-SDK.

#![deny(missing_docs)]

mod builder;
mod derive;
mod tool;
mod validation;

use proc_macro::TokenStream;

/// A placeholder derive macro.
#[proc_macro_derive(Placeholder)]
pub fn placeholder_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
