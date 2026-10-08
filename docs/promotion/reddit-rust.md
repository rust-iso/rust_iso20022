入口：[r/rust](https://www.reddit.com/r/rust/)。

形式：文本帖，含仓库链接和运行步骤。Flair 使用发布时实际提供的项目展示相关选项；当前规则页面未能完整读取，不指定未经确认的 flair 名称。[规则入口](https://www.reddit.com/r/rust/about/rules)。

标题：`ISO 20022 in Rust: 1,130 schema-generated models with opt-in features`

---

I'm sharing **rust_iso20022**, a Rust toolkit for ISO 20022 financial XML. The part most relevant to Rust library design is the split between a model-free inspection core and opt-in generated types.

It contains **1,130 message-version models across 32 business areas**, generated from official ISO 20022 XSDs. Enabling all of them is expensive, so applications can select families such as `model-pacs`, `model-pain` and `model-camt`. The default feature set is empty; message detection and XML field inspection still work without generated models.

For typed processing, the generated structs are the canonical model. The higher-level APIs retain access to those structs instead of introducing a second payment representation. Optional features provide Serde JSON and decimal/date conversion helpers.

Try the model-free example:

```bash
git clone --branch v0.1.6 --depth 1 https://github.com/rust-iso/rust_iso20022.git
cd rust_iso20022
cargo run --locked --example inspect_message
```

Or install the crate in an existing application:

```bash
cargo add rust_iso20022@0.1.6 --features model-pacs,serde,convert
```

The current version is **0.1.6**, with a **Rust 1.85** core MSRV. This patch release fixes dependency and CI/release issues; the model collection was already present in earlier releases.

Generated model breadth is separate from validation breadth: complete offline XSD validation is not implemented, and generated-message L2 binding currently covers `pacs.008.001.08`. CBPR+/SEPA reference checks are provisional. CLI, MT migration, WASM and MCP adapters are currently built from the repository, not published as crates.io adapters.

For people maintaining large generated Rust libraries: does this per-family feature layout fit how you would use it? For payment integration developers: which exact message versions and workflows would make the most useful next examples?

[GitHub](https://github.com/rust-iso/rust_iso20022) · [crates.io](https://crates.io/crates/rust_iso20022) · [Docs](https://rust-iso.github.io/rust_iso20022/) · [Support and limitations](https://github.com/rust-iso/rust_iso20022/blob/main/docs/status.md)
