入口：[LinkedIn](https://www.linkedin.com/)，以个人或项目主页的普通动态发布。

首行：`ISO 20022 message integration with Rust`

建议标签：`#ISO20022 #RustLang #Payments #FinTech`。普通动态没有与论坛相同的分类字段，标签用于表达主题。

正文从横线之后开始，按纯文本复制即可。

---

ISO 20022 message integration with Rust

For payment and banking integration teams, a useful starting point is identifying an incoming message, extracting its business fields and then working with typed data.

rust_iso20022 is an Apache-2.0 open-source toolkit for that workflow. It provides 1,130 schema-generated message-version models across 32 business areas, including payment and cash-management messages.

The default core can inspect XML and read message identity, header and business metadata without compiling generated models. Teams that need typed parsing can enable the relevant family, such as pacs, pain or camt. Optional JSON serialization and decimal conversion helpers support internal application processing.

The current published SDK is 0.1.6, with Rust 1.85 as its core MSRV. Runnable examples demonstrate payment inspection, typed field access and XML round-tripping.

Coverage is documented: complete offline XSD validation is not implemented; generated-message L2 binding covers pacs.008.001.08; CBPR+/SEPA public-reference checks are provisional developer checks rather than certification or complete production rule packs.

I'd welcome practical feedback from engineers working on payment-message ingestion, bank-statement integration or reconciliation: which message versions and examples would be most useful?

Repository: https://github.com/rust-iso/rust_iso20022

Examples: https://github.com/rust-iso/rust_iso20022/tree/v0.1.6/examples

Docs: https://rust-iso.github.io/rust_iso20022/

#ISO20022 #RustLang #Payments #FinTech
