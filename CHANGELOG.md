# Changelog

All notable changes to this project are documented here.

## [0.1.0] - 2026-06-25

Initial release.

### Added
- **prowide `AbstractMX` parity:** the `MxMessage` trait gives every generated
  `Document` its identity (`MESSAGE_NAME`, `NAMESPACE`, `BUSINESS_AREA`,
  `FUNCTIONALITY`, `VARIANT`, `VERSION`, `mx_id()`), plus `detect(xml)`,
  `parse_as::<T>()` and a generated `AnyMessage` enum with `parse_auto(xml)`
  for namespace-based auto-detection.
- **Business Application Header:** `app_hdr::parse_business_header` reads BAH
  (`head.001`) fields (from/to/BizMsgIdr/MsgDefIdr/CreDt), version-independent,
  without the model.
- **JSON:** `to_json` / `from_json` and `serde` derives on every generated
  message type (feature `serde`), mirroring prowide's `toJson`/`fromJson`. JSON
  uses the ISO 20022 element names (`MsgId`, `IBAN`, …), not Rust field names.
- **Metadata:** `metadata::extract` reads business metadata (message id, amount,
  currency, value date, transaction count) from a message, version-independently.
- **Business-message reader:** `read_business_message` returns the header, the
  detected `MxId` and the metadata of a full AppHdr+Document message in one call.
- **AppHdr building:** `BusinessHeader::to_app_hdr_xml` writes a `head.001`
  `<AppHdr>` element (complements the reader).
- **WASM/JS bindings** (`src/wasm.rs`, built with `--cfg direct_wasm` via
  `scripts/build-wasm.sh`): exposes the identification, catalogue and metadata
  layer to JavaScript/npm. The crate is now a `cdylib`+`rlib`.
- **Full model coverage (502/502):** the generator now disambiguates
  complexTypes with multiple anonymous `<xsd:choice>` groups, so the previously
  skipped `reda`/`seev` messages are generated.
- **Coverage expanded to 722 messages / 32 business areas:** added the securities
  settlement/management and trade-services families (`semt`, `sese`, `setr`,
  `tsin`, `tsmt`, `tsrv`, `trck`), with schemas sourced directly from
  iso20022.org's static schema path. `Fetcher::download_schema` uses that path.
- `cli` feature: an `iso20022` command-line tool for querying the message
  catalogue by message name, business area or namespace (uses `prettytable-rs`).
- `serde` feature now covers `MxId` (serializes to its canonical
  `"pacs.008.001.08"` name, deserializes via `MxId::parse`) as well as
  `BusinessArea` and `CatalogueEntry`.
- `LICENSE` (Apache-2.0), `.gitignore`, and `scripts/publish.sh`.

### Fixed
- Real ISO 20022 messages now parse: the generated types previously rejected
  every message with "bad namespace" because `xsd-parser` emitted unprefixed
  `#[yaserde(namespace)]`; codegen now emits the working
  `prefix`/`default_namespace` form.
- Monetary amounts no longer lose their value: `simpleContent` complexTypes
  (e.g. `…Amount`) had their text value dropped by `xsd-parser`; codegen now
  injects the value field.
- `to_xml` no longer leaks `<__Unknown__>` placeholder elements for unset
  choices (round-trip preserved).
- **Publishable to crates.io:** removed all git dependencies. `xsd-types` was
  dropped (scalars are `String`), `xsd-macro-utils` was replaced by the local
  `simple_type!` macro, and the codegen tool (which uses the git `xsd-parser`)
  moved to a separate, unpublished workspace crate (`tools/codegen`).
- Hand-written core: `MxId` (message identification from namespace / name),
  `BusinessArea` (all 37 ISO 20022 business areas), `Error`, and the
  `from_xml` / `to_xml` / `to_xml_fragment` (de)serialization helpers.
- `from_namespace` convenience parser and a `validate` module replacing the
  `xsd-parser` `Validate` trait at runtime.
- Generated message model (`generated` module): 495 message versions across
  25 business areas, each deriving `yaserde` `YaSerialize` / `YaDeserialize`.
- Generated message `catalogue` (`phf` tables): 502 message ids with namespace,
  business area and a `has_model` flag.
- `codegen` binary (`codegen` feature): XSD → yaserde Rust generator, with
  identical-item de-duplication and a skip-list for unsupported multi-choice
  schemas.
- `catalogue` feature: runtime `Fetcher` for downloading XSD schemas from a
  configurable source.
- `serde` feature: optional `serde` derives on core / catalogue types.
- Round-trip tests over real ISO 20022 sample messages.

### Known limitations
- Scalar values are exposed as `String` (lossless); see README.
- The `Ccy` attribute on amounts nested inside an `<xsd:choice>` is not
  re-serialized on write (yaserde flatten+enum limitation); reading is
  unaffected.
