# Platform Support & Caveats

Jules-SDK aims to be cross-platform, natively supporting various desktop OSes as well as WebAssembly.

## Supported Native Platforms

* **Linux (x86_64, aarch64)** - Fully supported, testing via CI.
* **macOS (x86_64, aarch64)** - Fully supported.
* **Windows (x86_64)** - Fully supported.

## WebAssembly (wasm32-unknown-unknown)

Jules-SDK supports `wasm32-unknown-unknown` compilation to enable usage in the browser and serverless edge environments (e.g. Cloudflare Workers).

### WASM Caveats & Limitations

1.  **Network Transport:** When compiled for `wasm32`, native sockets and asynchronous runtimes like `tokio`'s I/O reactor are not available.
    *   Instead, Jules-SDK utilizes `web-sys` and `js-sys` to bind to the browser's native `fetch` API.
    *   The `jules_api::wasm::client::FetchClient` handles HTTP traffic internally when targeting `wasm32`.
2.  **Concurrency:** WASM is inherently single-threaded in most browser contexts unless web workers and `SharedArrayBuffer` are specifically configured.
    *   You should avoid using functions or operations that might block the main thread.
    *   `Send` and `Sync` bounds on futures and types are generally relaxed or unneeded strictly for WASM execution, but we maintain them in our traits for native consistency. Be aware when integrating with standard Javascript `Promise` logic.
3.  **Feature Flags:** The `wasm` feature flag explicitly enables the WASM bindings (`jules-core/wasm`, `jules-api/wasm`). This is activated automatically based on the target architecture through `cfg(target_arch = "wasm32")` configuration in Cargo.toml.
4.  **Testing:** Testing WASM bindings requires `wasm-bindgen-test` rather than the standard `#[test]` attribute.

## Tokio Dependency

Jules-SDK abstracts its core asynchronous execution, however some advanced features or native I/O (like file system tools) heavily lean on Tokio. When building for WASM, ensure `tokio` (if used directly in your own code) is configured appropriately without native time or net features enabled.
