这些文件是按英文社区分别编写的发布材料，版本口径统一为 0.1.6。整理说明使用中文，社区发布正文使用英文。项目定位、topics 和 crates.io 元数据建议见 [宣传方案](../promotion-plan.md)，实现与验证边界见 [当前状态](../status.md)。

每份材料都区分填写在平台字段里的标题、分类，以及可以复制的正文。除另有说明外，复制正文时从横线之后开始。文案不假设已有银行客户、性能领先或官方认证；发布者可以加入自己的真实开发经历。

DEV 教程文件是纯英文文章，从首行 front matter 开始整体复制，或把标题与标签填写在编辑器字段中。其 `published: false` 保持草稿状态；本稿使用三个标签，符合官方最多四个标签的规则。[DEV 编辑器说明](https://dev.to/p/editor_guide)。

| 社区 | 发布材料 | 分类 / 标签 | 标题 |
| --- | --- | --- | --- |
| Rust Users Forum | [标题、正文和示例](rust-forum.md) | `announcements` | rust_iso20022: ISO 20022 XML tooling with opt-in typed Rust models |
| r/rust | [标题、正文和反馈问题](reddit-rust.md) | 项目展示；flair 以当时选项为准 | ISO 20022 in Rust: 1,130 schema-generated models with opt-in features |
| Hacker News | [链接投稿和作者说明](show-hn.md) | `Show HN:` | Show HN: ISO 20022 for Rust with 1,130 generated message models |
| DEV Community | [完整英文教程](devto.md) | `rust`、`xml`、`opensource` | Processing ISO 20022 payment XML in Rust: detection, typed parsing and JSON |
| LinkedIn | [行业场景短文](linkedin.md) | `#ISO20022 #RustLang #Payments #FinTech` | ISO 20022 message integration with Rust |
| This Week in Rust | [投稿条目和 PR 正文](this-week-in-rust.md) | 教程，考虑 `Rust Walkthroughs` | Processing ISO 20022 payment XML in Rust: detection, typed parsing and JSON |
| awesome-rust | [候选收录条目和 PR 正文](awesome-rust.md) | `Libraries → Finance` | Add rust_iso20022 to Finance libraries |

建议先发 Rust Forum 和 r/rust，再根据反馈完善教程与 Show HN。各平台的内容已按读者关注点调整，无需把同一段版本公告重复粘贴。

Show HN 发布的是可运行项目；作者应在场回答问题。TWiR 教程投稿需要先让教程具有可公开访问的 URL，awesome-rust 需要先核对其关注度门槛。这两份材料包含具体准备方式与正文，提交条件分别写在文件开头。

教程使用相同的虚构示例 XML，有默认核心与强类型两个完整 Rust 程序。校验功能不作为此示例的结论。若原样或主要沿用 AI 辅助整理的教程投稿 TWiR，应保留其中的披露说明；其要求见 [TWiR README](https://github.com/rust-lang/this-week-in-rust/blob/main/README.md)。
