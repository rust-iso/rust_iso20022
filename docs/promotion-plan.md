本文整理 rust_iso20022 面向英文社区的宣传定位、渠道、标题、简介和标签，依据当前 0.1.6 代码、README、状态说明和更新日志。平台规则核对日期：2026-10-08；实际投稿前需要核对当时的规则与分类。

**建议统一定位为：面向支付与银行报文集成的 Rust ISO 20022 开源 SDK。**

英文定位：An open-source ISO 20022 message toolkit for Rust.

优先吸引两类人：处理支付、银行对账、资金管理和金融报文的集成工程师；关注 XML、代码生成、强类型模型与按需编译的 Rust 开发者。前者关心能解决哪些报文问题，后者关心 API、安装成本和可运行示例。渠道优先级是结合项目特点给出的判断，不代表平台推荐或收录承诺。

项目的主要宣传依据如下。详细功能边界见 [实现状态](status.md)、[支持矩阵](support-matrix.md)、[模型特性](model-features.md) 和 [更新日志](../CHANGELOG.md)。

| 可以突出什么 | 对用户有什么用 | 宣传措辞 |
| --- | --- | --- |
| 基于官方 ISO 20022 XSD 生成的 1,130 个报文版本模型，覆盖 32 个业务领域 | 使用已有 Rust 类型处理不同报文家族与版本 | “1,130 个报文版本模型”；包含历史版本，不能解释为 1,130 个最新业务流程 |
| 默认不编译生成模型；按 `model-pacs` 等特性启用 | 识别、检查报文时无需编译全部模型 | “轻量检查入口，类型模型按需启用” |
| XML 解析与序列化、可选 Serde JSON、金额类型转换 | 接入 Rust 后端和内部数据处理流程 | “从 XML 到 Rust 类型，再到内部 JSON 表达” |
| 报文识别、头部与业务字段提取、目录和版本查询 | 适合报文接入、路由、排障、目录管理 | 用一个 pacs.008 实例展示消息类型、金额、币种和业务标识 |
| 常见 pacs/pain/camt 报文构建器 | 减少手写复杂 XML 的工作 | 展示具体已实现的构建器和报文版本 |

**渠道建议先从专业 Rust 社区开始，再发布业务场景教程。**

