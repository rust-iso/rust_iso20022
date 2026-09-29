# Domain feature aliases

The generated model remains selectable at business-area granularity. For
applications that prefer domain terminology, the crate also provides additive
aliases:

| Alias | Included families |
|---|---|
| `payments` | `head`, `acmt`, `admi`, `auth`, `camt`, `pacs`, `pain`, `reda`, `remt` |
| `securities` | `secl`, `seev`, `semt`, `sese`, `setr` |
| `trade` | `colr`, `tsin`, `tsmt`, `tsrv` |
| `cards` | `caaa`, `caad`, `caam`, `cafc`, `cafm`, `cafr`, `casp`, `casr`, `catm`, `catp` |
| `fx` | `fxtr` |

These aliases only expand to existing `model-*` features. They do not copy or
wrap generated structs. Use the smallest area feature when build resources are
constrained:

```toml
[dependencies]
rust_iso20022 = { version = "0.1", features = ["model-pacs", "serde"] }
```

Use a domain alias when an application intentionally handles a broad surface:

```toml
[dependencies]
rust_iso20022 = { version = "0.1", features = ["payments", "serde"] }
```

CI verifies that these aliases continue to match the documented family
composition and that the schema-derived support matrix has no uncommitted
drift after regeneration.

Common payment documents also have short aliases in `rust_iso20022::prelude`.
Each alias is the exact generated `Document` type:

```rust,ignore
use rust_iso20022::prelude::{
    HeadDocument, Camt053Document, Pacs008Document, Pain001Document,
};
```

The complete versioned paths remain available under `generated::<area>` for
applications that prefer explicit schema versions.
