入口：[Rust Users Forum](https://users.rust-lang.org/)。

分类：`announcements`。本帖以项目介绍为主；后续确有具体 API 审查问题时，可以单独使用 `code review`。[官方分类](https://users.rust-lang.org/categories)。

标题：`rust_iso20022: ISO 20022 XML tooling with opt-in typed Rust models`

---

I'm sharing **rust_iso20022**, an Apache-2.0 ISO 20022 message toolkit for Rust. It is aimed at applications that inspect or process financial XML, including payment and bank-statement integration.

The library has **1,130 schema-generated message-version models across 32 business areas**, including earlier message versions. The generated structs remain directly accessible as the canonical model.

The feature layout separates two common needs:

- The default core detects message identity, reads XML fields and extracts header/business metadata without compiling generated models.
- Typed processing is enabled per family, for example `model-pacs`, `model-pain` or `model-camt`. Optional `serde` and `convert` features add JSON serialization and decimal/date helpers.

There are also builders for selected pacs, pain and camt messages, a schema-derived catalogue and version-comparison APIs. The current crates.io version is **0.1.6**, and the core MSRV is **Rust 1.85**.

In your own Rust application:

```bash
cargo add rust_iso20022@0.1.6
# For typed payment models, JSON and decimal helpers:
cargo add rust_iso20022@0.1.6 --features model-pacs,serde,convert
```

To try the existing synthetic examples from the repository:

```bash
git clone --branch v0.1.6 --depth 1 https://github.com/rust-iso/rust_iso20022.git
cd rust_iso20022
cargo run --locked --example inspect_message
cargo run --locked --example typed_payment --features model-pacs,serde,convert
```

`inspect_message` identifies a pacs.008 message and extracts payment fields without generated types. `typed_payment` demonstrates generated field access, Decimal conversion, XML round-tripping and JSON serialization. Start with one model family: compiling the full `model` feature has substantial compilation and memory cost.

Coverage has explicit boundaries: generated-model coverage does not mean complete validation coverage. Complete offline XSD validation is not implemented; generated-message L2 binding currently covers `pacs.008.001.08`. CBPR+/SEPA public-reference checks are provisional developer checks, not certification or complete production rule packs. CLI, MT migration, WASM and MCP are separate unpublished workspace adapters.

I'd particularly welcome feedback on the per-family feature design, typed APIs and which message versions are useful in actual integration work.

[Repository](https://github.com/rust-iso/rust_iso20022) · [Crate](https://crates.io/crates/rust_iso20022) · [API docs](https://rust-iso.github.io/rust_iso20022/) · [Coverage and limitations](https://github.com/rust-iso/rust_iso20022/blob/main/docs/status.md)
