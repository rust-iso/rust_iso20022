入口：[awesome-rust](https://github.com/rust-unofficial/awesome-rust)。

建议分类：`Libraries → Finance`。该目录是金融库分类，不是 `Applications → Finance`。[当前目录](https://github.com/rust-unofficial/awesome-rust#finance-1)。

提交条件：先核对 [CONTRIBUTING](https://github.com/rust-unofficial/awesome-rust/blob/main/CONTRIBUTING.md) 的 stars、下载量或等效指标门槛。本次未取得实时项目计数，因此不宣称项目已达标。条目按字母顺序放置，并沿用列表的模板和 CI badge 格式。

候选条目：

```markdown
* [rust-iso/rust_iso20022](https://github.com/rust-iso/rust_iso20022) [[rust_iso20022](https://crates.io/crates/rust_iso20022)] - ISO 20022 financial-message toolkit with XML inspection, opt-in generated Rust models and optional JSON serialization. [![CI](https://github.com/rust-iso/rust_iso20022/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/rust-iso/rust_iso20022/actions/workflows/ci.yml)
```

PR 标题：`Add rust_iso20022 to Finance libraries`

PR 正文从横线之后开始。达标依据使用提交时的实际计数，在 PR 中补充；不要复制一个未经核实的数字。

---

Adds rust_iso20022 to the Libraries / Finance section.

rust_iso20022 is an Apache-2.0 ISO 20022 financial-message toolkit for Rust. It offers XML inspection without generated models, opt-in typed models for 32 business areas and optional JSON serialization. The core SDK is published on crates.io as 0.1.6 and documents its Rust 1.85 MSRV and validation limitations.

The proposed entry uses the repository/crate template and includes the main-branch CI badge. It belongs with finance libraries because it is a message-processing SDK for downstream applications.

Repository: https://github.com/rust-iso/rust_iso20022

Crate and download statistics: https://crates.io/crates/rust_iso20022

Coverage and limitations: https://github.com/rust-iso/rust_iso20022/blob/main/docs/status.md
