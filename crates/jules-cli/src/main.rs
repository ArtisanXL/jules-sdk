//! Command line interface for Jules-SDK.

#![deny(missing_docs)]

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