| 优先级 | 渠道和入口 | 分类或标签建议 | 推荐内容和标题 |
| --- | --- | --- | --- |
| 第一批 | [Rust Users Forum](https://users.rust-lang.org/categories) | `announcements`；真正请求 API 审查时使用 `code review` | 项目介绍、安装、示例和设计取舍。标题：`rust_iso20022: ISO 20022 XML tooling with opt-in typed Rust models` |
| 第一批 | [r/rust](https://www.reddit.com/r/rust/) | 项目分享；flair 按发布时的选项与规则选择 | 突出按需编译、XSD 生成和 API，提供可运行示例。标题：`ISO 20022 in Rust: 1,130 schema-generated models with opt-in features` |
| 第二批 | [Hacker News / Show HN](https://news.ycombinator.com/showhn.html) | 标题以 `Show HN:` 开头 | 展示完整可试用项目。标题：`Show HN: ISO 20022 for Rust with 1,130 generated message models` |
| 第二批 | [DEV Community](https://dev.to/) | 标签建议 `rust`、`xml`、`opensource` | 发有代码和输出的英文教程。标题：`Processing ISO 20022 payment XML in Rust: detection, typed parsing and JSON` |
| 第二批 | [LinkedIn](https://www.linkedin.com/) | 支付、银行集成和金融基础设施主题；建议 `#ISO20022 #RustLang #Payments #FinTech` | 突出报文接入、支付字段提取、银行对账场景。标题：`An open-source Rust toolkit for ISO 20022 payment and bank-statement integration` |
| 教程完成后 | [This Week in Rust](https://github.com/rust-lang/this-week-in-rust/blob/main/README.md) | 有较多 Rust 代码的教程可考虑 `Rust Walkthroughs` | 投稿教程：`Inspecting and parsing ISO 20022 payment messages in Rust`；不要直接提交普通版本更新 |
| 后续 | [awesome-rust](https://github.com/rust-unofficial/awesome-rust/blob/main/CONTRIBUTING.md) | `Libraries → Finance` | 达到其关注度门槛后提交收录建议，作为持续发现入口 |

Rust Users Forum 的官方分类明确支持项目公告和代码审查。[Rust 论坛分类](https://users.rust-lang.org/categories)。

Show HN 要求别人能实际试用，允许本地运行的工具。建议链接仓库，并在首条作者说明中给出安装或运行命令、为什么开发、已知边界和希望收到的反馈。单纯“0.1.6 发布了”通常不满足其对实质性展示的要求；不要组织点赞。[Show HN 官方规则](https://news.ycombinator.com/showhn.html)。

This Week in Rust 当前已不接受 `Projects/Tooling Updates` 栏目的 PR 投稿。可以先写有实际 Rust 代码的教程，再按 README 向下一期草稿提交相应内容；由编辑决定采用与分类。若投稿的是 LLM 撰写文章，官方要求披露这一点。贡献者招募是另一条路径，需要真实、明确难度并链接贡献指南的开放 issue。[TWiR 投稿规则](https://github.com/rust-lang/this-week-in-rust/blob/main/README.md)。

awesome-rust 要求项目具有一定使用或关注度，正文列出至少 50 GitHub stars、2,000 crates.io 下载或等效指标，并要求按模板和字母顺序添加。应先确认达标，再考虑投稿。[收录要求](https://github.com/rust-unofficial/awesome-rust/blob/main/CONTRIBUTING.md)。

当前阶段不优先投入 Product Hunt、泛 AI 或交易社区：现有产品最容易被专业工程师通过代码理解。CLI、迁移工具、WASM 和 MCP 可以成为后续独立教程的主题，但当前是仓库里的未发布适配器；介绍它们时要附源码构建方式。

**GitHub About 建议使用下面这段英文，突出用途和上手方式。**

```text
ISO 20022 message toolkit for Rust: XML inspection, typed parsing and serialization, optional JSON, and opt-in generated models across 32 business areas.
```

Website 建议指向 API 文档：<https://rust-iso.github.io/rust_iso20022/>。README 顶部保留 crates.io、文档、示例和支持矩阵的清晰入口。

建议 GitHub topics 使用以下 15 个，前五个表达核心定位：

```text
rust, iso20022, iso-20022, swift-mx, financial-messaging,
payments, banking, fintech, xml, serialization,
code-generation, serde, pacs, pain, camt
```

`webassembly`、`model-context-protocol`、`sepa` 可以在对应适配器或场景成为宣传重点时作为补充。`swift-mx` 比单独 `swift` 更清楚，后者容易与 Apple 的编程语言混淆。GitHub topics 应使用小写字母、数字和连字符，每项不超过 50 字符，总数不超过 20 个。[GitHub 官方说明](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/classifying-your-repository-with-topics)。

crates.io 的元数据与 GitHub topics 分别管理。现有 categories 合理，建议保留；keywords 可在下一次正常发布时考虑以下组合，不需要仅为了标签再发一个版本：

```toml
keywords = ["iso20022", "swift-mx", "payments", "xml", "financial-messaging"]
categories = ["finance", "encoding", "data-structures"]
```

这样保留领域关键词，并让 `xml` 对应主要输入格式。Cargo 对 keywords 和 categories 都最多允许 5 项；keyword 最多 20 字符，category 必须是 crates.io 的有效分类 slug。以上是下一版元数据建议，已发布的 0.1.6 元数据以 crates.io 为准。[Cargo 元数据规范](https://doc.rust-lang.org/cargo/reference/manifest.html#the-keywords-field)。

**英文项目介绍草稿适合 Rust Forum、r/rust；Show HN 使用前述专用标题。**

Title: rust_iso20022: ISO 20022 XML tooling with opt-in typed Rust models

rust_iso20022 is an Apache-2.0 open-source ISO 20022 message toolkit for Rust, aimed at payment and bank-message integration.

It provides 1,130 schema-generated message-version models across 32 business areas. The default core can detect messages, inspect XML fields and extract business metadata without compiling generated models. Typed processing is opt-in through Cargo features such as `model-pacs`, `model-pain` and `model-camt`. XML parsing and serialization, optional Serde JSON, decimal conversion helpers and builders for selected common messages are also available.

The current crates.io release is 0.1.6, with Rust 1.85 as the core SDK's MSRV. I'd welcome feedback on the API, feature selection and real integration needs involving pacs.008, pain.001 or camt.053.

```bash
cargo add rust_iso20022
cargo add rust_iso20022 --features model-pacs,serde,convert
```

Repository: <https://github.com/rust-iso/rust_iso20022>

Docs: <https://rust-iso.github.io/rust_iso20022/>

Scope: generated-model coverage does not imply full validation coverage. Complete offline XSD validation is not implemented, and generated-message L2 binding currently covers pacs.008.001.08. CBPR+/SEPA public-reference checks are provisional developer checks, not certification or complete production rule packs. CLI, migration, WASM and MCP adapters are currently unpublished workspace crates.

**第一篇教程建议围绕一个可运行的 pacs.008 流程展开。**

英文标题：Processing ISO 20022 payment XML in Rust: detection, typed parsing and JSON

文章顺序：先说明 pacs.008 的用途和示例输入；用默认核心识别报文与提取金额、币种、标识；再启用 `model-pacs,serde,convert` 展示强类型访问、Decimal 和 JSON；最后说明模型覆盖、校验覆盖及编译范围。JSON 是内部数据表达方式，不暗示银行接受 JSON 作为标准 ISO 20022 XML 的替代传输格式。

已有示例可以直接作为素材。在仓库根目录运行：

```bash
cargo run --example inspect_message
cargo run --example typed_payment --features model-pacs,serde,convert
cargo run --example payment_pipeline --features model-pacs,serde
```

引用 [inspect_message](../examples/inspect_message.rs)、[typed_payment](../examples/typed_payment.rs)、[payment_pipeline](../examples/payment_pipeline.rs) 的实际代码和输出。首次体验优先默认核心和单个家族，完整 `model` 或宽泛的业务别名会带来较大的编译和内存成本。

后续选题可以逐篇发布，避免重复版本公告：

| 选题 | 推荐读者 | 内容依据 |
| --- | --- | --- |
| 如何把 1,130 个 ISO 20022 模型拆成按需编译的 Rust features | Rust 库作者、代码生成工具开发者 | 模型家族、生成器、CI 分片；解释编译成本 |
| 从 pacs.008 XML 提取付款元数据，接入 Rust 后端 | 支付和银行集成工程师 | `inspect_message`、`payment_pipeline` |
| 用生成的 Rust 类型构建 pain.001 和 camt 报文 | 资金管理、账务开发者 | 已实现构建器和具体报文版本 |
| ISO 20022 模型覆盖为什么不等于规则校验覆盖 | 金融报文集成工程师 | 支持矩阵、验证和 profile 状态 |
| 从 MT103 到 pacs.008：逐字段映射及其边界 | 维护 MT/MX 迁移系统的工程师 | 仓库迁移适配器、映射报告；说明源码构建和不能保证普遍无损 |

**2026-10-08 已同步对外文档，并按社区分别准备发布草稿。**

1. [status.md](status.md) 已记录 0.1.6 发布、初始八目标 fuzz 基线和 release verification 的完成情况，同时保留有限覆盖的说明。
2. [compatibility.md](compatibility.md) 已区分当前 0.1.6 与历史 0.1.1/0.1.2 比较基线，并同步 XML 安全工作的状态。[security.md](security.md) 和 WP-029 的历史说明也已关联当前状态。
3. README 第一行已使用 `Open-source ISO 20022 message toolkit for Rust`；另说明未发布适配器和校验覆盖。MT103 文档已修正为源码路径依赖。

各渠道独立文案、标题、分类和投稿操作说明见 [社区发布材料索引](promotion/README.md)。不要使用“完整 SWIFT/SEPA 合规”“全部版本完整校验”“银行认证”或无证据的“最快/唯一”。

0.1.6 主要修复依赖和 CI/发布流程；公开 SDK API、生成模型和功能特性与 0.1.5 保持一致。可以说“当前已发布 0.1.6”，但不能说“0.1.6 新增 1,130 个模型”。[0.1.6 更新日志](../CHANGELOG.md)。

执行节奏建议：先统一 GitHub 简介、topics 和文档状态；随后在 Rust Forum 与 r/rust 各发一次英文项目介绍，并留出回复反馈的时间；再发布英文教程并考虑 TWiR；准备好无需注册的运行步骤后做一次 Show HN；行业场景文章用于 LinkedIn。成效优先看真实问题、示例试用反馈、下游接入和贡献，stars 与下载量作为辅助指标。
