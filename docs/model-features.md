# Choosing an ISO 20022 model feature

Every ISO 20022 message name starts with a four-letter business-area code. To
use generated Rust types, enable the matching `model-<area>` feature:

```text
pacs.008.001.08  ->  model-pacs
pain.001.001.11  ->  model-pain
camt.053.001.08  ->  model-camt
```

For the production migration target `camt.053.001.09`, resource-constrained
consumers may enable `model-camt-053-001-09`. This focused feature exposes the
same generated module and `AnyMessage` support without compiling the other 168
cash-management versions. `model-camt` remains the compatible full-family
feature and includes that module as before.

```toml
[dependencies]
rust_iso20022 = { version = "0.1", default-features = false, features = [
    "model-camt-053-001-09",
] }
```

Then replace the dots in the message name with underscores to get its module
path. For example, `pacs.008.001.08` is available as:

```rust,ignore
rust_iso20022::generated::pacs::pacs_008_001_08::Document
```

Enable more than one area when an application sends or receives multiple
families:

```toml
[dependencies]
rust_iso20022 = { version = "0.1", features = [
    "model-head",
    "model-pacs",
    "model-pain",
    "model-camt",
] }
```

## Available model families

| Area | Feature | Description | Message versions |
|---|---|---|---:|
| `acmt` | `model-acmt` | Account Management | 59 |
| `admi` | `model-admi` | Administration | 12 |
| `auth` | `model-auth` | Authorities | 118 |
| `caaa` | `model-caaa` | Acceptor to Acquirer Card Transactions | 51 |
| `caad` | `model-caad` | Card Administration | 17 |
| `caam` | `model-caam` | ATM Management | 25 |
| `cafc` | `model-cafc` | Fee Collection | 4 |
| `cafm` | `model-cafm` | File Management | 4 |
| `cafr` | `model-cafr` | Fraud Reporting and Disposition | 8 |
| `cain` | `model-cain` | Acquirer to Issuer Card Transactions | 40 |
| `camt` | `model-camt` | Cash Management | 169 |
| `canm` | `model-canm` | Network Management | 8 |
| `casp` | `model-casp` | Sale to POI Card Transactions | 33 |
| `casr` | `model-casr` | Settlement Reporting | 4 |
| `catm` | `model-catm` | Terminal Management | 16 |
| `catp` | `model-catp` | ATM Card Transactions | 33 |
| `colr` | `model-colr` | Collateral Management | 38 |
| `fxtr` | `model-fxtr` | Foreign Exchange Trade | 28 |
| `head` | `model-head` | Business Application Header | 3 |
| `pacs` | `model-pacs` | Payments Clearing and Settlement | 22 |
| `pain` | `model-pain` | Payments Initiation | 27 |
| `reda` | `model-reda` | Reference Data | 74 |
| `remt` | `model-remt` | Payments Remittance Advice | 4 |
| `secl` | `model-secl` | Securities Clearing | 15 |
| `seev` | `model-seev` | Securities Events | 98 |
| `semt` | `model-semt` | Securities Management | 48 |
| `sese` | `model-sese` | Securities Settlement | 57 |
| `setr` | `model-setr` | Securities Trade | 29 |
| `trck` | `model-trck` | Payments Tracker | 3 |
| `tsin` | `model-tsin` | Trade Services Initiation | 12 |
| `tsmt` | `model-tsmt` | Trade Services Management | 52 |
| `tsrv` | `model-tsrv` | Trade Services | 19 |
| | `model` | All 32 families | **1,130** |

Prefer the smallest set of area features your application needs. The umbrella
`model` feature is useful for gateways that must parse every known message, but
it takes substantially more time and memory to compile.

For local checks of generated models, `CARGO_PROFILE_DEV_DEBUG=0` and
`CARGO_PROFILE_TEST_DEBUG=0` avoid generating large debug symbol tables. The
repository's verification scripts set these values by default and use one
Cargo job to bound peak memory.

The hosted docs enable `head`, `pacs`, and `pain` as representative generated
models so that the docs.rs build stays within its memory limit. All families in
the table are part of the crate and are tested independently before release.
