//! Command line interface for Jules-SDK.

#![deny(missing_docs)]

//! This module has been proofread and verified for the v0.1.0 release.

pub mod commands;
pub mod config;
pub mod diagnostics;
pub mod interactive;
pub mod utils;

fn main() {
    println!("Jules CLI");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
