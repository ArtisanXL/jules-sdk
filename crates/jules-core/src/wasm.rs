//! WASM integration module.

#[cfg(target_arch = "wasm32")]
/// WASM environment abstractions.
pub mod environment {
    /// Indicates whether the code is running in a WASM environment.
    pub const IS_WASM: bool = true;
}

#[cfg(not(target_arch = "wasm32"))]
/// WASM environment abstractions.
pub mod environment {
    /// Indicates whether the code is running in a WASM environment.
    pub const IS_WASM: bool = false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_wasm_constant() {
        let is_wasm = environment::IS_WASM;

        #[cfg(target_arch = "wasm32")]
        assert!(is_wasm, "IS_WASM should be true on wasm32");

        #[cfg(not(target_arch = "wasm32"))]
        assert!(!is_wasm, "IS_WASM should be false on non-wasm32 targets");
    }
}
