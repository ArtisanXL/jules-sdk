# jules-macros

> Procedural macros for Jules-SDK.

[![Crates.io](https://img.shields.io/crates/v/jules-macros)](https://crates.io/crates/jules-macros)
[![Docs.rs](https://img.shields.io/docsrs/jules-macros)](https://docs.rs/jules-macros)
[![License](https://img.shields.io/crates/l/jules-macros)](../../LICENSE)

> **This crate is an internal implementation detail of [`jules-sdk`](../jules-sdk).** Users should never depend on `jules-macros` directly — access it through `jules-sdk`'s re-exports.

## Status: not yet functional

This crate is scaffolding for future proc-macro support (builder derives, tool-definition derives, request/response validation). **None of that exists yet.** Today it exports exactly one macro:

```rust
use jules_macros::Placeholder;

#[derive(Placeholder)]
struct Anything;
```

`#[derive(Placeholder)]` is a no-op — it generates no code and exists only to prove the proc-macro crate wiring (crate type, workspace dependency, re-export through `jules-sdk`) works end-to-end. Its `builder`, `derive`, `tool`, and `validation` internal modules are currently empty stubs.

Do not build against this crate expecting derive macros for sessions, tools, or validation — check [PROJECT_STATE.md](../../PROJECT_STATE.md) and [ROADMAP.md](../../ROADMAP.md) for when real macros land.

## Installation

```toml
[dependencies]
jules-macros = "0.1"
```

In practice you get this transitively through [`jules-sdk`](../jules-sdk), which re-exports `Placeholder`.

## More

* [jules-sdk](../jules-sdk) — the facade crate most users should depend on instead
* [Root README](../../README.md) · [PROJECT_STATE.md](../../PROJECT_STATE.md) · [ROADMAP.md](../../ROADMAP.md)

## License

Dual-licensed under [MIT](../../LICENSE-MIT) or [Apache-2.0](../../LICENSE-APACHE), at your option.
