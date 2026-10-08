入口：[This Week in Rust 仓库](https://github.com/rust-lang/this-week-in-rust)。

投稿类型：有可运行代码的教程，建议在下一期草稿的 `Rust Walkthroughs` 区域添加条目，由编辑决定归类。当前不接受 `Projects/Tooling Updates` 的 PR 投稿。[官方 README](https://github.com/rust-lang/this-week-in-rust/blob/main/README.md)。

本次使用已准备好的 [英文教程](devto.md)，标题与文章一致：`Processing ISO 20022 payment XML in Rust: detection, typed parsing and JSON`。

提交前先让教程公开可访问。下面条目使用本文档提交推送后在仓库中的确切地址；如果先发布到 DEV.to，则把该地址替换为实际文章 URL。推送前，这个新文件的远程地址尚不可访问。教程保留 AI 辅助披露，满足按当前草稿投稿时的作者透明度要求。

下一期草稿中添加的一行：

```markdown
* [Processing ISO 20022 payment XML in Rust: detection, typed parsing and JSON](https://github.com/rust-iso/rust_iso20022/blob/main/docs/promotion/devto.md)
```

PR 标题：`Add ISO 20022 payment XML walkthrough`

PR 正文从横线之后开始。

---

This submission adds a Rust walkthrough for processing a synthetic ISO 20022 pacs.008 XML message with rust_iso20022 0.1.6.

The article includes two complete programs: model-free message detection and field inspection, followed by opt-in generated-type parsing, Decimal conversion, XML round-tripping and JSON serialization. It explains family-level Cargo features and distinguishes model coverage from validation coverage.

The tutorial was prepared with AI assistance, disclosed in the article. Its code examples have been checked locally against the SDK source corresponding to the 0.1.6 release.

Suggested category: Rust Walkthroughs. This is a tutorial submission, not a Project/Tooling Updates entry.
