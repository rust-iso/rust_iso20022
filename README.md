# rust_iso20022

[![Crates.io](https://img.shields.io/crates/v/rust_iso20022.svg)](https://crates.io/crates/rust_iso20022)
[![Documentation](https://img.shields.io/badge/docs-GitHub%20Pages-blue)](https://rust-iso.github.io/rust_iso20022/)
[![docs.rs](https://docs.rs/rust_iso20022/badge.svg)](https://docs.rs/rust_iso20022)
[![Downloads](https://img.shields.io/crates/d/rust_iso20022.svg)](https://crates.io/crates/rust_iso20022)
[![MSRV](https://img.shields.io/badge/MSRV-1.85-dea584.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/crates/l/rust_iso20022.svg)](LICENSE)

**Production-grade ISO 20022 SDK for Rust.** Detect, inspect, parse, build,
validate, compare, migrate, and serialize financial MX messages with generated,
strongly typed models.

Generated models remain fully accessible. High-level APIs are additive: every
builder and migration returns the existing generated struct, and validation
operates directly on generated values or short-lived field views over them.

The crate covers **1,130 message versions across 32 business areas**. Its small
core can inspect any ISO 20022 message without compiling the generated model;
applications that need compile-time field access can enable only the message
families they use.

- Identify a message from its XML namespace (`pacs.008.001.08`,
  `camt.053.001.08`, and so on).
- Read headers, payment metadata, and arbitrary XML fields without generated
  types.
- Parse and build strongly typed messages generated from the official
  iso20022.org XSD schemas.
- Round-trip XML and, optionally, JSON with ISO 20022 element names.
- Query a schema-derived catalogue and compare message versions semantically.
- Produce structured L2 validation reports with stable rule IDs and paths.
- Use typed builders for the phase-one pacs, pain, and camt messages.
- Run detect, inspect, serialize, validate, explain, catalogue, and comparison
  commands through a thin CLI.
- Convert MT103, MT202, and MT940 through the separate migration crate with a
  complete field mapping report.
- Work with exact string scalars or convert amounts and dates to
  `rust_decimal` and `chrono` values.

The versioned CBPR+ and SEPA profile framework is implemented, but no complete
production L3 rule pack is currently claimed. See [profile status](docs/profiles.md).

See the [payment integration flow](docs/integration.md) for the recommended
detect → catalogue → generated parse → validate → serialize path.

## Quick start

Add the feature-free core:

```bash
cargo add rust_iso20022
```

Identify and inspect an incoming message:

```rust
use rust_iso20022::{detect, MxNode};

let xml = r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08">
  <FIToFICstmrCdtTrf>
    <GrpHdr><MsgId>ABC-1</MsgId></GrpHdr>
    <CdtTrfTxInf>
      <IntrBkSttlmAmt Ccy="EUR">1234.56</IntrBkSttlmAmt>
    </CdtTrfTxInf>
  </FIToFICstmrCdtTrf>
</Document>"#;

let id = detect(xml).expect("ISO 20022 message");
assert_eq!(id.message_name(), "pacs.008.001.08");
assert_eq!(id.business_area.description(), "Payments Clearing and Settlement");

let tree = MxNode::parse(xml).expect("valid XML");
assert_eq!(tree.find("MsgId").and_then(|node| node.text()), Some("ABC-1"));

let amount = tree.find("IntrBkSttlmAmt").unwrap();
assert_eq!(amount.text(), Some("1234.56"));
assert_eq!(amount.attr("Ccy"), Some("EUR"));
# Ok::<(), rust_iso20022::Error>(())
```

This path needs no generated model and is a good fit for routing, observability,
validation, metadata extraction, and systems that accept many message versions.

## Typed messages

The first four letters of a message name select its Cargo feature. For
`pacs.008.001.08`, enable `model-pacs`:

```bash
cargo add rust_iso20022 --features model-pacs,serde,convert
```

```rust
use rust_iso20022::prelude::Pacs008Document as Document;
use rust_iso20022::{from_xml, to_json, to_xml, MxMessage};

# let xml = r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08">
#   <FIToFICstmrCdtTrf><GrpHdr><MsgId>ABC-1</MsgId><NbOfTxs>1</NbOfTxs></GrpHdr></FIToFICstmrCdtTrf>
# </Document>"#;
let document: Document = from_xml(xml)?;
assert_eq!(Document::MESSAGE_NAME, "pacs.008.001.08");
assert_eq!(document.fi_to_fi_cstmr_cdt_trf.grp_hdr.msg_id.0, "ABC-1");

let xml_again = to_xml(&document)?;
let json = to_json(&document)?;
# let _ = (xml_again, json);
# Ok::<(), rust_iso20022::Error>(())
```

The `prelude` aliases are the generated document types themselves. Use the
full `generated::...` path when you prefer an explicit versioned path.

For routing and feature diagnostics, the catalogue accepts either a message
name or a full namespace:

```rust
let entry = rust_iso20022::lookup_message("pacs.008.001.08").unwrap();
assert_eq!(rust_iso20022::required_feature(entry.message_name), Some("model-pacs"));
```

The mapping is predictable:

```text
pacs.008.001.08
  ├── Cargo feature: model-pacs
  └── Rust type:     generated::pacs::pacs_008_001_08::Document
```

See [the model feature guide](docs/model-features.md) for all 32 business areas
and their meanings, the [domain feature aliases](docs/domain-features.md) for
broader deployment-oriented selections, or the
[generated support matrix](docs/support-matrix.md) for the schema-derived
message counts.

## Choose the right API

| Need | API | Feature |
|---|---|---|
| Detect the message type | `detect`, `MxId`, `BusinessArea` | none |
| Read any field without generated types | `MxNode::parse` | none |
| Read AppHdr, amount, currency, dates, and parties | `read_business_message`, `metadata::extract` | none |
| Parse or build a typed message | `from_xml`, `to_xml`, `generated::<area>` | `model-<area>` |
| Auto-dispatch to a typed message | `generated::any::parse_auto` | one or more `model-<area>` |
| Serialize typed messages as JSON | `from_json`, `to_json` | `serde` |
| Convert exact scalar strings | `convert::{to_decimal, to_date, to_datetime}` | `convert` |
| Validate installed generated bindings | `validation`, `ValidationReport` | matching `model-<area>` |
| Address exact profile releases | `profiles` | `profiles` |
| Compare schema versions | `compare::compare_versions` | none |
| Download schemas at runtime | `fetch::Fetcher` | `catalogue` |
| Use the standalone CLI | `rust_iso20022_cli` workspace crate | separate package |

The static message catalogue is always available and does not access the
network.

## Installation and features

For a typical payments service:

```toml
[dependencies]
rust_iso20022 = { version = "0.1", features = [
    "model-head", # Business Application Header
    "model-pacs", # interbank clearing and settlement
    "model-pain", # customer payment initiation
    "serde",      # JSON support
    "convert",    # Decimal and chrono conversions
] }
```

| Feature | Default | Effect |
|---|:---:|---|
| `model-<area>` | no | Generated types for one business area, such as `model-pacs` |
| `payments` | no | Common payment-domain families (`head`, `camt`, `pacs`, `pain`, and related areas) |
| `securities` | no | Securities settlement/trade message families |
| `trade` | no | Collateral, trading, and treasury message families |
| `cards` | no | Card payment and card-related message families |
| `fx` | no | Foreign-exchange message family |
| `model` | no | All 1,130 generated message modules; expensive to compile |
| `serde` | no | Serde derives plus `to_json` and `from_json` |
| `convert` | no | `rust_decimal` and `chrono` scalar conversions |
| `catalogue` | no | Async runtime XSD fetcher using Tokio and Reqwest |
| `profiles` | no | Exact-release identity and dispatch framework; no rule pack implied |
| `cli` | no | Legacy root catalogue binary compatibility feature |

Prefer per-area features. The umbrella `model` feature is intended for
gateways that genuinely need all message families.

Domain aliases are convenient for applications that want a complete business
surface; use individual `model-*` features when compile time and memory are
more important than breadth.

Minimum supported Rust version: **1.85**.

## Coverage

Generated types are available for these ISO 20022 business areas:

`acmt`, `admi`, `auth`, `caaa`, `caad`, `caam`, `cafc`, `cafm`, `cafr`, `cain`,
`camt`, `canm`, `casp`, `casr`, `catm`, `catp`, `colr`, `fxtr`, `head`, `pacs`,
`pain`, `reda`, `remt`, `secl`, `seev`, `semt`, `sese`, `setr`, `trck`, `tsin`,
`tsmt`, and `tsrv`.

Current and earlier message versions are included so that older messages still
in circulation remain parseable. See [model features](docs/model-features.md)
for the per-area counts and [implementation status](docs/status.md) for known
schema-source gaps.

## Runnable examples

```bash
# Identify a message, read AppHdr and payment metadata, and inspect arbitrary fields.
cargo run --example inspect_message

# Parse pacs.008 into generated types, then serialize it as XML and JSON.
cargo run --example typed_payment --features model-pacs,serde,convert

# Catalogue, explanation, and semantic version-diff examples need no model.
cargo run --example catalogue
cargo run --example explain
cargo run --example version_diff
cargo run --example support_matrix

# Detect, catalogue, parse, validate, and serialize one payment end to end.
cargo run --example payment_pipeline --features model-pacs,serde

# Validate the canonical generated pacs.008 value.
cargo run --example validation --features model-pacs

# Exercise exact profile identity without claiming an installed rule pack.
cargo run --example profile --features profiles

# Convert MT103 and retain its mapping report.
cargo run -p rust_iso20022_migration --example mt103 --features mt103
```

## Command-line SDK

```bash
cargo build -p rust_iso20022_cli --features json,model-pacs,profiles

iso20022 detect payment.xml
iso20022 inspect --json payment.xml
iso20022 validate --layer l2 payment.xml
iso20022 explain ISO20022-L2-IBAN-CHECKSUM
iso20022 versions --json pacs.008.001
iso20022 compare pacs.008.001.08 pacs.008.001.10
```

See the [CLI reference](docs/cli.md) for stable JSON envelopes and exit codes.

## Design notes

- Generated scalar fields use `String` to preserve their exact XML value and
  avoid floating-point rounding. Convert only when arithmetic is needed.
- XSD choices use structs of `Option<...>` fields, which preserve nested values
  and attributes while omitting unset choices during serialization.
- Unknown coded-enumeration values use an `__Unknown__(String)` fallback rather
  than discarding the original input.
- XML round-tripping preserves the data model, not byte-for-byte formatting.
- The generated code targets `yaserde` 0.7 semantics. Upgrading YaSerde requires
  regenerating and revalidating the complete model.

## WebAssembly and code generation

The identification, catalogue, header, metadata, and generic-tree APIs can be
built for JavaScript with `scripts/build-wasm.sh`; see the
[WASM API reference](docs/wasm-api.md).

Maintainers can regenerate the checked-in model from the XSD sources:

```bash
cargo run -p rust_iso20022_codegen -- --input xsds --output src/generated
```

The generator and XSD sources are excluded from the published crate, which has
no Git dependencies.

## Contributing and support

Bug reports, message compatibility cases, and pull requests are welcome in the
[GitHub repository](https://github.com/rust-iso/rust_iso20022). When reporting a
parsing issue, include the message identifier and a minimal redacted XML sample
if possible.

Report suspected vulnerabilities privately as described in
[SECURITY.md](SECURITY.md); never attach a real financial message.

## Using rust_iso20022 in production?

Sponsor ongoing ISO schema updates, annual CBPR+/SEPA research, migration
tooling, security, fuzzing, documentation, and long-term maintenance through
[Ko-fi](https://ko-fi.com/jnz). Sponsorship supports the open-source core; it
does not unlock a private or closed implementation.

## Compliance boundary

This project provides schema-derived models, parsing, implemented semantic and
profile validation rules, and developer tooling. A successful result means only
“valid according to the implemented rules and identified release.” It does not
guarantee acceptance by a particular bank or network, regulatory certification,
network onboarding approval, or legal compliance.

## License

Licensed under the [Apache License 2.0](LICENSE).
