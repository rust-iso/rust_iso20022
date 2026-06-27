# rust_iso20022

A Rust library for ISO 20022 (MX / SWIFT) financial messages, generated from the
official iso20022.org XSD schemas. It provides a strongly-typed model for MX
messages with XML and JSON parsing/serialization, message identification, a
generic message tree for reading any message without the model, and business
metadata extraction.

## What's in the crate

| Layer | Module | Always available? |
|-------|--------|-------------------|
| Core | (root) | yes — `MxId`, `BusinessArea`, `from_xml`/`to_xml`, `detect`, `MxNode`, `Error` |
| Catalogue | `catalogue` | yes — every message id + namespace as static [`phf`](https://crates.io/crates/phf) tables |
| Model | `generated` | per area — `generated::<area>::<msg>::Document`, enable with `model-<area>` |

**Coverage:** a typed model is generated for **1130** message versions across
**32** business areas (`acmt`, `admi`, `auth`, `caaa`, `caad`, `caam`, `cafc`,
`cafm`, `cafr`, `cain`, `camt`, `canm`, `casp`, `casr`, `catm`, `catp`, `colr`,
`fxtr`, `head`, `pacs`, `pain`, `reda`, `remt`, `secl`, `seev`, `semt`, `sese`,
`setr`, `tsin`, `tsmt`, `tsrv`, `trck`). This is the current iso20022.org
catalogue (latest versions) plus the earlier versions, so older in-circulation
messages still parse.

The generated types derive [`yaserde`](https://crates.io/crates/yaserde) for XML
and (with the `serde` feature) `serde::{Serialize, Deserialize}` for JSON.

## Capabilities

| Capability | API |
|---|---|
| Typed model, all message versions | `generated::<area>::<msg>::Document` |
| XML parse / serialize | `from_xml` / `to_xml` |
| JSON parse / serialize (ISO element names) | `from_json` / `to_json` (feature `serde`) |
| Per-message identity (namespace, MxId, area, functionality, variant, version) | the `MxMessage` trait on every `Document` |
| Auto-detect & parse | `detect`, `parse_as::<T>()`, `generated::any::parse_auto` → `AnyMessage` |
| Business Application Header — read & build | `app_hdr::parse_business_header` / `BusinessHeader::to_app_hdr_xml` |
| Business metadata extraction | `metadata::extract` |
| Business message (header + typed document) | `Envelope<D>` / `parse_envelope`, `read_business_message` |
| Generic tree — read any message without the model | `MxNode::parse` |
| Typed scalars (amount → `Decimal`, dates → `chrono`) | `convert::{to_decimal, to_date, to_datetime}` (feature `convert`) |
| Message catalogue | `catalogue` |
| WebAssembly / JS bindings | `src/wasm.rs` (build via `scripts/build-wasm.sh`) |

## Quick start

Identify a message and read its metadata — no `model` feature needed:

```rust
use rust_iso20022::{detect, BusinessArea, MxNode};

let xml = r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08">
  <FIToFICstmrCdtTrf><GrpHdr><MsgId>ABC-1</MsgId></GrpHdr>
    <CdtTrfTxInf><IntrBkSttlmAmt Ccy="EUR">1234.56</IntrBkSttlmAmt></CdtTrfTxInf>
  </FIToFICstmrCdtTrf></Document>"#;

let id = detect(xml).unwrap();
assert_eq!(id.message_name(), "pacs.008.001.08");
assert_eq!(id.business_area, BusinessArea::pacs);

// Read fields from the generic tree without the typed model:
let doc = MxNode::parse(xml).unwrap();
assert_eq!(doc.find("MsgId").and_then(|n| n.text()), Some("ABC-1"));
let amt = doc.find("IntrBkSttlmAmt").unwrap();
assert_eq!((amt.text(), amt.attr("Ccy")), (Some("1234.56"), Some("EUR")));
```

Typed parse / serialize (enable the area's model, e.g.
`cargo add rust_iso20022 -F model-pacs`):

```rust,ignore
use rust_iso20022::generated::pacs::pacs_008_001_08::Document;

let doc: Document = rust_iso20022::from_xml(&xml)?;
let back: String = rust_iso20022::to_xml(&doc)?;
```

See the runnable examples:

```bash
cargo run --example inspect_message                                   # no features
cargo run --example typed_payment --features model-pacs,serde,convert # typed
```

## Features

| Feature | Default | Effect |
|---------|---------|--------|
| `model-<area>` | no | the typed model for one business area, e.g. `model-pacs`. Enable only what you need — a single area compiles in seconds vs many minutes for all |
| `model` | no | all `model-<area>` at once (~1130 modules; slow to compile) |
| `serde` | no | `serde` + JSON (`to_json`/`from_json`) for the core, catalogue and message types |
| `convert` | no | typed scalar conversions (`to_decimal`/`to_date`/`to_datetime`) via `rust_decimal`/`chrono` |
| `cli` | no | the `iso20022` command-line catalogue lookup tool |
| `catalogue` | no | runtime XSD fetcher (`fetch` module): pulls in `tokio`, `reqwest`, `regex` |

The core (`MxId`, `BusinessArea`, `MxNode`, `detect`) and the `catalogue` are
always available. The XSD → Rust generator is a separate, unpublished workspace
crate (`tools/codegen`), so the published crate has **no git dependencies**.

## Command-line tool

```bash
cargo run --features cli --bin iso20022 -- pacs.008   # all pacs.008 versions
# or after `cargo install rust_iso20022 --features cli`:
iso20022 camt          # every message in the camt business area
iso20022               # the whole catalogue
```

## Regenerating the model

The model and catalogue are produced from the XSD schemas in `xsds/` by the
codegen tool:

```bash
cargo run -p rust_iso20022_codegen -- --input xsds --output src/generated
```

To (re)download schemas, the `catalogue` feature provides a `Fetcher`. The
authoritative source is iso20022.org; its static schema path
(`/sites/default/files/documents/messages/<area>/schemas/<name>.xsd`) is the
reliable programmatic download path (`Fetcher::download_schema`).

## Design notes

- **Scalars are `String`** — lossless (exact text, no float rounding, ideal for
  money) and keeps XML and JSON in sync. Convert on demand with the `convert`
  feature.
- **Choices** are modelled as a struct of `Option<…>` fields (the same shape
  JAXB uses), so an amount inside a `<xsd:choice>` round-trips with its `Ccy`
  attribute and unset choices are simply omitted.
- **JSON** uses the ISO 20022 element names (`MsgId`, `IBAN`, …); `to_json` /
  `from_json` round-trip.
- A value that is not a known member of a coded enumeration parses to an
  `__Unknown__(String)` fallback (surfaces only for invalid input).
- XML round-tripping preserves the data model, not byte-for-byte formatting.

## License

Apache-2.0.
