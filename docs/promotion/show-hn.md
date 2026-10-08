入口：[HN 提交页面](https://news.ycombinator.com/submit)。

提交形式：链接投稿，URL 使用 `https://github.com/rust-iso/rust_iso20022`。Show HN 要求项目能实际试用，作者应在场回应；单纯补丁版本公告通常不足以构成 Show HN。[官方规则](https://news.ycombinator.com/showhn.html)。

标题：`Show HN: ISO 20022 for Rust with 1,130 generated message models`

首条作者说明：复制下面的 text 块，不包含代码围栏。HN 使用纯文本链接，命令单独成行并缩进。

```text
I'm sharing rust_iso20022, an open-source Rust toolkit for ISO 20022 financial XML, including payment and bank-statement messages.

The problem it addresses is working with a large, versioned XML message catalogue while keeping straightforward inspection available without compiling the entire generated model. It provides 1,130 schema-generated message-version models across 32 business areas; applications opt into families such as pacs, pain or camt when they need typed field access.

To try the inspection example locally, with Rust 1.85 or newer:

  git clone --branch v0.1.6 --depth 1 https://github.com/rust-iso/rust_iso20022.git
  cd rust_iso20022
  cargo run --locked --example inspect_message

That example detects a synthetic pacs.008 payment message and reads its header, amount, currency and identifiers without generated models. For typed parsing, decimal conversion, XML round-tripping and JSON:

  cargo run --locked --example typed_payment --features model-pacs,serde,convert

The typed example compiles the pacs family, so it has a larger build cost. The whole model collection is available, but compiling all families at once requires substantial time and memory.

The published SDK is version 0.1.6, licensed under Apache-2.0. Generated types remain directly accessible. The repository also has CLI, MT migration, WASM and MCP adapters, which are currently unpublished workspace crates.

The validation scope is explicit: complete offline XSD validation is not implemented; generated-message L2 binding covers pacs.008.001.08; CBPR+/SEPA public-reference checks are provisional and do not imply bank certification or full production rule coverage.

I'm interested in feedback from payment integration developers and people designing large generated Rust libraries: which message versions, examples and feature boundaries would help your work?

Crate: https://crates.io/crates/rust_iso20022
Docs: https://rust-iso.github.io/rust_iso20022/
Coverage: https://github.com/rust-iso/rust_iso20022/blob/main/docs/status.md
```
