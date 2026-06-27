# rust_iso20022

ISO 20022 financial message definitions, identification and code sets for Rust,
generated from the official ISO 20022 XSD schemas.

This is the ISO 20022 (MX / SWIFT payments) counterpart to
[`rust_iso3166`](https://github.com/rust-iso/rust_iso3166), and is functionally
inspired by Prowide's Java [`prowide-iso20022`](https://github.com/prowide/prowide-iso20022)
library: a strongly-typed model for MX messages plus XML parsing and
serialization.

## What's in the crate

The crate has three layers:

| Layer | Module | Source | Description |
|-------|--------|--------|-------------|
| Core  | (root) | hand-written | `MxId`, `BusinessArea`, `from_xml`/`to_xml`, `Error` |
| Model | `generated` | generated from XSD | one module per message version, e.g. `generated::pacs::pacs_008_001_08` |
| Catalogue | `catalogue` | generated from XSD | every message id + namespace as static `phf` tables |

**Coverage:** a typed model is generated for **722** message schemas across
32 business areas (`acmt`, `admi`, `auth`, `caaa`, `caad`, `caam`, `cafc`,
`cafm`, `cafr`, `cain`, `camt`, `canm`, `casp`, `casr`, `catm`, `catp`, `colr`,
`fxtr`, `head`, `pacs`, `pain`, `reda`, `remt`, `secl`, `seev`, `semt`, `sese`,
`setr`, `tsin`, `tsmt`, `tsrv`, `trck`) — including the securities settlement /
management and trade-services families.

The generated types derive [`yaserde`](https://crates.io/crates/yaserde)'s
`YaSerialize` / `YaDeserialize` for XML, and (with the `serde` feature)
`serde::{Serialize, Deserialize}` for JSON.

## prowide-iso20022 parity

| prowide capability | this crate |
|---|---|
| Typed model, all message versions | `generated::<area>::<msg>::Document` |
| XML parse / serialize | `from_xml` / `to_xml` |
| JSON parse / serialize (`fromJson`/`toJson`) | `from_json` / `to_json` (feature `serde`) |
| `AbstractMX` identity (namespace, MxId, area, functionality, variant, version) | the `MxMessage` trait, implemented by every `Document` |
| `AbstractMX.parse(xml)` auto-detection | `detect(xml)`, `parse_as::<T>()`, and `generated::any::parse_auto(xml)` → `AnyMessage` |
| Business Application Header (BAH / `head.001`) | `app_hdr::parse_business_header` → `BusinessHeader` |
| Message catalogue | `catalogue` |

```rust
# #[cfg(feature = "model")] {
use rust_iso20022::{MxMessage, detect};
use rust_iso20022::generated::any::{parse_auto, AnyMessage};

// Auto-detect and parse any supported message:
let msg = parse_auto(&xml)?;
println!("{} from {:?}", msg.message_name(), msg.mx_id().business_area);

// Or work with a known type, which carries its own identity:
use rust_iso20022::generated::pacs::pacs_008_001_08::Document;
assert_eq!(Document::MESSAGE_NAME, "pacs.008.001.08");
# }
```

## Usage

```rust
use rust_iso20022::{MxId, BusinessArea};

// Identify a message from its namespace or bare name.
let id = rust_iso20022::from_namespace(
    "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08",
).unwrap();
assert_eq!(id.business_area, BusinessArea::pacs);
assert_eq!(id.functionality, "008");
assert_eq!(id.message_name(), "pacs.008.001.08");

// The catalogue knows every staged message.
assert!(rust_iso20022::catalogue::contains("pacs.008.001.08"));
let entry = rust_iso20022::catalogue::from_message_name("pacs.008.001.08").unwrap();
assert_eq!(entry.namespace, "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08");
```

Parsing and serializing a message (enable that area's model feature, e.g.
`cargo add rust_iso20022 -F model-pacs`):

```rust,ignore
use rust_iso20022::generated::pacs::pacs_008_001_08::Document;

let xml = std::fs::read_to_string("message.xml")?;
let doc: Document = rust_iso20022::from_xml(&xml)?;
let back: String = rust_iso20022::to_xml(&doc)?;
```

## Features

| Feature | Default | Effect |
|---------|---------|--------|
| `model-<area>` | no | the typed model for one business area, e.g. `model-pacs`, `model-camt`. Enable only what you need — compiling a single family takes seconds vs many minutes for all |
| `model` | no | all `model-<area>` at once (~722 modules; slow to compile) |
| `serde` | no | `serde` support for `MxId`, `BusinessArea`, `CatalogueEntry` (serialize to canonical strings) |
| `convert` | no | typed scalar conversions (`to_decimal`/`to_date`/`to_datetime`) via `rust_decimal`/`chrono` |
| `cli` | no | the `iso20022` command-line catalogue lookup tool |
| `catalogue` | no | runtime XSD fetcher (`fetch` module): pulls in `tokio`, `reqwest`, `regex` |

The XSD → Rust generator is a separate workspace crate (`tools/codegen`), not a
feature of the published crate.

> The core (`MxId`, `BusinessArea`) and `catalogue` are always available; the
> typed `generated` model requires a `model-<area>` feature (or `model`).

## Command-line tool

```bash
cargo run --features cli --bin iso20022 -- pacs.008
# or after `cargo install rust_iso20022 --features cli`:
iso20022 camt          # every message in the camt business area
iso20022 008.001.08    # match on functionality/variant/version
iso20022               # the whole catalogue
```

It prints a table of matching messages (name, area, description, whether a typed
model exists, and the namespace).

## Regenerating the model

The model and catalogue are produced from XSD schemas in `xsds/` by the codegen
tool, which lives in a separate, unpublished workspace crate (`tools/codegen`)
so the published crate has no git dependencies:

```bash
cargo run -p rust_iso20022_codegen -- --input xsds --output src/generated
```

To (re)download the schemas, the `catalogue` feature provides a `Fetcher`. The
canonical source is <https://www.iso20022.org>, but that host is behind Akamai
bot-protection that refuses non-browser clients, so the fetcher's source is
configurable and can target a schema mirror.

## Design notes & limitations

- **Scalar values are `String`.** `xsd-types` scalars (`Decimal`, `Date`,
  `DateTime`, …) lack `serde` support, so generated leaf values are exposed as
  `String`. This is lossless (exact text, no float rounding — desirable for
  monetary amounts) and keeps XML and JSON in sync.
- **Choices** are modelled as a struct of `Option<…>` fields (JAXB/prowide
  style), so amounts inside a `<xsd:choice>` round-trip with their `Ccy`
  attribute and unset choices are simply omitted. The exception is *inline*
  choices nested in a sequence (16 messages), still modelled as enums — for those
  an unset choice may emit a placeholder, which `to_xml` strips.
- **JSON shape:** JSON uses the ISO 20022 element names (`MsgId`, `IBAN`, …),
  mirroring the yaserde renames; `to_json`/`from_json` round-trip.
- XML round-tripping preserves the data model but not necessarily byte-for-byte
  formatting (namespaces, whitespace).
- The generator handles multi-`<xsd:choice>` complexTypes by disambiguation, so
  all 502 messages now have a model (no skips).

## License

Apache-2.0.
